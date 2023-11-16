use anyhow::{Context, Result};
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;
use std::sync::Arc;

mod commands;

// Needed for Poise
pub struct Data {}

pub struct RazzbotService {
	discord: Arc<poise::Framework<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>>,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for RazzbotService {
	async fn bind(
		mut self,
		addr: std::net::SocketAddr,
	) -> Result<(), shuttle_runtime::Error> {
		tokio::select!(
			_ = self.discord.start() => {},
		);
		Ok(())
	}
}

#[shuttle_runtime::main]
async fn razzbot(
	#[shuttle_secrets::Secrets] secrets: SecretStore,
) -> Result<RazzbotService, shuttle_runtime::Error> {
	let token = secrets.get("DISCORD_TOKEN").context("'DISCORD_TOKEN' not found")?;
	let discord_client = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![commands::ping()],
			..Default::default()
		})
		.token(token)
		.intents(serenity::GatewayIntents::non_privileged())
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(Data {})
			})
		})
		.build()
		.await
		.with_context(|| "Failed to build discord bot")?;

	let _discord_context = discord_client.client().cache_and_http.clone();

	Ok(RazzbotService {
		discord: discord_client,
	})
}
