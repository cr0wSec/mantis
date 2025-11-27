use rocket_dyn_templates::Template;

#[get("/")]
pub fn get_index() -> Template {
    Template::render("index", ())
}
#[get("/login")]
pub fn get_login() -> Template {
    Template::render("login", ())
}

#[get("/register")]
pub fn get_register() -> Template {
    Template::render("register", ())
}