use rocket::request::Form;
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(FromForm)]
struct UserInput {
    question: String,
    answer: String,
}

#[get("/test")]
fn test() -> Html<&'static str> {
    Html(r#"This is a test."#)
}

#[get("/submit")]
fn submit_input() -> Template {
    let mut context = HashMap::new();
    context.insert("uid", "Pierre");
    Template::render("input", &context)
}

#[post("/submit", data = "<user_input>")]
fn submit_validation(user_input: Form<UserInput>) -> Redirect {
    println!(
        "=q=> {} \n=a=> {}",
        user_input.question.to_string(),
        user_input.answer.to_string()
    );
    Redirect::to("/arbeit/submit")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![submit_validation, submit_input, test]
}
