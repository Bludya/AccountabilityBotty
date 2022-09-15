use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::handler::Handler;

use actix_web::web::Data;
use serenity::prelude::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

// A new type to hold the newly built server and its port
pub struct Application {
    client: Client,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let client = run(
            configuration.application.discord_token,
            connection_pool,
            configuration.application.base_url,
        )
        .await?;

        Ok(Self { client })
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(&mut self) -> Result<(), std::io::Error> {
        if let Err(why) = self.client.start().await {
            println!("Client error: {:?}", why);
        }
        Ok(())
    }
}

pub struct ApplicationBaseUrl(pub String);

async fn run(token: String, db_pool: PgPool, base_url: String) -> Result<Client, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(client)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
