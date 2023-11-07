use super::headers::{Headers, self};



pub struct Response<T> {
    pub status: u16,
    pub headers: Headers,
    pub body: Option<T>,

}

impl<T> Response<T> {
    pub fn not_found(body: T, headers: Headers) -> Self {
        let status = 404;
        let body= Some(body);

        Response {status , headers, body }
    }
}





