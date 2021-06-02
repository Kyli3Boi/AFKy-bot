#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::{
	error::Error,
	fmt::format,
	fs,
	path::Path,
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
		id::*,
		interactions::{
            ApplicationCommand,
            ApplicationCommandInteractionDataOptionValue,
            ApplicationCommandOptionType,
            Interaction,
            InteractionResponseType,
            InteractionType,
        },
	},
	prelude::*,
	builder::CreateMessage,
	http::AttachmentType,
	client::bridge::gateway::GatewayIntents,
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

	// Command interaction
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if interaction.kind == InteractionType::ApplicationCommand {
            if let Some(data) = interaction.data.as_ref() {
                let content = match data.name.as_str() {
                    "language" => {
						let options = data
							.options
							.get(0)
							.expect("Expected user option")
							.resolved
							.as_ref()
							.expect("Expected a language");

						if let ApplicationCommandInteractionDataOptionValue::String(String) =
							options
						{
							format!("Selected {}", String)
						}
						else {
							"Please select a language".to_string()
						}
						
					},
                    "interval" => {
                        let options = data
                            .options
                            .get(0)
                            .expect("Expected user option")
                            .resolved
                            .as_ref()
                            .expect("Expected a interval");

                        if let ApplicationCommandInteractionDataOptionValue::Integer(Integar) =
                            options
                        {
							// This is to display type (day/week) instead of Integar number, to user (Possibly another way to do this?)
                            let Interval:String;
							let TimeDay:i64 = 86400;
							let TimeWeek:i64 = 604800;
							let TimeMonth:i64 = 2629800;
							
							if i64::eq(&TimeDay, Integar) {
								Interval = "1 Day".to_string()
							}
							else if i64::eq(&TimeWeek, Integar) {
								Interval = "1 Week".to_string()
							}
							else if i64::eq(&TimeMonth, Integar) {
								Interval = "1 Month".to_string()
							}
							else {
								Interval = "1 Year".to_string()
							}
							
							format!("Notification interval set to {}", Interval)
                        
						} else {
                            "Please select a interval".to_string()
                        }
                    },
					"clearalldata" => {
						let options = data
							.options
							.get(0)
							.expect("Expected user option")
							.resolved
							.as_ref()
							.expect("Expected a answer");

						if let ApplicationCommandInteractionDataOptionValue::String(String) =
							options
						{
							if String::eq(String, "Yes") {
								format!("You Selected {}. Kiss your data goodbye", String)
							}
							else {
								"Phew close call, you selected No".to_string()
							}
							
						}
						else {
							"Please select a option".to_string()
						}
					},
					"github" => {
						let options = data
							.options
							.get(0)
							.expect("Expected user option")
							.resolved
							.as_ref()
							.expect("Expected a answer");

						if let ApplicationCommandInteractionDataOptionValue::String(_String) =
							options
						{
							"https://github.com/qawery-just-sad/AFKy-bot".to_string()	
						}
						else {
							":(".to_string()
						}
					},
					"invite" => {
						let options = data
							.options
							.get(0)
							.expect("Expected user option")
							.resolved
							.as_ref()
							.expect("Expected a answer");

						if let ApplicationCommandInteractionDataOptionValue::String(_String) =
							options
						{
							"https://discord.com/invite".to_string() // Placeholder
						}
						else {
							":(".to_string()
						}
					},
                    _ => "not implemented :(".to_string()
                };

                if let Err(why) = interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }

	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
		ctx.set_presence(None, OnlineStatus::Idle).await;

		// Commands
		let commands = ApplicationCommand::create_global_application_commands(&ctx.http, |commands| {
            commands
                 .create_application_command(|command| {
                    command.name("language")
					.description("Sets your preferred language")
					.create_option(|option| {
						option
							.name("language")
							.description("Your preferred language")
							.kind(ApplicationCommandOptionType::String)
							.required(true)
							.add_string_choice("English", "English")
							.add_string_choice("French", "French")
							.add_string_choice("Spanish", "Spanish")
							.add_string_choice("Russian", "Russian")
					})
                })
                .create_application_command(|command| {
                    command.name("interval")
					.description("Sets the notification interval")
					.create_option(|option| {
                        option
                            .name("interval")
                            .description("How long the player has to be AFK, for the notification to be sent")
                            .kind(ApplicationCommandOptionType::Integer)
                            .required(true)
							.add_int_choice("1 Day", 86400)
							.add_int_choice("1 Week", 604800)
							.add_int_choice("1 Month", 2629800)
							.add_int_choice("1 Year", 31557600)
                    })
                })
                .create_application_command(|command| {
                    command.name("clearalldata")
					.description("Clears all data")
					.create_option(|option| {
                            option
                                .name("clearalldata")
                                .description("Are you sure?")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
								.add_string_choice("No", "No")
								.add_string_choice("No", "Noo")
								.add_string_choice("Yes", "Yes")
								.add_string_choice("No", "Nooo")
                    })
				})
				.create_application_command(|command| {
					command.name("github")
					.description("Displays my Github URL")
					.create_option(|option| {
							option
								.name("github")
								.description("Display Github URL")
								.kind(ApplicationCommandOptionType::String)
								.required(true)
								.add_string_choice("Yes", "Yes")
					})
                })
				.create_application_command(|command| {
					command.name("invite")
					.description("Displays my invite URL")
					.create_option(|option| {
							option
								.name("invite")
								.description("Display invite URL")
								.kind(ApplicationCommandOptionType::String)
								.required(true)
								.add_string_choice("Yes", "Yes")
					})
                })
        })
        .await;

		println!("I now have the following global slash commands: {:#?}", commands);
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
	
	let application_id: u64 =
        dotenv::var("APPLICATION_ID")
		.expect("Expected an application id in the environment")
		.parse()
		.expect("application id is not a valid id");

	let mut client = Client::builder(&dctoken)
		.event_handler(Handler)
		.intents(GatewayIntents::all())
		.application_id(application_id)
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