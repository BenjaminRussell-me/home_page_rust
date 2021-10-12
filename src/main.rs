#[macro_use] extern crate rocket;
use std::collections::HashMap;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}

#[get("/")]
fn index() -> Template {
        let context: HashMap<&str, &str> = [("name", "Jonathan")]
        .iter().cloned().collect();
        Template::render("index", context)
}
