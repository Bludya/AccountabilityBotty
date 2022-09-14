use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::handler::Handler;

use actix_web::web::Data;
use serenity::prelude::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

// A new type to hold the newly built server and its port
pub struct Application {
    port: u16,
    client: Client,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        // let address = format!(
        //     "{}:{}",
        //     configuration.application.host, configuration.application.port
        // );
        // let listener = TcpListener::bind(&address)?;
        // let port = listener.local_addr().unwrap().port();
        let client = run(
            configuration.application.discord_token,
            connection_pool,
            configuration.application.base_url,
        )
        .await?;

        Ok(Self { port: 10, client })
    }

    pub fn port(&self) -> u16 {
        self.port
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
    // let db_pool = Data::new(db_pool);
    // let base_url = Data::new(ApplicationBaseUrl(base_url));

    // let server = HttpServer::new(move || {
    //     App::new()
    //         // Instead of `Logger::default`
    //         .wrap(TracingLogger::default())
    //         .route("/subscriptions", web::post().to(subscribe))
    //         .app_data(db_pool.clone())
    //         .app_data(base_url.clone())
    // })
    // .listen(listener)?
    // .run();

    let intents = GatewayIntents::GUILD_MESSAGES;

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
