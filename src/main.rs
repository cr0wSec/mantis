pub mod controllers;
pub mod config;
pub mod models;

#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use crate::config::SERVER_CONFIG;
use crate::controllers::ui::views::{get_index, get_login, get_register};

#[launch]
fn rocket() -> Rocket<Build> {

    let figment = rocket::Config::figment()
        .merge(("secret_key", SERVER_CONFIG.secret_key.clone()));

    rocket::custom(figment)
        .mount("/", routes![get_index, get_login, get_register])
        .attach(Template::fairing())
}