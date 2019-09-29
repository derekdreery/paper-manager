use rocket_contrib::templates::{Template, tera::Context};

#[rocket::get("/")]
pub fn index() -> Template {
    let context = Context::new();
    Template::render("index", &context)
}
