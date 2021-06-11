// voir 17.3 du livre: utilisation de state pattern
// base post gre pour stocker le vok
// dockerisation

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::content::Html;
use rocket_contrib::databases::diesel;
use rocket_contrib::templates::Template;

#[database("my_pg_db")]
struct VoklerDbConn(diesel::PgConnection);
mod arbeit;
mod session;

#[get("/")]
fn home() -> Html<&'static str> {
    Html(
        r#"<a href="arbeit/submit">Submit a question</a> or <a href="arbeit/test">Carry out a test</a>."#,
    )
}

fn main() {
    rocket::ignite()
        .mount("/", routes![home])
        .mount("/arbeit", arbeit::routes())
        .attach(Template::fairing())
        .attach(VoklerDbConn::fairing())
        .launch();
}
