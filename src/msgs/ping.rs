
use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn ping(ctx: Context, msg: Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}
