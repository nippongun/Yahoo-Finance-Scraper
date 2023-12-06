use csv::Writer;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct DataItem {
    name: String,
    values: Vec<String>,
}

pub fn write_csv(filename: &str, data: Vec<(String, Vec<String>)>) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(filename)?;

    for (string_value, vec_value) in data {
        let mut row = vec![string_value];
        row.extend(vec_value);
        writer.write_record(&row)?;
    }

    writer.flush()?;
    Ok(())
}
#[allow(dead_code)]
pub fn write_json(filename: &str, data: Vec<(String, Vec<String>)>) -> Result<(), Box<dyn Error>> {
    let items: Vec<DataItem> = data
        .into_iter()
        .map(|(name, values)| DataItem { name, values })
        .collect();
    let mut file = File::create(filename)?;

    let json = serde_json::to_string(&items)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
