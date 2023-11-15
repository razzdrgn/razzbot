type Context<'a> = poise::Context<'a, crate::Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

/// Ping, Pong
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say("Pong!").await?;
	Ok(())
}