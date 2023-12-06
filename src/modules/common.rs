use scraper::{Html, Selector};
use std::error::Error;

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
