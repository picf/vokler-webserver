use crate::arbeit::TestType;
use crate::req;
use rocket_contrib::databases::postgres;
#[database("my_pg_db")]
pub struct VoklerDbConn(postgres::Connection);

pub fn update_question(conn: VoklerDbConn, id: i32, success: bool) {
    conn.execute(req::UPDATE, &[&success, &id]).unwrap();
}

pub fn add_new_question(conn: VoklerDbConn, question: String, answer: String) {
    let is_new = conn
        .query(req::TEST_EXISTANCE, &[&question, &answer])
        .unwrap()
        .is_empty();
    if is_new {
        let _ = conn.execute(req::INSERT, &[&question, &answer]).unwrap();
    }
}

pub fn get_question(conn: VoklerDbConn, test_type: &TestType) -> (String, String, i32) {
    let res = conn.query(req::REQ_BY_SCORE, &[]).unwrap();
    let row = res.get(0);
    return (row.get("question"), row.get("answer"), row.get("id"));
}
