use log::{error, info};
use regex::Regex;
use serenity::model::prelude::Message;
use serenity::Error;

pub fn match_text_in_command(content: &str, command: &str) -> Result<String, String> {
    let re = Regex::new(r"[\w\s\-\?!]+").map_err(|e| e.to_string())?;
    let no_command = content.replace(command, "");

    let mat = match re.captures(no_command.as_str()) {
        Some(m) => m[0].to_string(),
        None => return Err("Match not found".to_string()),
    };

    Ok(mat)
}

pub fn log_error_send_msg(msg: &Message, why: &Error) {
    error!(
        "Failed to send personal message to author: {} with id: {}. Reason: {}",
        msg.author.name, msg.author.id, why
    );
}

pub fn log_new_command_received(cmdName: &str, msg: &Message) {
    info!(
        "New {} request received by user id: {}, channelId: {}, content: {}",
        cmdName, msg.author.name, msg.author.id, msg.content
    );
}
