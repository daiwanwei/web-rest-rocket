use std::error::Error;

pub trait CustomError:Error{
    fn get_code(&self)->i32{
        500
    }
    fn get_message(&self)->&'static str{
        "unknown message"
    }
}