# Yahoo-Finance-Scraper 📈
No Yahoo Finance API, no problem! With this web scraper you can collect the individual statements of different companies. Freshly printed to a .csv file or .json (for your geeks out there 🤓).

## Features ⚙️
This scraper will be kept simple because simplicitiy is cool. 😎

### Planned Features 📓
- [x] Full JSON support
- [x] Fetch financials and cash flow.
- [x] Extensive command line arguments 
- [ ] Fetch current market price
- [ ] Stock information

## Usage 🛠️

To use the scraper, either compile with 
```
cargo build --release
```
... or wait until I realeased a package.

```
yahoo_finance_scraper [OPTIONS] --ticker <TICKER>
```

Use 
```
yahoo_finance_scraper --help
```
for help
```
Options:
  -t, --ticker <TICKER>  Set the ticker
  -b, --balancesheet     Set to balance sheet
  -f, --financials       Set to retrieve fiancials
  -c, --cashflow         Set to retrieve cash flow
  -h, --help             Print help
  -V, --version          Print version
```

## License and contribution 💵
All rights reserved until further notice. I will make it free as in freedom 🦅🇺🇸
You may contact me on LinkedIn or my e-mail for questions, contribution or feedback.
