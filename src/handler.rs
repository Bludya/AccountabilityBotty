use crate::utils::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::msgs::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let cmd: Vec<&str> = msg.content.split(' ').collect();
        
        println!("MSG: {:?}", msg);
        match cmd[0] {
            PING_COMMAND => ping(&ctx, &msg).await,
            CHALLENGE_COMMAND => register_challenge(&ctx, &msg)
            &_ => return,
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
