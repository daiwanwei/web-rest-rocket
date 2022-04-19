use std::error::Error;
use rocket::serde::{json::Json};
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use crate::controllers::response::{DataResponse, handle_data_response, handle_data_response_with_option, handle_response, Response};
use crate::database::DbConn;
use crate::services::post::{CreatePostDto, get_post_by_id, PostDto, UpdatePostDto};
use crate::services::post;

#[openapi(tag = "Post")]
#[post("/posts", data = "<create_post_dto>")]
pub async fn create_post(conn: DbConn, create_post_dto: Json<CreatePostDto>) -> Json<DataResponse<PostDto>> {
    let res = post::create_post(&conn, create_post_dto.into_inner()).await;
    Json(handle_data_response(res))
}

#[openapi(tag = "Post")]
#[put("/posts", data = "<update_post_dto>")]
pub async fn update_post(conn: DbConn, update_post_dto: Json<UpdatePostDto>) -> Json<DataResponse<PostDto>> {
    let res = post::update_post(&conn, update_post_dto.into_inner()).await;
    Json(handle_data_response(res))
}

#[openapi(tag = "Post")]
#[delete("/posts/<post_id>")]
pub async fn delete_post(conn:DbConn,post_id: i32) -> Json<Response>{
    let res=post::delete_post(&conn,post_id).await;
    Json(handle_response(res))
}

#[openapi(tag = "Post")]
#[get("/posts/<post_id>")]
pub async fn get_post(conn:DbConn,post_id: i32) -> Json<DataResponse<PostDto>> {
    let res=get_post_by_id(&conn,post_id).await;
    Json(handle_data_response_with_option(res))
}
