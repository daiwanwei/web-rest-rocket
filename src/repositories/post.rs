use std::error::Error as StdError;
use diesel::result::Error as DieselError;
use rocket::serde::{Serialize, Deserialize};
use crate::database::DbConn;
use crate::diesel::prelude::*;
use crate::repositories::error::Error as DaoError;
use crate::schema::posts;
use crate::schema::posts::{body, title};

pub async fn find_by_id(db: &DbConn, post_id: i32) -> Result<Option<Post>, Box<dyn StdError>> {
    db.run(move |c| {
        posts::table
            .filter(posts::id.eq(post_id))
            .first(c)
    }).await
        .map(|p|Some(p))
        .or_else(|e| match e {
        DieselError::NotFound=>Ok(None),
        _=>Err(Box::new(e) as Box<dyn StdError>)
    })
}

pub async fn create(db: &DbConn, new_post: NewPost) -> Result<Post, Box<dyn StdError>> {
    db.run(|c| {
        diesel::insert_into(posts::table)
            .values::<NewPost>(new_post)
            .get_result::<Post>(c)
    }).await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
}

pub async fn save(db: &DbConn, post: Post) -> Result<Post, Box<dyn StdError>> {
    db.run(move |c| {
        diesel::update(posts::table)
            .filter(posts::id.eq(post.id))
            .set((
                     title.eq(post.title),
                     body.eq(post.body),
                     posts::published.eq(post.published)
            )).get_result::<Post>(c)
    }).await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
}

pub async fn delete(db: &DbConn, post_id: i32) -> Result<(), Box<dyn StdError>> {
    match db.run(move |c| {
        diesel::delete(posts::table)
            .filter(posts::id.eq(post_id))
            .execute(c)
    }).await {
        Ok(affected) => {
            if affected > 0 {
                Ok(())
            } else {
                Err(Box::new(DaoError::PostNotFound))
            }
        },
        Err(e)=>Err(Box::new(e))
    }
}

#[derive(Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Clone, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}
