#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::{
	error::Error,
	fmt::format,
	path::Path,
	fs
};
use dotenv;
use pickledb::{
	PickleDb,
	PickleDbDumpPolicy,
	SerializationMethod
};
use chrono::{
	prelude::*,
	Duration
};
use serenity::{
    async_trait,
    model::{
		channel::Message,
		gateway::Ready,
		guild::*,
		user::*,
		id::*
	},
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	
    async fn message(&self, _ctx: Context, msg: Message) {

		if msg.author.bot || msg.is_private() {
			return;
		}
		
		let snowflake = msg.author.id.to_string();
		let timestamp = msg.timestamp.to_string();
		let gid = msg.guild_id.unwrap().to_string();
		let path = format!("./data/{}.json",gid).to_owned();


		// PicleDB
		let mut store = match PickleDb::load(
			&path, 
			PickleDbDumpPolicy::AutoDump, 
			SerializationMethod::Json) {
				Ok(db) => db,
				Err(_e) => create_db(&path),
			};

		let check = match store.get::<String>(&snowflake) {
			Some(s) => { s },
			None => timestamp.to_owned(),
		};

		let check_utc = match check.parse::<DateTime<Utc>>() {
			Ok(d) => d,
			Err(e) => return println!("{:?}",e),
		};

		if msg.timestamp - check_utc > Duration::minutes(1) {
			println!("It's been a minute");
		}

		println!("Snowflake - {} | Timestamp - {} | DBTimestamp - {} | GuildID - {} | Path - {}", snowflake, timestamp, check_utc, gid, path);

		store.set(&snowflake, &timestamp).unwrap();
    }

	async fn guild_member_addition(&self, _ctx: Context, gid: GuildId, member: Member){
		let snowflake = member.user.id.to_string();
		let timestamp = member.joined_at.unwrap().to_string();
		let path = format!("./data/{}.json",gid).to_owned();

		let mut store = create_db(&path);
		store.set(&snowflake, &timestamp).unwrap();
	}

	async fn guild_member_removal(&self, _ctx: Context, gid: GuildId, kicked: User) {
		let snowflake = kicked.id.to_string();
		let path = format!("./data/{}.json",gid).to_owned();
		let mut store = create_db(&path);

		store.rem(&snowflake).unwrap();
	}

	async fn guild_create(&self, ctx: Context, guild: Guild) {
		let gid = guild.id;
		let channel = guild.system_channel_id.unwrap();
		let path = format!("./data/{}.json",gid).to_owned();

		if Path::new(&path).exists() {
			return;
		}

		create_db(&path);

		if let Err(e) = channel.say(&ctx.http, "Hi I'm AFKy! If you gonna be away for a year I will be the first one to know :)").await{
			println!("{:?}", e);
		}
	}

	async fn guild_delete(&self, _ctx: Context, incomplete: GuildUnavailable) {
		let gid = incomplete.id;
		let path = format!("./data/{}.json",gid).to_owned();
		delete_db(&path);
	}

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
		ctx.set_presence(None, OnlineStatus::Idle).await;
    }
}

#[tokio::main]
async fn main() {

	dotenv::dotenv().ok();

	// PicleDB
	if !(Path::new("./data").exists()) {
		fs::create_dir("./data").expect("Can not create \"data\" directory.");
	}

	// Serenity
	let dctoken = dotenv::var("DCTOKEN")
		.expect("Expected Discord token in the environment. Take example from \".exampledotenv\" and fill in \"DCTOKEN\"");
	
	let mut client = Client::builder(&dctoken)
        .event_handler(Handler)
        .await
        .expect("Error while creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}

}

fn create_db(path: &String) -> PickleDb {

	if !(Path::new("./data").exists()) {
		fs::create_dir("./data").expect("Can not create \"data\" directory.");
	}

	let file = PickleDb::new(
		path,
		PickleDbDumpPolicy::AutoDump,
		SerializationMethod::Json);
	return file;
}

fn delete_db(path: &String) {
	if Path::new(path).exists() {
		fs::remove_file(path).unwrap();
	}
}

// TODO fn load_db(path: &string) -> PicleDb {}