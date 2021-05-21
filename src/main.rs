#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::{
	error::Error,
	fmt::format,
	fs,
	path::Path
};
use dotenv;
use pickledb::{
	PickleDb,
	PickleDbDumpPolicy,
	SerializationMethod
};
use chrono::{
	Duration,
	ParseError,
	prelude::*
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
	prelude::*,
	builder::CreateMessage,
	http::AttachmentType,
	client::bridge::gateway::GatewayIntents
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	
	async fn message(&self, ctx: Context, msg: Message) {

		if msg.author.bot || msg.is_private() {
			return;
		}

		let snowflake = msg.author.id.to_string();
		let timestamp = msg.timestamp.to_string();
		let gid = msg.guild_id.unwrap().to_string();
		let path = format!("./data/{}.json",gid).to_owned();


		// PicleDB
		let mut store = load_db(&path);

		let dur = match store.get::<i64>(&"durration") {
			Some(d) => d,
			None => default_durr(),
		};

		let check = match store.get::<String>(&snowflake) {
			Some(s) => s,
			None => timestamp.to_owned(),
		};

		let check_utc = match check.parse::<DateTime<Utc>>() {
			Ok(d) => d,
			Err(e) => date_err(e),
		};

		if msg.timestamp - check_utc > Duration::minutes(dur) {
			if let Err(e) = msg.channel_id.send_message(&ctx.http, |r|{
					r.add_file(AttachmentType::Path(Path::new("./img/oneeternitylater.jpg")))
			}).await {
				println!("{:?}",e);
			};
		}

		store.set(&snowflake, &timestamp).unwrap();
	}

	async fn guild_member_addition(&self, _ctx: Context, gid: GuildId, member: Member){
		let snowflake = member.user.id.to_string();
		let timestamp = Utc::now().to_string();
		let path = format!("./data/{}.json",gid).to_owned();

		let mut store = load_db(&path);
		store.set(&snowflake, &timestamp).unwrap();
	}

	async fn guild_member_removal(&self, _ctx: Context, gid: GuildId, kicked: User) {
		let snowflake = kicked.id.to_string();
		let path = format!("./data/{}.json",gid).to_owned();
		let mut store = load_db(&path);

		store.rem(&snowflake).unwrap();
	}

	async fn guild_create(&self, ctx: Context, guild: Guild) {
		let gid = guild.id;
		let channel = guild.system_channel_id.unwrap();
		let path = format!("./data/{}.json",gid).to_owned();

		// coz when bot starts up it thinks that he joined a new guild
		if Path::new(&path).exists() {
			return;
		}

		let mut store = create_db(&path);

		for (UserId(UID), Member) in guild.members {
			if !(&Member.user.bot){
				store.set(&UID.to_string(), &Utc::now().to_string()).unwrap();
			}
		}

		if let Err(e) = channel.say(&ctx.http, "Hi I'm AFKy! If you gonna be away for a year I will be the first one to know 🕗").await{
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
		.intents(GatewayIntents::all())
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

	let mut file = PickleDb::new(
		path,
		PickleDbDumpPolicy::AutoDump,
		SerializationMethod::Json);

	file.set(&"durration", &default_durr()).unwrap();

	return file;
}

fn delete_db(path: &String) {
	if Path::new(path).exists() {
		fs::remove_file(path).unwrap();
	}
}

fn load_db(path: &String) -> PickleDb {

	let store = match PickleDb::load(
		&path, 
		PickleDbDumpPolicy::AutoDump, 
		SerializationMethod::Json) {
			Ok(db) => db,
			Err(_e) => create_db(&path),
		};
	return store;
}

fn default_durr() -> i64 {
	let year = chrono::Date::year(&Utc::now().date());
	if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
		return 527040;
	}
	return 525600;
}

fn date_err(err: ParseError) -> DateTime<Utc> {
	println!("Error parsing date: {:?}\n Supplying current date.", err);
	let date = Utc::now();
	return date;
}
