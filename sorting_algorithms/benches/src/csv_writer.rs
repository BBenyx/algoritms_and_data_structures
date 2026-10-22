use csv::Writer;
use std::error::Error;

pub fn write_to_csv<T: AsRef<[u8]>>(file_name: String, data: Vec<Vec<T>>) -> Result<(), Box<dyn Error>> {
    let file_name = file_name + ".csv";
    let mut wtr = Writer::from_path(&file_name)?;

    for record in data {
        match wtr.write_record(record) {
            Err(err) => panic!("Failed to write file {file_name}! {err}"),
            _ => (),
        }
    }

    wtr.flush()?;
    Ok(())
}
