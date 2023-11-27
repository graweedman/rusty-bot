use regex::Regex;
use std::collections::HashMap;

pub struct RequestPath {
    path: String,
    query_params: Option<HashMap<String, String>>,
}

impl RequestPath {
    pub fn new(path: &str) -> Self {
        let mut query = path.split("?").skip(1).next().unwrap_or("").to_string();
        let path = path.split("?").next().unwrap_or("").to_string();
        let query_params = parse_query_params(&query);

        RequestPath { path, query_params }
    }

    pub fn get_query_params(&self) -> Option<&HashMap<String, String>> {
        self.query_params.as_ref()
    }

    pub fn get_route(&self) -> &str {
        if let Some(index) = self.path.find("?") {
            &self.path[..index]
        } else {
            &self.path
        }
    }

    pub fn to_string(&self) -> &str {
        &self.path
    }
}

fn parse_query_params(query_string: &str) -> Option<HashMap<String, String>> {
    if validate_query_string(query_string) {
        let mut query_params: HashMap<String, String> = HashMap::new();

        let query_string_parts: Vec<&str> = query_string.split("&").collect();
        for query_string_part in query_string_parts {
            let query_string_part_parts: Vec<&str> = query_string_part.split("=").collect();
            let key = query_string_part_parts.first().unwrap().to_string();
            let value = query_string_part_parts.last().unwrap().to_string();
            query_params.insert(key, value);
        }
        Some(query_params)
    } else {
        None
    }
}

fn validate_query_string(query_string: &str) -> bool {
    let pattern = Regex::new(r"^[a-zA-Z0-9_]+=[a-zA-Z0-9_]+$").expect("Invalid Regex");

    let key_value_pairs: Vec<&str> = query_string.split("&").collect();
    for key_value_pair in key_value_pairs {
        if !pattern.is_match(key_value_pair) {
            return false;
        }
    }
    true
}
