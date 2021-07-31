use crate::db_conn;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(FromForm)]
struct AnswerTest {
    answer: String,
    test_type: String,
}

#[derive(FromForm)]
struct SubmitInput {
    question: String,
    answer: String,
}

pub enum TestType {
    age,
    score,
}

impl fmt::Display for TestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestType::age => write!(f, "age"),
            TestType::score => write!(f, "score"),
        }
    }
}

impl FromStr for TestType {
    type Err = ();

    fn from_str(input: &str) -> Result<TestType, Self::Err> {
        match input {
            "age" => Ok(TestType::age),
            "score" => Ok(TestType::score),
            _ => Err(()),
        }
    }
}

const DEFAULT_TEST_TYPE: TestType = TestType::score;

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
        cookies.add_private(Cookie::new("test_type", answer_test.test_type.to_string()));
        db_conn::update_question(conn, id.parse::<i32>().unwrap(), true);
        Redirect::to("/arbeit/test")
    } else {
        cookies.add_private(Cookie::new("test_type", answer_test.test_type.to_string()));
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

    let test_type = match cookies.get_private("test_type") {
        None => DEFAULT_TEST_TYPE,
        Some(answer) => answer.value().parse::<TestType>().unwrap(),
    };

    let (ques, ans, id) = db_conn::get_question(conn, &test_type);
    context.insert("question".to_string(), ques.to_string());
    context.insert("test_type".to_string(), test_type.to_string());
    let (checked_age, checked_score) = match test_type {
        TestType::age => ("checked", ""),
        TestType::score => ("", "checked"),
    };
    context.insert("checked_age".to_string(), checked_age.to_string());
    context.insert("checked_score".to_string(), checked_score.to_string());
    cookies.add_private(Cookie::new("test_type", test_type.to_string()));
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
