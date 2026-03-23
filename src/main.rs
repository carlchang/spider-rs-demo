use clap::{Parser, ValueEnum};
use log::{error, info, LevelFilter};
use spider_rs_demo::{format_json, format_table, MovieCrawler};
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about = "Movie crawler with login support")]
struct Args {
    #[arg(short, long, default_value = "https://login2.scrape.center/")]
    url: String,

    #[arg(short, long, default_value = "admin")]
    username: String,

    #[arg(short, long, default_value = "admin")]
    password: String,

    #[arg(short, long, value_enum, default_value = "json")]
    output: OutputFormat,
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Json,
    Table,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Json
    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let args = Args::parse();

    info!("Starting movie crawler");
    info!("Target URL: {}", args.url);
    info!("Username: {}", args.username);

    let mut crawler = match MovieCrawler::new(&args.url, &args.username, &args.password) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create crawler: {}", e);
            process::exit(1);
        }
    };

    match crawler.crawl_with_login().await {
        Ok(result) => {
            info!("Successfully crawled {} movies", result.total);

            match args.output {
                OutputFormat::Json => {
                    println!("{}", format_json(&result));
                }
                OutputFormat::Table => {
                    println!("{}", format_table(&result));
                }
            }

            if result.total == 0 {
                error!("No movies found. Please check the login credentials or website structure.");
                process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to crawl movies: {}", e);
            process::exit(1);
        }
    }
}
