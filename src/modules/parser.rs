use scraper::{Html, Selector};
use std::error::Error;

pub fn parse_table(html: &str) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
    let document = Html::parse_fragment(html);
    let table_selector = Selector::parse("div.tableBody").unwrap();
    let row_selector = Selector::parse("div.row.lv-0").unwrap();
    let cell_selector = Selector::parse("div.column").unwrap();
    let mut data = Vec::new();

    if let Some(table) = document.select(&table_selector).next() {
        for row in table.select(&row_selector) {
            if let Some((name, value)) = parse_row(&row, &cell_selector) {
                data.push((name, value));
            }
        }
    }

    Ok(data)
}

fn parse_row(row: &scraper::ElementRef, cell_selector: &Selector) -> Option<(String, Vec<String>)> {
    let cells = row.select(&cell_selector).collect::<Vec<_>>();
    let mut cells_iter = cells.iter();

    let item_name = cells_iter
        .next()?
        .select(&Selector::parse("div.rowTitle").unwrap())
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let mut yoy_data: Vec<String> = Vec::with_capacity(cells.len() - 1);
    for cell in cells_iter {
        yoy_data.push(cell.text().collect::<String>().trim().to_string());
    }

    Some((item_name, yoy_data))
}

pub fn parse_table_header(html: &str) -> Result<(String, Vec<String>), Box<dyn Error>> {
    let document = Html::parse_fragment(html);
    let row_selector = Selector::parse("div.row.yf-1ezv2n5").unwrap();
    let column_selector = Selector::parse("div.column.yf-1ezv2n5").unwrap();

    let row = document
        .select(&row_selector)
        .next()
        .ok_or("Row not found")?;

    let mut elements: Vec<String> = row
        .select(&column_selector)
        .map(|element| element.text().collect::<String>().trim().to_string())
        .collect();

    if elements.is_empty() {
        return Err("No elements found in the row".into());
    }

    let breakdown = elements.remove(0);
    if breakdown != "Breakdown" {
        return Err("First element is not 'Breakdown'".into());
    }

    Ok((breakdown, elements))
}

pub fn parse_stock_summary(html: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let document = Html::parse_fragment(html);
    let li_selector = Selector::parse("li.yf-tx3nkj").unwrap();
    let span_selector = Selector::parse("span").unwrap();

    let mut result = Vec::new();

    for li in document.select(&li_selector) {
        let mut spans = li.select(&span_selector);
        if let (Some(label_span), Some(value_span)) = (spans.next(), spans.next()) {
            let label = label_span.text().collect::<String>().trim().to_string();
            let value = value_span
                .text()
                .collect::<String>()
                .trim()
                .replace('\u{a0}', " ")
                .to_string();

            // Special handling for 'Earnings Date'
            if label == "Earnings Date" {
                let date_range = value.split(" - ").collect::<Vec<_>>();
                if date_range.len() == 2 {
                    result.push((label, format!("{} - {}", date_range[0], date_range[1])));
                } else {
                    result.push((label, value));
                }
            } else {
                result.push((label, value));
            }
        }
    }

    Ok(result)
}
