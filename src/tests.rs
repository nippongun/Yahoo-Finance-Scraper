use crate::modules::{balancesheet::parse_balance_sheet, common::parse_table_header};

#[test]
fn test_parse_table_header() {
    let html = r#"
    <div class="D(tbhg)">Header 1</div>
    <div class="D(tbhg)">Header 2</div>
    <div class="D(tbhg)">Header 3</div>
"#;

    let expected = vec!["Header 1", "Header 2", "Header 3"];

    let result = parse_table_header(html).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_parse_balance_sheet() {
    let html = r#"
            <div class="D(itb)">
                <div class="D(tbr)">
                    <div class="D(tbc)">Asset 1</div>
                    <div class="D(tbc)">100</div>
                    <div class="D(tbc)">200</div>
                    <div class="D(tbc)">300</div>
                    <div class="D(tbc)">350</div>
                </div>
                <div class="D(tbr)">
                    <div class="D(tbc)">Asset 2</div>
                    <div class="D(tbc)">400</div>
                    <div class="D(tbc)">500</div>
                    <div class="D(tbc)">600</div>
                    <div class="D(tbc)">650</div>
                </div>
                <div class="D(tbr)">
                    <div class="D(tbc)">Liability 1</div>
                    <div class="D(tbc)">700</div>
                    <div class="D(tbc)">800</div>
                    <div class="D(tbc)">900</div>
                    <div class="D(tbc)">950</div>
                </div>
            </div>
        "#;

    let expected = vec![
        (
            "Asset 1".to_string(),
            vec![
                "100".to_string(),
                "200".to_string(),
                "300".to_string(),
                "350".to_string(),
            ],
        ),
        (
            "Asset 2".to_string(),
            vec![
                "400".to_string(),
                "500".to_string(),
                "600".to_string(),
                "650".to_string(),
            ],
        ),
        (
            "Liability 1".to_string(),
            vec![
                "700".to_string(),
                "800".to_string(),
                "900".to_string(),
                "950".to_string(),
            ],
        ),
    ];

    assert_eq!(parse_balance_sheet(html).unwrap(), expected);
}
