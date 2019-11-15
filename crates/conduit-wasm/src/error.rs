use crate::types::ErrorInfo;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    // 401
    #[fail(display = "Unauthorized")]
    Unauthorized,

    // 403
    #[fail(display = "Forbidden")]
    Forbidden,

    // 404
    #[fail(display = "Not Found")]
    NotFound,

    // 422
    #[fail(display = "Unprocessable Entity: {:?}", _0)]
    UnprocessableEntity(ErrorInfo),

    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    // serde deserialize error
    #[fail(display = "Deserialize Error")]
    DeserializeError,

    #[fail(display = "Http Request Error")]
    RequestError,
}
