use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Interested in joining (Select upto 2)")]
    interested_in_joining: String,
    #[serde(rename = "How will you be able to contribute to the above selected team(s)?")]
    contribution: String,
}

fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.interested_in_joining.contains("Contents Team")
            && record.contribution.chars().count() > 200
        {
            println!("{}", record.name);
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv("data.csv") {
        eprintln!("Error reading CSV file: {}", err);
    }
}
