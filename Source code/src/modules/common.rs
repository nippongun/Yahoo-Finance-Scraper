use scraper::{Html, Selector};
use std::error::Error;

#[allow(dead_code)]
pub fn parse_table_header(html: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let header_selector = Selector::parse("div.D\\(tbhg\\)").unwrap();
    let headers = document.select(&header_selector);
    let mut result = Vec::new();

    for element in headers {
        result.push(element.inner_html());
    }
    Ok(result)
}
