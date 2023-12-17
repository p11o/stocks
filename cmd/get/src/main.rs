use anyhow::{Result, Context};
use chrono::{Duration, Utc};
use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde_json::Value;
use std::{env, fs, thread, time::Duration as StdDuration};

const MAX_RETRIES: u32 = 10;
const INITIAL_RETRY_DELAY: u64 = 2; // seconds

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Ticker symbol
    #[arg(long)]
    ticker: String,

    // Multiplier of timespan for candle
    #[arg(long, default_value_t = 1)]
    multiplier: u64,

    // Timespan for candle
    #[arg(long, default_value = "minute")]
    timespan: String,

    // From date
    #[arg(long)]
    from: Option<String>,
    
    // To date
    #[arg(long)]
    to: Option<String>,

    // Adjusted price based on splits
    #[arg(long)]
    adjusted: Option<String>,

    // Sort
    #[arg(long)]
    sort: Option<String>,

    // Limit for number of records
    #[arg(long, default_value = "50000")]
    limit: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let from_date_default = default_from_date();
    let to_date_default = default_to_date();
    let from_date = args.from.as_deref().unwrap_or(&from_date_default);
    let to_date = args.to.as_deref().unwrap_or(&to_date_default);

    let polygon_api_key = env::var("POLYGON_API_KEY").context("POLYGON_API_KEY required")?;

    let query_string = build_query_string(&[
        ("adjusted", args.adjusted.as_deref().unwrap_or_default()),
        ("sort", args.sort.as_deref().unwrap_or_default()),
        ("limit", &args.limit),
    ]);
    let client = Client::new();
    let mut url = Some(format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/{}/{}/{}?{}",
        args.ticker, args.multiplier, args.timespan, from_date, to_date, query_string
    ));
    let mut page = 1;
    let mut retry_delay = INITIAL_RETRY_DELAY;

    while let Some(ref current_url) = url {
        let filename = format!(
            "{}_{}_{}_page_{}.json",
            args.ticker, from_date, to_date, page
        );
        let res = client
            .get(current_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", polygon_api_key))
            .send()
            .with_context(|| format!("Failed to send request to {}", current_url))?;

        match res.status().as_u16() {
            200 => {
                let body = res.text()?;
                fs::write(&filename, &body)
                    .with_context(|| format!("Failed to write to file {}", filename))?;
                println!("Saved {}.", filename);

                url = parse_next_url(&body);
                page += 1;
                retry_delay = INITIAL_RETRY_DELAY;
            },
            429 => {
                println!("Received 429 Too Many Requests. Retrying in {} seconds...", retry_delay);
                thread::sleep(StdDuration::from_secs(retry_delay));
                retry_delay = std::cmp::min(retry_delay * 2, MAX_RETRIES as u64);
            },
            code => {
                println!("Received HTTP code {}. Exiting", code);
                break;
            }
        }
    }

    Ok(())
}

fn build_query_string(params: &[(&str, &str)]) -> String {
    params.iter()
        .filter(|&(_, value)| !value.is_empty())
        .map(|&(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&")
}

fn default_from_date() -> String {
    Utc::now()
        .checked_sub_signed(Duration::days(5))
        .expect("Invalid date")
        .format("%Y-%m-%d")
        .to_string()
}

fn default_to_date() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

fn parse_next_url(body: &str) -> Option<String> {
    serde_json::from_str::<Value>(body)
        .ok()
        .and_then(|json| json.get("next_url").and_then(|v| v.as_str()).map(String::from))
}
