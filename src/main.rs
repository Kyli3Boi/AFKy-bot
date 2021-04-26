#![allow(non_snake_case)]
#![allow(unused_imports)]

use dotenv;
use mongodb;
use mongodb::bson::doc;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, guild::*, id::*},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "?ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
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
	let token = dotenv::var("DCTOKEN")
		.expect("Expected a token in the environment");
	
	let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}

	// MongoDB
	let clientURI = dotenv::var("MONGODB_URI")
		.expect("Expected a token in the environment");

	let mut options = mongodb::options::ClientOptions::parse(&clientURI).await.expect("WELP OPTIONS ARE BAD");
	options.app_name = Some("AFKy Bot".to_string());
	let mdbClient = mongodb::Client::with_options(options).expect("WELP CLIENT IS BAD");

	let db = mdbClient.database("newthing");
	let coll = db.collection("yes");
	let users = vec![
		doc! {"userID":"123143", "timestamp":"today"},
		doc! {"userID":"343535", "timestamp":"yesterday"}
	];
	coll.insert_many(users, None).await.expect("Err, sum bad when add");

}