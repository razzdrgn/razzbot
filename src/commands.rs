use rand::prelude::*;

type Context<'a> = poise::Context<'a, crate::Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

/// Ping, Pong
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say("Pong!").await?;
	Ok(())
}

/// Rolls a dice
#[poise::command(slash_command)]
pub async fn roll(
	ctx: Context<'_>,
	#[description = "Amount of dice to roll"] dice: Option<u32>,
	#[description = "Amount of sides on the dice"] sides: u32,
	#[description = "Modifier to final result"] modifier: Option<i32>,
) -> Result<(), Error> {
	let mut result: Vec<i32> = vec![];
	for _i in 0..dice.unwrap_or(1) {
		result.push(rand::thread_rng().gen_range(1..sides) as i32)
	}
	let mod_result: Vec<i32> = result
		.iter()
		.map(|x| x + modifier.unwrap_or_default())
		.collect();
	let sum = result.iter().sum::<i32>();
	let output: String;
	if let Some(modifier) = modifier {
		output = format!(
			"Dice rolled: {:?}, with modifier {} applied: {:?}, sum is: {}",
			result, modifier, mod_result, sum
		);
	} else {
		output = format!("Dice rolled: {:?}, sum is: {}", result, sum);
	}
	ctx.say(output).await?;
	Ok(())
}
