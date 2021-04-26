#![allow(non_snake_case)]
#![allow(unused_imports)]

use dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, guild::*, id::*},
    prelude::*,
};

struct Afkers {
	snowflake: u64,
	lastseen: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	
    async fn message(&self, _ctx: Context, msg: Message) {
		let timestamp = msg.timestamp;
		let snowflake = msg.author.id;
		let dud = Afkers{snowflake: *msg.author.id.as_u64(), lastseen: msg.timestamp.to_string()};
    	println!("Snowflake - {} | Timestamp - {}", snowflake, timestamp);
		println!("{} {}", dud.snowflake, dud.lastseen);
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