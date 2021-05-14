#![allow(non_snake_case)]
#![allow(unused_imports)]
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::path::Path;
use chrono::prelude::*;
use dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, guild::*, id::*},
    prelude::*,
};

struct Afkers {
	snowflake: String,
	timestamp: String,
}

const DATA: &str = "data.json";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	
    async fn message(&self, _ctx: Context, msg: Message) {
		let dud = Afkers{
			snowflake: msg.author.id.to_string(),
			timestamp: msg.timestamp.to_string()
		};

    	println!("Snowflake - {} | Timestamp - {}", dud.snowflake, dud.timestamp);


		// PicleDB
		let mut store = PickleDb::load(
			DATA, 
			PickleDbDumpPolicy::AutoDump, 
			SerializationMethod::Json
		).unwrap();

		let check = store
			.get::<String>(&dud.snowflake)
			.unwrap()
			.parse::<DateTime<Utc>>()
			.unwrap();


		println!("{}", check);
		store.set(&dud.snowflake, &dud.timestamp).unwrap();
    }

	// async fn guild_member_addition(&self, ctx: Context, gid: GuildId, member: Member,){
	// }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

	dotenv::dotenv().ok();

	// PicleDB
	if !(Path::new(DATA).exists()) {
		let _file = PickleDb::new(
			DATA, 
			PickleDbDumpPolicy::AutoDump, 
			SerializationMethod::Json);
	}

	// Serenity
	let dctoken = dotenv::var("DCTOKEN")
		.expect("Expected discord token in the environment");
	
	let mut client = Client::builder(&dctoken)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}

}