use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_sync_db_pools::{database};
use rocket_sync_db_pools::diesel;

#[database("diesel")]
pub struct DbConn(diesel::PgConnection);

impl<'r> OpenApiFromRequest<'r> for DbConn {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}