pub struct Body {
    data: String
}

pub trait Buffer {
    fn to_buffer(&self) -> Vec<u8>;
}

impl Body {
    pub fn new(data: String) -> Self {
        Body { data }
    }

    pub fn from_str(data: &str) -> Self {
        Body { data: data.to_string() }
    }
}

impl Buffer for Body {
    fn to_buffer(&self) -> Vec<u8> {
        self.data.as_bytes().to_vec()
    }
}


