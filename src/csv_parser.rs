use std::error::Error;

use crate::ui::table_columns::PartyResults;

pub fn parse_file(path: &str) -> Result<Vec<PartyResults>, Box<dyn Error>> {
    let mut candidates = Vec::new();

    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: PartyResults = result?;

        candidates.push(record);
    }

    Ok(candidates)
}
