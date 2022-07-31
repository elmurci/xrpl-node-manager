use std::collections::HashMap;

pub fn get_bool_from_binary(str: &str) -> bool {
    match str {
        "0" => false,
        "1" => true,
        _ => false
    }
}

pub fn get_vec_from_hashmap(item: &HashMap<String,String>) -> Vec<String> {
    let mut result: Vec<String> = vec!();
    for (key, value) in item {
        result.push(value.to_string());
    }
    result
}