#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

use rocket_okapi::{openapi_get_routes};
use rocket_okapi::rapidoc::{GeneralConfig, HideShowConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use crate::diesel::prelude::*;

mod controllers;
mod database;
mod repositories;
mod schema;
mod services;
mod utils;

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .attach(database::DbConn::fairing())
        .mount(
            "/",
            openapi_get_routes![
                controllers::post::create_post,
                controllers::post::delete_post,
                controllers::post::get_post,
                controllers::post::update_post,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(()) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
