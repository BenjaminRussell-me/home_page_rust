#[macro_use] extern crate rocket;
extern crate rocket_contrib;
use rocket::fs::{FileServer, relative};
use std::collections::HashMap;
use rocket_dyn_templates::Template;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", routes![thoughts])
        .mount("/css", FileServer::from(relative!("assets/css")))
}

#[get("/")]
fn index() -> Template {
        let context: HashMap<&str, &str> = [
            ("name", "Benjamin"),
            ("other", "tada"),
            ("page", "test")
        ]
        .iter().cloned().collect();
        Template::render("index", context)
}

#[get("/thoughts")]
fn thoughts() -> Template {
        let context: HashMap<&str, &str> = [
            ("name", "Benjamin"),
            ("other", "tada"),
            ("page", "test")
        ]
        .iter().cloned().collect();
        Template::render("thoughts", context)
}
