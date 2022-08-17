use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use fehler::throws;
use anyhow::Error;

#[throws(_)]
pub async fn read_cfg(filepath: String) -> HashMap<String,HashMap<String,String>> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut result: HashMap<String,HashMap<String, String>> = HashMap::new();
    let mut key: String = "".to_string();
    let mut temp_val = HashMap::new();
    let mut temp_adding = false;
    let mut counter = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let first_character: String = line.chars().skip(0).take(1).collect();
        if !first_character.eq("#") && !first_character.eq("") {
            if first_character.eq("[") {
                if temp_adding {
                    result.insert(key.clone(), temp_val.clone());
                } else {
                    temp_adding = true;
                }
                key = line.trim().to_string().replace("[", "").replace("]", "");
                temp_val.clear();
                counter = 0;
            } else {
                let s: Vec<&str> = line.split("=").collect();
                let k = if s.len() > 1 { s[0].trim().to_string() } else { counter.to_string() };
                let v = if s.len() > 1 { s[1] } else { s[0] }.trim().to_string();
                counter = counter + 1;
                temp_val.insert(k, v);
            }
        }
    }
    result
}

