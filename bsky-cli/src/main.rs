use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use secstr::SecStr;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Post a new message to BlueSky
    Post {
        #[arg(short, long)]
        message: String,

        #[arg(short, long)]
        visibility: Option<String>,
    },
    /// Read posts from BlueSky
    Read {
        #[arg(short, long, default_value = "20")]
        limit: u32,

        #[arg(short, long)]
        time_range: Option<String>,
    },
    /// List users from a file
    ListUsers {
        #[arg(short, long, default_value = "10")]
        limit: u32,

        #[arg(short, long)]
        filter: Option<String>,
    },
}

struct BlueSkyClient {
    handle: SecStr,
    app_password: SecStr,
    base_url: String,
}

impl BlueSkyClient {
    pub fn new(handle: String, app_password: String) -> Result<Self> {
        Ok(Self {
            handle: SecStr::from(handle),
            app_password: SecStr::from(app_password),
            base_url: "https://bsky.social/xrpc".to_string(),
        })
    }

    pub async fn post_message(&self, _message: &str, _visibility: Option<String>) -> Result<()> {
        // Implement BlueSky post logic
        Ok(())
    }

    pub async fn read_posts(&self, _limit: u32, _time_range: Option<String>) -> Result<()> {
        // Implement BlueSky read posts logic
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    let handle = env::var("BLUESKY_HANDLE").context("BLUESKY_HANDLE not set")?;
    let app_password = env::var("BLUESKY_APP_PASSWORD").context("BLUESKY_APP_PASSWORD not set")?;

    let client = BlueSkyClient::new(handle, app_password)?;

    match &cli.command {
        Commands::Post { message, visibility } => {
            client.post_message(message, visibility.clone()).await?;
        }
        Commands::Read { limit, time_range } => {
            client.read_posts(*limit, time_range.clone()).await?;
        }
        Commands::ListUsers { limit, filter } => {
            let users = read_users_from_file("users.txt")?;
            let filtered_users = filter_users(users?, filter.clone(), *limit as usize);
            for user in filtered_users {
                println!("{}", user);
            }
        }
    }

    Ok(())
}

fn read_users_from_file(file_path: &str) -> Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path).context("Could not open users file")?;
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| line.context("Could not read line from file"))
        .collect()
}

fn filter_users(users: Vec<String>, filter: Option<String>, limit: usize) -> Vec<String> {
    let filtered_users: Vec<String> = match filter {
        Some(f) => users.into_iter().filter(|user| user.contains(&f)).collect(),
        None => users,
    };

    filtered_users.into_iter().take(limit).collect()
}
