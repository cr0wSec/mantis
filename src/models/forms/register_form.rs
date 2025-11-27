use rocket::form::FromForm;
use serde::Deserialize;
use validator::Validate;

const MAX_LENGTH: u64 = 64;
const MIN_LENGTH: u64 = 1;
#[derive(Debug, Validate, Deserialize, FromForm)]
pub struct RegisterForm {
    #[validate(length(min = "MIN_LENGTH", max = "MAX_LENGTH"))]
    firstname: String,
    #[validate(length(min = "MIN_LENGTH", max = "MAX_LENGTH"))]
    lastname: String,
    #[validate(email, length(min = 5, max = 70))]
    email: String,
    #[validate(length(min = "MIN_LENGTH", max = "MAX_LENGTH"))]
    password: String,
    #[validate(
        length(min = "MIN_LENGTH", max = "MAX_LENGTH"),
        must_match(other = "password")
    )]
    confirm_password: String,
}


