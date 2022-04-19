use std::error::Error as StdError;
use crate::repositories::error::Error as DaoError;
use crate::services::error::{Error as ServiceError, ErrorKind as ServiceErrorKind};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use crate::database::DbConn;
use crate::repositories::post;
use crate::repositories::post::NewPost;
use crate::services::error::Error::Post;

pub async fn create_post(db: &DbConn, create_post_dto: CreatePostDto) -> Result<PostDto, Box<dyn StdError>> {
    let new_post = NewPost {
        title: create_post_dto.title,
        body: create_post_dto.body,
        published: false,
    };
    post::create(db, new_post).await
        .map(|p| PostDto {
            id: p.id,
            title: p.title,
            body: p.body,
            published: p.published,
        })
}

pub async fn delete_post(db: &DbConn, post_id: i32) -> Result<(), Box<dyn StdError>> {
    post::delete(db, post_id).await
        .map_err(|e| {
            if let Some(dao_err) = e.downcast_ref::<DaoError>() {
                match dao_err {
                    DaoError::PostNotFound => Box::new(
                        ServiceError::Post(ServiceErrorKind::PostNotFound)
                    ),
                    _ => e,
                }
            } else {
                e
            }
        })
}

pub async fn get_post_by_id(db: &DbConn, post_id: i32) -> Result<Option<PostDto>, Box<dyn StdError>> {
    post::find_by_id(db, post_id).await
        .map(|p| p.and_then(|x| Some(
            PostDto {
                id: x.id,
                title: x.title,
                body: x.body,
                published: x.published,
            }
        )))
}

pub async fn update_post(db: &DbConn, update_post_dto: UpdatePostDto) -> Result<PostDto, Box<dyn StdError>> {
    let get_res= post::find_by_id(db, update_post_dto.id).await?;
    match get_res{
        Some(mut post)=>{
            post.title=update_post_dto.title;
            post.body=update_post_dto.body;
            let new_post=post::save(db, post).await?;
            Ok(PostDto{
                id: new_post.id,
                title: new_post.title,
                body: new_post.body,
                published: new_post.published,
            })
        },
        None=>Err(Box::new(
            ServiceError::Post(ServiceErrorKind::PostNotFound)
        ))
    }

}

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CreatePostDto {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UpdatePostDto {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct PostDto {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}