use serde::{Deserialize, Serialize};
use std::fs;

use crate::opts::OutputFormat;

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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;

        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
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
        process_csv(test_csv, test_json.to_string(), OutputFormat::Json)?;

        // Read and verify the output JSON
        let json_content = fs::read_to_string(test_json)?;
        let players: Vec<serde_json::Value> = serde_json::from_str(&json_content)?;

        assert_eq!(players.len(), 2);

        // 使用新的方式验证JSON值
        assert_eq!(players[0]["Name"].as_str().unwrap(), "John Doe");
        assert_eq!(players[0]["Position"].as_str().unwrap(), "Forward");
        assert_eq!(players[0]["Kit Number"].as_str().unwrap(), "10");

        assert_eq!(players[1]["Name"].as_str().unwrap(), "Jane Smith");
        assert_eq!(players[1]["Position"].as_str().unwrap(), "Defender");
        assert_eq!(players[1]["Kit Number"].as_str().unwrap(), "5");

        // Clean up test files
        fs::remove_file(test_csv)?;
        fs::remove_file(test_json)?;

        Ok(())
    }
}
