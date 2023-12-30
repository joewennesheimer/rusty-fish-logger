use fish_logger::log_fish;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let databasepath = format!("{}/fish.db", env::current_dir()?.display().to_string());
    let db = Surreal::new::<Ws>("127.0.0.0:8000").await?;

    db.use_ns("namespace").use_db("database").await?;

    println!("\nWelcome to the Rusty Fish Logger!");
    println!("Type 'quit' at any time to exit the program.\n");

    let _log_fish = log_fish(db).await?;
    Ok(())
}