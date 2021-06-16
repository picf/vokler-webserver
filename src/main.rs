// voir 17.3 du livre: utilisation de state pattern
// base post gre pour stocker le vok
// dockerisation

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

mod arbeit;
mod db_conn;
mod session;

#[get("/")]
fn home() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("home", context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![home])
        .mount("/arbeit", arbeit::routes())
        .attach(Template::fairing())
        .attach(db_conn::VoklerDbConn::fairing())
        .launch();
}
