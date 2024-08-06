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

    // Write header row TODO: get header from html with function
    // let header_row = vec!["Item", "Year 1", "Year 2", "Year 3", "Year 4"];
    // writer.write_record(&header_row)?;

    // Write data rows
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
    // Convert the data variable to a vector of DataItem structs
    let items: Vec<DataItem> = data
        .into_iter()
        .map(|(name, values)| DataItem { name, values })
        .collect();

    // Open a file for writing
    let mut file = File::create(filename)?;

    // Serialize the items vector to JSON and write it to the file
    let json = serde_json::to_string(&items)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
