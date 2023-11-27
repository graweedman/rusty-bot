#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpStatus {
    // Informational 1xx
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,

    // Successful 2xx
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    // Redirection 3xx
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // Client Error 4xx
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418, // Yes, there's a 418 status code. It's an April Fools' joke.

    // Server Error 5xx
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
}

impl HttpStatus {
    pub fn to_string(&self) -> String {
        match self {
            HttpStatus::Continue => "100 Continue".to_string(),
            HttpStatus::SwitchingProtocols => "101 Switching Protocols".to_string(),
            HttpStatus::Processing => "102 Processing".to_string(),
            HttpStatus::Ok => "200 OK".to_string(),
            HttpStatus::Created => "201 Created".to_string(),
            HttpStatus::Accepted => "202 Accepted".to_string(),
            HttpStatus::NoContent => "204 No Content".to_string(),
            HttpStatus::ResetContent => "205 Reset Content".to_string(),
            HttpStatus::PartialContent => "206 Partial Content".to_string(),
            HttpStatus::MultipleChoices => "300 Multiple Choices".to_string(),
            HttpStatus::MovedPermanently => "301 Moved Permanently".to_string(),
            HttpStatus::Found => "302 Found".to_string(),
            HttpStatus::SeeOther => "303 See Other".to_string(),
            HttpStatus::NotModified => "304 Not Modified".to_string(),
            HttpStatus::UseProxy => "305 Use Proxy".to_string(),
            HttpStatus::TemporaryRedirect => "307 Temporary Redirect".to_string(),
            HttpStatus::PermanentRedirect => "308 Permanent Redirect".to_string(),
            HttpStatus::BadRequest => "400 Bad Request".to_string(),
            HttpStatus::Unauthorized => "401 Unauthorized".to_string(),
            HttpStatus::PaymentRequired => "402 Payment Required".to_string(),
            HttpStatus::Forbidden => "403 Forbidden".to_string(),
            HttpStatus::NotFound => "404 Not Found".to_string(),
            HttpStatus::MethodNotAllowed => "405 Method Not Allowed".to_string(),
            HttpStatus::NotAcceptable => "406 Not Acceptable".to_string(),
            HttpStatus::ProxyAuthenticationRequired => "407 Proxy Authentication Required".to_string(),
            HttpStatus::RequestTimeout => "408 Request Timeout".to_string(),
            HttpStatus::Conflict => "409 Conflict".to_string(),
            HttpStatus::Gone => "410 Gone".to_string(),
            HttpStatus::LengthRequired => "411 Length Required".to_string(),
            HttpStatus::PreconditionFailed => "412 Precondition Failed".to_string(),
            HttpStatus::PayloadTooLarge => "413 Payload Too Large".to_string(),
            HttpStatus::UriTooLong => "414 URI Too Long".to_string(),
            HttpStatus::UnsupportedMediaType => "415 Unsupported Media Type".to_string(),
            HttpStatus::RangeNotSatisfiable => "416 Range Not Satisfiable".to_string(),
            HttpStatus::ExpectationFailed => "417 Expectation Failed".to_string(),
            HttpStatus::ImATeapot => "418 I'm a teapot".to_string(),
            HttpStatus::InternalServerError => "500 Internal Server Error".to_string(),
            HttpStatus::NotImplemented => "501 Not Implemented".to_string(),
            HttpStatus::BadGateway => "502 Bad Gateway".to_string(),
            HttpStatus::ServiceUnavailable => "503 Service Unavailable".to_string(),
            HttpStatus::GatewayTimeout => "504 Gateway Timeout".to_string(),
            HttpStatus::HttpVersionNotSupported => "505 HTTP Version Not Supported".to_string(),
        }
    }
}



