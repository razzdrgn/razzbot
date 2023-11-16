type Context<'a> = poise::Context<'a, crate::Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

/// Ping, Pong
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	let latency = poise::serenity_prelude::model::timestamp::Timestamp::now().timestamp() - ctx.created_at().timestamp();
	ctx.say(format!("Pong! This message took {} milliseconds!", latency)).await?;
	Ok(())
}