use rocket::form::FromForm;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, FromForm)]
pub struct LoginForm {
    #[validate(email, length(min = 5, max = 70))]
    email: String,
    #[validate(length(min = 1, max = 64))]
    password: String,
}
