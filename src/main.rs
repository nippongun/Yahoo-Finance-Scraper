mod modules;

use clap::Parser;
use modules::cli;
use modules::export;
use modules::parser;
use reqwest::{header, Client};
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let value = cli::Value::parse();

    let ticker = value.ticker;
    let file_format = "csv";

    if value.balancesheet.unwrap() {
        let category = String::from("balance-sheet");
        export_data_for_category(&ticker, &category, file_format).await?;
    }
    if value.financials.unwrap() {
        let category = String::from("financials");
        export_data_for_category(&ticker, &category, file_format).await?;
    }
    if value.cashflow.unwrap() {
        let category = String::from("cash-flow");
        export_data_for_category(&ticker, &category, file_format).await?;
    }
    if value.stocksummary.unwrap() {
        let category = String::from("stock-summary");
        export_data_for_stock_summary(
            &build_url(&ticker, ""),
            &build_filename(&ticker, &category, file_format),
        )
        .await?;
    }
    Ok(())
}

async fn export_data_for_category(
    ticker: &str,
    category: &str,
    file_format: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &build_url(&ticker, &category);
    let filename = &build_filename(&ticker, &category, &file_format);
    fetch_and_export_data(url, filename).await?;
    println!("{} exported", filename);
    Ok(())
}

async fn fetch_and_export_data(url: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let response = get_html(&url).await?;
    let header = parser::parse_table_header(&response)?;
    let mut data = parser::parse_table(&response)?;

    data.insert(0, header);
    export::write_csv(filename, data)?;
    Ok(())
}

async fn export_data_for_stock_summary(url: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let response = get_html(&url).await?;
    let data = parser::parse_stock_summary(&response)?;
    export::write_stock_summary_to_csv(filename, data)?;
    println!("{} exported", filename);
    Ok(())
}

fn build_filename(ticker: &str, category: &str, file_format: &str) -> String {
    format!("{}_{}.{}", ticker, category, file_format)
}

fn build_url(ticker: &str, category: &str) -> String {
    format!("https://finance.yahoo.com/quote/{}/{}", ticker, category)
}

async fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()?;

    let head_resp = client.head(url).send().await?;
    if let Some(content_type) = head_resp.headers().get(header::CONTENT_TYPE) {
        if !content_type.to_str()?.contains("text/html") {
            return Err(format!("Unexpected content type: {:?}", content_type).into());
        }
    }

    println!("Fetching HTML from: {}", url);
    let resp = client.get(url).send().await?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("HTTP error status: {}", status).into());
    }

    let body = resp.text().await?;

    println!("Successfully fetched HTML ({} bytes)", body.len());
    Ok(body)
}

#[cfg(test)]
mod tests;
