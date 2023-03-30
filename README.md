# Yahoo-Finance-Scraper
Since there is no public Yahoo Finance API, the scraper fetches the balance sheet of a provided ticker symbol (e.g. AAPL). The data will be stored in a CSV or JSON file.

## Features
- The only feature currently is to fetch the balance sheet and store it as a CSV
  - Unfortunately, the JSON is not yet avaialable through the command line

### Upcoming features
- Full JSON support
- Fetch finacials and cash flow.
  - Either separate or all with one command
- Extensive command line arguments 
- Fetch current market price
- Stock information

## Usage

To use the scraper, either compile with 
```
cargo build --release
```
... or wait until I realeased a package.

```
Yahoo-Finance-Scraper [OPTIONS] --ticker <TICKER> --filename <FILENAME>
```

Use 
```
Yahoo-Finance-Scraper --help
```
for help
```
Options:
  -t, --ticker <TICKER>      The ticker in question
  -f, --filename <FILENAME>  Filename for the output
  -b, --balancesheet         Sets the balance sheet (default)
  -h, --help                 Print help
  -V, --version              Print version
```

## License and contribution
All rights reserved until further notice. I will make it free as in Freedom🦅🇺🇸
You may contact me on LinkedIn or my e-mail for questions, contribution or feedback.
