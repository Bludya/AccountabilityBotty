use log::{error, info, warn};
use serenity::model::channel::Message;
use serenity::model::prelude::{ChannelId, UserId};
use serenity::model::Timestamp;
use serenity::prelude::*;

use crate::utils::{
    log_error_send_msg, log_new_command_received, match_text_in_command, ParseErr,
    CHALLENGE_COMMAND, CHALLENGE_REGISTERED_MSG, WRONG_CHALLENGE_NAME_MSG,
};

struct Challenge {
    name: String,
    owner_id: UserId,
    channel_id: ChannelId,
    timestamp: Timestamp,
}

impl Challenge {
    pub fn parse(msg: &Message) -> Result<Challenge, ParseErr> {
        let name = match_text_in_command(&msg.content, CHALLENGE_COMMAND)
            .map_err(|e| ParseErr::ParseCommandTextErr(e))?;

        Ok(Challenge {
            name,
            owner_id: msg.author.id,
            channel_id: msg.channel_id,
            timestamp: msg.timestamp,
        })
    }
}

pub async fn register_challenge(ctx: &Context, msg: &Message) {
    log_new_command_received("challenge", &msg);

    // parse the challenge
    let challenge = match Challenge::parse(&msg) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to parse challenge: {:?}", e);
            if let Err(why) = msg
                .author
                .direct_message(&ctx.http, |m| {
                    m.content(format!("{}", WRONG_CHALLENGE_NAME_MSG))
                })
                .await
            {
                log_error_send_msg(&msg, &why);
            }

            return;
        }
    };

    // TODO: save to db

    // TODO: send messages to tagged buddies

    // return success message
    if let Err(why) = msg
        .author
        .direct_message(&ctx.http, |m| {
            m.content(format!(
                "{} \n Challenge name: {}",
                CHALLENGE_REGISTERED_MSG, challenge.name
            ))
        })
        .await
    {
        log_error_send_msg(&msg, &why);
    }
}
