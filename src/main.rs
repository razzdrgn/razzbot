use anyhow::{Context, Result}; // Error handling- Shuttle does not work with Eyre yet :C
use poise::{serenity_prelude as serenity}; // Use Poises' serenity exports
use shuttle_runtime::SecretStore;

mod commands; // Where all the Poise commands are implemented

// Needed for Poise, honestly not sure what to stick in here yet
pub struct Data {}

// Shuttle Service struct, each field represents a "runtime" (ar at least that's how I'm thinking of it)
pub struct RazzbotService {
	discord: serenity::Client,
}

// Defines the function that processes all the startup details for each runtime in the service
#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for RazzbotService {
	async fn bind(mut self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
		tokio::select!(
			_ = self.discord.start() => {},
		);
		Ok(())
	}
}

// Event Handler for handling internal discord events
async fn event_handler(
	ctx: &serenity::Context,
	event: &serenity::FullEvent,
	_framework: poise::FrameworkContext<'_, Data, Box<dyn std::error::Error + Send + Sync>>,
	_data: &Data,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	match event {
		serenity::FullEvent::Ready { data_about_bot, .. } => {
			// Print to console when bot successfully connects to Discord instance
			println!("{} has successfully logged in!", data_about_bot.user.name);
		}
		serenity::FullEvent::Message { new_message, .. } => {
			// Automatically publish all messages in announcement channels
			if let serenity::Channel::Guild(message_channel) = new_message.channel(ctx).await? {
				if let serenity::ChannelType::News = message_channel.kind {
					new_message.crosspost(ctx).await?;
				}
			}
		}
		_ => {}
	}

	Ok(())
}

// Main function where everything gets initialized before being passed into the runtime starter
#[shuttle_runtime::main]
async fn razzbot(
	#[shuttle_runtime::Secrets] secrets: SecretStore,
) -> Result<RazzbotService, shuttle_runtime::Error> {
	// Get the token from the SecretStore
	let token = secrets
		.get("DISCORD_TOKEN")
		.context("'DISCORD_TOKEN' not found")?;

	let intents = serenity::GatewayIntents::non_privileged(); // Discord Gateway Intents go in here using or (|) as bitwise combiners

	// Build the Poise Framework to use as the Discord runtime
	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![commands::ping(), commands::roll()], // All implemented commands go here (god I hope there's a shorter way to implement this)
			event_handler: |_ctx, event, _framework, _data| {
				Box::pin(event_handler(_ctx, event, _framework, _data))
			},
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			// I'll be real I'm not sure what this is doing
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(Data {})
			})
		})
		.build();

	let discord_client = serenity::ClientBuilder::new(token, intents)
		.framework(framework)
		.await
		.with_context(|| "Failed to build discord bot")?;

	// Extracts Discord context to allow for other services to push messages
	// Not used yet, need to figure out how to reimplement at some point
	// let _discord_ctx_cache = discord_client.cache.clone();
	// let _discord_ctx_http = discord_client.http.clone();

	// Starts the service runtimes by passing in constructed objects
	Ok(RazzbotService {
		discord: discord_client,
	})
}
