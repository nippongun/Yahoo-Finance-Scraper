use crate::modules::{parser::parse_table, parser::parse_table_header};
#[test]
fn test_parse_table_header() {
    let html = r#"<div class="row yf-1ezv2n5"><div class="sticky column yf-1ezv2n5">Breakdown</div> <div class="column yf-1ezv2n5 alt">TTM </div><div class="column yf-1ezv2n5">9/30/2023 </div><div class="column yf-1ezv2n5 alt">9/30/2022 </div><div class="column yf-1ezv2n5">9/30/2021 </div><div class="column yf-1ezv2n5 alt">9/30/2020 </div></div>"#;

    let expected = (
        "Breakdown".to_string(),
        vec!["TTM", "9/30/2023", "9/30/2022", "9/30/2021", "9/30/2020"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );

    let result = parse_table_header(html);
    assert_eq!(result.unwrap(), expected);
}
#[test]
fn test_parse_table() {
    let html = r#"
            <div class="tableBody yf-1pgoo1f">
                <div class="row lv-0 yf-1xjz32c">
                    <div class="column sticky yf-1xjz32c">
                        <div class="rowTitle yf-1xjz32c" title="Total Revenue">Total Revenue</div>
                    </div>
                    <div class="column yf-1xjz32c alt">381,623,000 </div>
                    <div class="column yf-1xjz32c">383,285,000 </div>
                    <div class="column yf-1xjz32c alt">394,328,000 </div>
                    <div class="column yf-1xjz32c">365,817,000 </div>
                    <div class="column yf-1xjz32c alt">274,515,000 </div>
                </div>
            </div>
        "#;

    let result = parse_table(html).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].0, "Total Revenue");
    assert_eq!(
        result[0].1,
        vec![
            "381,623,000",
            "383,285,000",
            "394,328,000",
            "365,817,000",
            "274,515,000"
        ]
    );
}
