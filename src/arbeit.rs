use crate::db_conn;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(FromForm)]
struct AnswerTest {
    answer: String,
}

#[derive(FromForm)]
struct SubmitInput {
    question: String,
    answer: String,
}

#[post("/test_answer", data = "<answer_test>")]
fn test_answer(
    conn: db_conn::VoklerDbConn,
    mut cookies: Cookies,
    answer_test: Form<AnswerTest>,
) -> Redirect {
    let answer: String = match cookies.get_private("answer") {
        None => {
            panic!("No field answer found")
        }
        Some(answer) => answer.value().to_string(),
    };
    let id: String = match cookies.get_private("id") {
        None => {
            panic!("No field id found")
        }
        Some(id) => id.value().to_string(),
    };
    if answer_test.answer == answer {
        db_conn::update_question(conn, id.parse::<i32>().unwrap(), true);
        Redirect::to("/arbeit/test")
    } else {
        db_conn::update_question(conn, id.parse::<i32>().unwrap(), false);
        Redirect::to("/arbeit/test_error")
    }
}

#[get("/test_error")]
fn test_error(mut cookies: Cookies) -> Template {
    let answer: String = match cookies.get_private("answer") {
        None => {
            panic!("No field answer found")
        }
        Some(answer) => answer.value().to_string(),
    };
    let question: String = match cookies.get_private("question") {
        None => {
            panic!("No field question found")
        }
        Some(question) => question.value().to_string(),
    };
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("answer".to_string(), answer);
    context.insert("question".to_string(), question);
    Template::render("test_error", context)
}

#[get("/test")]
fn test(conn: db_conn::VoklerDbConn, mut cookies: Cookies) -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    let (ques, ans, id) = db_conn::get_question(conn);
    context.insert("question".to_string(), ques.to_string());
    cookies.add_private(Cookie::new("question", ques.to_string()));
    cookies.add_private(Cookie::new("answer", ans));
    cookies.add_private(Cookie::new("id", id.to_string()));
    Template::render("test", context)
}

#[get("/submit")]
fn submit_input() -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("uid".to_string(), "Pierre".to_string());
    Template::render("input", &context)
}

#[post("/submit", data = "<user_input>")]
fn submit_validation(conn: db_conn::VoklerDbConn, user_input: Form<SubmitInput>) -> Redirect {
    db_conn::add_new_question(
        conn,
        user_input.question.to_string(),
        user_input.answer.to_string(),
    );
    Redirect::to("/arbeit/submit")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        submit_validation,
        submit_input,
        test,
        test_answer,
        test_error
    ]
}
