use scraper::{Html, Selector};
use std::error::Error;

pub fn parse_financials(html: &str) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    //let header_selector = Selector::parse("div.D\\(tbhg\\)").unwrap();
    let table_selector = Selector::parse("div.D\\(itb\\)").unwrap();
    let row_selector = Selector::parse("div.D\\(tbr\\)").unwrap();
    let cell_selector = Selector::parse("div.D\\(tbc\\)").unwrap();
    let mut balance_sheet = Vec::new();

    if let Some(table) = document.select(&table_selector).next() {
        for row in table.select(&row_selector) {
            if let Some((name, value)) = parse_balance_sheet_row(&row, &cell_selector) {
                balance_sheet.push((name, value));
            }
        }
    }

    Ok(balance_sheet)
}

fn parse_financials_row(
    row: &scraper::ElementRef,
    cell_selector: &Selector,
) -> Option<(String, Vec<String>)> {
    let mut cells = row.select(&cell_selector);

    let item_name = cells.next()?.text().collect::<String>().trim().to_string();
    let mut yoy_data: Vec<String> = Vec::with_capacity(4);
    for _n in 0..4 {
        yoy_data.push(cells.next()?.text().collect::<String>().trim().to_string());
    }

    Some((item_name, yoy_data))
}
