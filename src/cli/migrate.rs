use std::env;
use clap::{Parser, Subcommand};
use sqlx::mysql::MySqlPool;
use std::error::Error;
use dotenv::dotenv;

#[derive(Parser)]
#[command(name = "migrate")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Up,
    Down
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Up => {
            let pool = MySqlPool::connect(&database_url).await?;
            sqlx::migrate!("./migrations").run(&pool).await?;
            println!("Migrations applied successfully!");
        }
        Commands::Down => {
            let pool = MySqlPool::connect(&database_url).await?;
            sqlx::migrate!("./migrations").undo(&pool, 1).await?;
            println!("Last migration reverted successfully!");
        }
    }

    Ok(())
}
