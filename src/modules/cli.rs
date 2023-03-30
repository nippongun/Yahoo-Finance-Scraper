use clap::Parser;

/// Retrieves the (collapsed) balance sheet from Yahoo Finance with the provided ticker.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Value {
    /// The ticker in question
    #[clap(short = 't', long)]
    pub ticker: String,
    /// Filename for the output
    #[clap(short = 'f', long)]
    pub filename: String,

    /// Sets the balance sheet (default)
    #[clap(short, long)]
    pub balancesheet: bool,
}
