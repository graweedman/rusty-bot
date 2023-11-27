
pub struct Headers {
    headers: Vec<(String, String)>
}

impl Headers {
    pub fn new(lines: Option<&Vec<String>>) -> Self {
        let mut headers: Vec<(String, String)> = Vec::new();
        if lines.is_none() {
            return Headers { headers };
        }
        for line in lines.unwrap() {
            if line.is_empty() {
                break;
            }
            let mut header_parts = line.splitn(2, ": ");
            if let (Some(key), Some(value)) = (header_parts.next(), header_parts.next()) {
                headers.push((key.to_string(), value.to_string()));
            }
        }
        Headers { headers }
    }

    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.push((name.to_string(), value.to_string()));
    }

    pub fn to_string(&self) -> String {
        let mut headers = String::new();
        for (name, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", name, value));
        }
        headers
    }
}


