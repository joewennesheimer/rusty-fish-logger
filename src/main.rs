use fish_logger::log_fish;
use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to the database...");
    DB.connect::<Ws>("fish-logger-surrealdb-1:8000").await?;

    DB.use_ns("namespace").use_db("database").await?;

    println!("\nWelcome to the Rusty Fish Logger!");
    println!("Type 'quit' at any time to exit the program.\n");

    let _log_fish = log_fish(&DB).await?;
    Ok(())
}
