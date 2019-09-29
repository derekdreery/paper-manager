#![feature(decl_macro, proc_macro_hygiene)]

use rocket_contrib::templates::Template;

mod routes;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", rocket::routes![routes::index])
        .launch();
}
