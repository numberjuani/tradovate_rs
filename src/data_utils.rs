use std::{fs::File, io::Write};



pub fn create_csv_file<T: serde::Serialize>(data: &[T], filename: &str) {
    let filename = format!(
        "{}-{}.csv",
        filename,
        chrono::Local::now().format("%F-%H%M")
    );
    if !data.is_empty() {
        let mut writer = csv::Writer::from_path(filename).unwrap();
        for line in data {
            writer.serialize(line).unwrap();
        }
    }
}

pub fn string_to_csv(data: &str, filename: &str) {
    let filename = format!(
        "{}-{}.csv",
        filename,
        chrono::Local::now().format("%F-%H%M")
    );
    let mut file = File::create(filename).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}