use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::utils::{log_error_send_msg, HELP_MSG};

pub async fn help(ctx: &Context, msg: &Message) {
    if let Err(why) = msg
        .author
        .direct_message(&ctx.http, |m| m.content(HELP_MSG))
        .await
    {
        log_error_send_msg(&msg, &why)
    }
}
