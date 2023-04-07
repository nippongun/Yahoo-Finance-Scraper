mod modules;

use clap::Parser;
use modules::cli;
use modules::export;
use modules::parser;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let value = cli::Value::parse();
    let ticker = value.ticker;
    let filename = value.filename;
    let url = format!(
        "https://finance.yahoo.com/quote/{}/financials?q={}",
        ticker, ticker
    );
    let response = get_html(&url).await?;
    let balance_sheet = parser::parse_table(&response)?;

    println!("Balance Sheet for {}: \n{:#?}", ticker, balance_sheet);
    export::write_csv(&filename, balance_sheet).expect("Something went wrong");

    Ok(())
}

async fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let resp = client.get(url).send().await?.text().await?;
    Ok(resp)
}

#[cfg(test)]
mod tests;
