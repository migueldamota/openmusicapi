use std::fmt;
use std::fmt::{Debug, Display};
use actix_web::{HttpResponse, http::StatusCode, ResponseError};

#[derive(Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub status: StatusCode,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl From<actix_web::Error> for ErrorResponse {
    fn from(err: actix_web::Error) -> Self {
        ErrorResponse {
            status: err.error_response().status(),
            message: err.to_string(),
        }
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {

        struct Response {
            error: ErrorResponse
        }

        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(Response {
                error: ErrorResponse {
                    message: self.message.to_string(),
                    status: self.status_code()
                }
            })
    }
}
