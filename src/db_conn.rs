use rocket_contrib::databases::postgres;

#[database("my_pg_db")]
pub struct VoklerDbConn(postgres::Connection);

pub fn update_question(conn: VoklerDbConn, id: i32, success: bool) {
    conn.execute(
        "
        UPDATE users.pierre
        SET score = array_append(score[2:5], $1),
        last = now()
        WHERE id = $2;",
        &[&success, &id],
    )
    .unwrap();
}

pub fn add_new_question(conn: VoklerDbConn, question: String, answer: String) {
    let is_new = conn
        .query(
            "SELECT * FROM users.pierre WHERE question = $1 OR answer = $2;",
            &[&question, &answer],
        )
        .unwrap()
        .is_empty();
    if is_new {
        let _ = conn
            .execute(
                "INSERT INTO users.pierre(question, answer, score, last)
        VALUES ($1, $2, '{0, 0, 0, 0, 0}', NOW());",
                &[&question, &answer],
            )
            .unwrap();
    }
}

pub fn get_question(conn: VoklerDbConn) -> (String, String, i32) {
    let res = conn
        .query(
            "SELECT question, answer, id FROM users.pierre
            ORDER BY cardinality(array_positions(score, false)) desc, last asc
            LIMIT 1",
            &[],
        )
        .unwrap();
    let row = res.get(0);
    return (row.get("question"), row.get("answer"), row.get("id"));
}
