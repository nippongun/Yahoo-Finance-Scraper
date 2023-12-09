use scraper::{Html, Selector};
use std::error::Error;

pub fn parse_table(html: &str) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let table_selector = Selector::parse("div.D\\(itb\\)").unwrap();
    let row_selector = Selector::parse("div.D\\(tbr\\)").unwrap();
    let cell_selector = Selector::parse("div.D\\(tbc\\)").unwrap();
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

pub fn parse_table_header(html: &str) -> Result<(String, Vec<String>), Box<dyn Error>> {
    let document = Html::parse_fragment(html);
    let tbhg_selector = Selector::parse(".D\\(tbhg\\)").unwrap();
    let tbhg = document.select(&tbhg_selector).next().unwrap();

    let tbr_selector = Selector::parse(".D\\(tbr\\)").unwrap();
    let tbr = tbhg.select(&tbr_selector).next().unwrap();

    let text_vec: Vec<String> = tbr
        .text()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    if let Some(first_element) = text_vec.first() {
        Ok((first_element.clone(), text_vec[1..].to_vec()))
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No text found",
        )))
    }
}
