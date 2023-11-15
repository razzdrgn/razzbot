# razzbot

A (hopefully) simple Discord Bot, built in Rust.

## Features

Right now? Just a /ping command that returns "Pong!". However, I'd ideally like to add the following at some point, in order of how quickly I think I can implement it:

- Automatically publishing messages posted in announcement channels
- User Join/Leave events logging/handling
- Edit and Delete logging for moderation
- Role management, and some kind of user verification/captcha system
- Subscribing to Discord News Channel webhooks to get rid of the stupid per-channel limit on subscriptions
- Twitch EventSub Webhook Callback to handle notifications for when I go live on Twitch
- YouTube PubSub Callback to parse atom feed notifications from YouTube for when I upload videos
- Some fun little games or levelling system, maybe

For me, the goal is primarily to replace the bots that I currently use on my discord servers so these features are mainly lifted from those. I'll probably add more at some point, like maybe a YouTube/Spotify/Whatever scraper/listener thing, but I'm just going to churn away at this and see if I can get things to work as I work on them.

## Libraries

I'm using [serenity](https://github.com/serenity-rs/serenity), as it's the only Discord API for rust that's still currently being supported and maintained.
I'm also using [poise](https://github.com/serenity-rs/poise) to try and make the command structure easier to build and follow.

I'm running deployments using [shuttle](https://shuttle.rs) because it's free and it integrates with cargo.