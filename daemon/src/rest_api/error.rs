// Copyright 2019 Bitwise IO, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix::MailboxError;
use actix_web::error::{PayloadError, ResponseError, UrlGenerationError};
use actix_web::HttpResponse;
use std::error::Error;

use std::fmt;

#[derive(Debug)]
pub enum RestApiServerError {
    StartUpError(String),
    StdError(std::io::Error),
}

impl From<std::io::Error> for RestApiServerError {
    fn from(err: std::io::Error) -> RestApiServerError {
        RestApiServerError::StdError(err)
    }
}

impl Error for RestApiServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RestApiServerError::StartUpError(_) => None,
            RestApiServerError::StdError(err) => Some(err),
        }
    }
}

impl fmt::Display for RestApiServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RestApiServerError::StartUpError(e) => write!(f, "Start-up Error: {}", e),
            RestApiServerError::StdError(e) => write!(f, "Std Error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum RestApiResponseError {
    BadRequest(String),
    SawtoothConnectionError(String),
    SawtoothValidatorResponseError(String),
    RequestHandlerError(String),
}

impl Error for RestApiResponseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RestApiResponseError::BadRequest(_) => None,
            RestApiResponseError::SawtoothConnectionError(_) => None,
            RestApiResponseError::SawtoothValidatorResponseError(_) => None,
            RestApiResponseError::RequestHandlerError(_) => None,
        }
    }
}

impl fmt::Display for RestApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RestApiResponseError::BadRequest(ref s) => write!(f, "Bad Request: {}", s),
            RestApiResponseError::SawtoothConnectionError(ref s) => {
                write!(f, "Zmq Connection Error: {}", s)
            }
            RestApiResponseError::SawtoothValidatorResponseError(ref s) => {
                write!(f, "Sawtooth Validator Response Error: {}", s)
            }
            RestApiResponseError::RequestHandlerError(ref s) => {
                write!(f, "Sawtooth Validator Response Error: {}", s)
            }
        }
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for RestApiResponseError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            RestApiResponseError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            RestApiResponseError::SawtoothConnectionError(ref message) => {
                HttpResponse::ServiceUnavailable().json(message)
            }
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        }
    }
}

impl From<PayloadError> for RestApiResponseError {
    fn from(err: PayloadError) -> RestApiResponseError {
        RestApiResponseError::BadRequest(format!(
            "Payload was not well formated. {}",
            err.to_string()
        ))
    }
}

impl From<MailboxError> for RestApiResponseError {
    fn from(err: MailboxError) -> RestApiResponseError {
        RestApiResponseError::RequestHandlerError(format!(
            "Failed to deliver message to request handler. {}",
            err.to_string()
        ))
    }
}

impl From<UrlGenerationError> for RestApiResponseError {
    fn from(err: UrlGenerationError) -> RestApiResponseError {
        RestApiResponseError::RequestHandlerError(format!(
            "Failed generate response URL. {}",
            err.to_string()
        ))
    }
}
