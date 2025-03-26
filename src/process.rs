use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_process_csv() -> anyhow::Result<()> {
        // Create test CSV file
        let test_csv = "test_players.csv";
        let test_json = "test_players.json";

        let csv_content = "Name,Position,DOB,Nationality,Kit Number\n\
            John Doe,Forward,1990-01-01,England,10\n\
            Jane Smith,Defender,1992-03-15,France,5";

        File::create(test_csv)?.write_all(csv_content.as_bytes())?;

        // Run the process_csv function
        process_csv(test_csv, test_json)?;

        // Read and verify the output JSON
        let json_content = fs::read_to_string(test_json)?;
        let players: Vec<Player> = serde_json::from_str(&json_content)?;

        assert_eq!(players.len(), 2);
        assert_eq!(players[0].name, "John Doe");
        assert_eq!(players[0].position, "Forward");
        assert_eq!(players[0].kit, 10);
        assert_eq!(players[1].name, "Jane Smith");
        assert_eq!(players[1].position, "Defender");
        assert_eq!(players[1].kit, 5);

        // Clean up test files
        fs::remove_file(test_csv)?;
        fs::remove_file(test_json)?;

        Ok(())
    }
}
