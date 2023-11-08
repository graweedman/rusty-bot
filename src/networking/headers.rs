
pub struct Headers {
    headers: Vec<(String, String)>
}

impl Headers {
    pub fn new() -> Self {
        Headers { headers: Vec::new() }
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


