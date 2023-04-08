use clap::{ArgAction, Parser};
/// Retrieves the (collapsed) balance sheet from Yahoo Finance with the provided ticker.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Value {
    /// Set the ticker
    #[clap(short = 't', long)]
    pub ticker: String,

    /// Set to balance sheet
    #[clap(short = 'b', long, action = ArgAction::SetTrue)]
    pub balancesheet: Option<bool>,

    /// Set to retrieve fiancials
    #[clap(short = 'f', long, action = ArgAction::SetTrue)]
    pub financials: Option<bool>,

    /// Set to retrieve cash flow
    #[clap(short = 'c', long, action = ArgAction::SetTrue)]
    pub cashflow: Option<bool>,
}
