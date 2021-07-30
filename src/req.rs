pub const UPDATE: &str = "
        UPDATE users.pierre
        SET score = array_append(score[2:5], $1),
        last = now()
        WHERE id = $2;";

pub const TEST_EXISTANCE: &str = "SELECT * FROM users.pierre WHERE question = $1 OR answer = $2;";

pub const INSERT: &str = "INSERT INTO users.pierre(question, answer, score, last)
        VALUES ($1, $2, '{0, 0, 0, 0, 0}', NOW());";

pub const REQ_BY_AGE: &str = "
            SELECT question, answer, id
            FROM users.pierre
            ORDER BY last asc            
            LIMIT 1";

pub const REQ_BY_SCORE: &str = "
            SELECT question, answer, id,
            CASE
            	WHEN cardinality(array_positions(score, false)) >=3 then cardinality(array_positions(score, false))* 10
            	ELSE (cardinality(array_positions(score, false)) + 2)*random()
            END as func_score
            FROM users.pierre
            ORDER BY func_score desc            
            LIMIT 1";
