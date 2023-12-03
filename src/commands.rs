use rand::prelude::*;

type Context<'a> = poise::Context<'a, crate::Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

/// Ping, Pong
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say("Pong!").await?;
	Ok(())
}

/// Just roll with it!
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
	let output: String;
	if result.len() > 1 {
		if let Some(modifier) = modifier {
			output = format!(
				"You rolled the following rolls: {:?}\nWith your modifier of {} applied, you got: {:?}\nThe sum is {}, your highest roll is {}, and lowest {}.\nThe average of all your rolls is {}",
				result,
				modifier,
				mod_result,
				mod_result.iter().sum::<i32>(),
				mod_result.iter().max().unwrap(),
				mod_result.iter().min().unwrap(),
				(mod_result.iter().sum::<i32>() as f32) / (mod_result.len() as f32)
			);
		} else {
			output = format!(
				"You rolled the following rolls: {:?}\nThe sum is {}, your highest roll is {}, and lowest {}.\nThe average of all your rolls is {}",
				result,
				result.iter().sum::<i32>(),
				result.iter().max().unwrap(),
				result.iter().min().unwrap(),
				(result.iter().sum::<i32>() as f32) / (result.len() as f32)
			);
		}
	} else if let Some(modifier) = modifier {
		output = format!(
			"You rolled a {}! With your modifier of {}, that's a {}!",
			result[0], modifier, mod_result[0]
		);
	} else {
		output = format!("You rolled a {}!", result[0]);
	}

	ctx.say(output).await?;
	Ok(())
}
