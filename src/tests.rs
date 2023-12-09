use crate::modules::{parser::parse_table, parser::parse_table_header};

#[test]
fn test_parse_table_header() {
    let html = r#"<div class="D(tbhg)"><div class="D(tbr) C($primaryColor)"><div class="D(ib) Fw(b) Ta(start) Px(15px)--mv2 Px(10px) W(247px)--mv2 W(222px) Bxz(bb) Bdendw(1px) Bdstartw(1px) Bdbw(1px) Bdends(s) Bdstarts(s) Bdbs(s) Bdc($seperatorColor) Py(6px) Pos(st) Start(0) Bgc($lv2BgColor)"><span>Breakdown</span><div class="W(3px) Pos(a) Start(100%) T(0) H(100%) Bg($pfColumnFakeShadowGradient) Pe(n) Pend(5px)"></div></div><div class="Ta(c) Py(6px) Bxz(bb) BdB Bdc($seperatorColor) Miw(120px) Miw(100px)--pnclg D(ib) Fw(b) Tt(u) Bgc($lv1BgColor)"><span>ttm</span></div><div class="Ta(c) Py(6px) Bxz(bb) BdB Bdc($seperatorColor) Miw(120px) Miw(100px)--pnclg D(ib) Fw(b)"><span>9/29/2023</span></div><div class="Ta(c) Py(6px) Bxz(bb) BdB Bdc($seperatorColor) Miw(120px) Miw(100px)--pnclg D(ib) Fw(b) Bgc($lv1BgColor)"><span>9/29/2022</span></div><div class="Ta(c) Py(6px) Bxz(bb) BdB Bdc($seperatorColor) Miw(120px) Miw(100px)--pnclg D(ib) Fw(b)"><span>9/29/2021</span></div><div class="Ta(c) Py(6px) Bxz(bb) BdB Bdc($seperatorColor) Miw(120px) Miw(100px)--pnclg D(ib) Fw(b) Bgc($lv1BgColor)"><span>9/29/2020</span></div></div></div>"#;

    let expected = (
        "Breakdown".to_string(),
        vec!["ttm", "9/29/2023", "9/29/2022", "9/29/2021", "9/29/2020"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );

    let result = parse_table_header(html);

    assert_eq!(result.unwrap(), expected);
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

    assert_eq!(parse_table(html).unwrap(), expected);
}
