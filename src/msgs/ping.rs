
use log::{info, error};
use serenity::builder::CreateButton;
use serenity::model::channel::Message;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::user::User;
use serenity::prelude::*;

pub async fn ping(ctx: Context, msg: Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }

    ask_buddies(&ctx, &msg.mentions).await;
}

pub async fn ask_buddies(ctx: &Context, buddies: &[User]) {
    for b in buddies.iter() {
        let msg = b.direct_message(&ctx.http, |m| m.content("fewef").components(|c| {
            c.create_action_row(|row| {
                row.create_button(|button| {
                    button.custom_id("buddy_accept")
                    .label("Yes!")
                    .style(ButtonStyle::Primary)
                })
                .create_button(|button| {
                    button.custom_id("buddy_deny")
                    .label("No!")
                    .style(ButtonStyle::Primary)
                })
            });
            
        if let Err(why) = 
        })).await {
            println!("Error sending message: {:?}", why);
        }
    };
}