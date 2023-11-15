use anyhow::Context;
use poise::serenity_prelude as serenity;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

mod commands;
// Needed for Poise
pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;

#[shuttle_runtime::main]
async fn razzbot(
	#[shuttle_secrets::Secrets] secrets: SecretStore,
) -> ShuttlePoise<Data, Error> {
	let token = secrets.get("DISCORD_TOKEN").context("'DISCORD_TOKEN' not found")?;
	let framework = poise::Framework::builder()
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
		.map_err(shuttle_runtime::CustomError::new)?;

	Ok(framework.into())
}
