use scraper::{Html, Selector};
use std::error::Error;

pub fn parse_table(html: &str) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let table_selector = Selector::parse("div.D\\(itb\\)").unwrap();
    let row_selector = Selector::parse("div.D\\(tbr\\)").unwrap();
    let cell_selector = Selector::parse("div.D\\(tbc\\)").unwrap();
    let mut balance_sheet = Vec::new();

    if let Some(table) = document.select(&table_selector).next() {
        for row in table.select(&row_selector) {
            if let Some((name, value)) = parse_row(&row, &cell_selector) {
                balance_sheet.push((name, value));
            }
        }
    }

    Ok(balance_sheet)
}

fn parse_row(row: &scraper::ElementRef, cell_selector: &Selector) -> Option<(String, Vec<String>)> {
    let cells = row.select(&cell_selector).collect::<Vec<_>>();
    let mut cells_iter = cells.iter();

    let item_name = cells_iter
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();
    let mut yoy_data: Vec<String> = Vec::with_capacity(cells.len() - 1);
    for _ in 0..(cells.len() - 1) {
        yoy_data.push(
            cells_iter
                .next()?
                .text()
                .collect::<String>()
                .trim()
                .to_string(),
        );
    }

    Some((item_name, yoy_data))
}
