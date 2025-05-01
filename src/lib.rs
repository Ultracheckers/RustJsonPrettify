use anyhow::{Context, Result};
use serde_json::{to_string_pretty, Value};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn read_json_from_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;
    
    let mut content = String::new();
    file.read_to_string(&mut content)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    
    Ok(content)
}

pub fn read_json_from_stdin() -> Result<String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .context("Failed to read from stdin")?;
    
    Ok(content)
}

pub fn prettify_json(content: &str) -> Result<String> {
    let json_value: Value = serde_json::from_str(content)
        .context("Failed to parse JSON")?;
    
    let pretty = to_string_pretty(&json_value)
        .context("Failed to prettify JSON")?;
    
    Ok(pretty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prettify_json() {
        let ugly_json = r#"{"name":"John","age":30,"city":"New York"}"#;
        let expected = r#"{
  "name": "John",
  "age": 30,
  "city": "New York"
}"#;
        
        let result = prettify_json(ugly_json).unwrap();
        assert_eq!(result, expected);
    }
}