use std::error::Error as StdError;
use services::error::Error as ServiceError;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use crate::services;
use crate::utils::error::CustomError;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct DataResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub code: i32,
    pub message: String,
}

pub fn handle_data_response<T>(res: Result<T, Box<dyn StdError>>) -> DataResponse<T> {
    match res {
        Ok(data) => DataResponse {
            code: 200,
            message: "ok".to_string(),
            data: Some(data),
        },
        Err(e) => handle_data_response_with_error(e),
    }
}

pub fn handle_data_response_with_option<T>(res: Result<Option<T>, Box<dyn StdError>>) -> DataResponse<T> {
    match res {
        Ok(opt) => DataResponse {
            code: 200,
            message: "ok".to_string(),
            data: opt,
        },
        Err(e) => handle_data_response_with_error(e),
    }
}

pub fn handle_response(res: Result<(), Box<dyn StdError>>) -> Response {
    match res {
        Ok(()) => Response {
            code: 200,
            message: "ok".to_string(),
        },
        Err(e) => handle_response_with_error(e),
    }
}

pub fn handle_data_response_with_error<T>(e: Box<dyn StdError>) -> DataResponse<T> {
    if let Some(service_err)= e.downcast_ref::<ServiceError>(){
        DataResponse{
            code:service_err.get_code(),
            message:service_err.to_string(),
            data:None,
        }
    }else {
        DataResponse{
            code:500,
            message:e.to_string(),
            data:None,
        }
    }
}

pub fn handle_response_with_error(e: Box<dyn StdError>) -> Response {
    if let Some(service_err)= e.downcast_ref::<ServiceError>(){
        Response{
            code:service_err.get_code(),
            message:service_err.to_string(),
        }
    }else {
        Response{
            code:500,
            message:e.to_string(),
        }
    }
}