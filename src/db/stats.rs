use crate::db::models::{DailyReadingStats, ReadingSession};
use rusqlite::{params, Connection, Result};

pub fn record_reading_session(conn: &Connection, book_id: &str, duration_seconds: u32) -> Result<()> {
    let now = chrono::Utc::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let session_id = uuid::Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO reading_sessions (id, book_id, duration_seconds, timestamp, date_str)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![session_id, book_id, duration_seconds, now.timestamp(), date_str],
    )?;
    Ok(())
}

pub fn get_7_day_stats(conn: &Connection) -> Result<Vec<DailyReadingStats>> {
    let now = chrono::Utc::now();
    let mut stats = Vec::new();

    for i in (0..7).rev() {
        let day = now - chrono::Duration::days(i);
        let date_str = day.format("%Y-%m-%d").to_string();
        let day_name = match day.format("%a").to_string().as_str() {
            "Mon" => "Sen",
            "Tue" => "Sel",
            "Wed" => "Rab",
            "Thu" => "Kam",
            "Fri" => "Jum",
            "Sat" => "Sab",
            _ => "Min",
        };

        let mut stmt = conn.prepare(
            "SELECT COALESCE(SUM(duration_seconds), 0) FROM reading_sessions WHERE date_str = ?1",
        )?;
        let total_seconds: u32 = stmt.query_row(params![date_str], |row| row.get(0))?;
        let minutes = (total_seconds + 30) / 60;

        stats.push(DailyReadingStats {
            date_str,
            minutes,
            day_label: day_name.to_string(),
        });
    }

    Ok(stats)
}

pub fn get_all_reading_sessions(conn: &Connection) -> Result<Vec<ReadingSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, book_id, duration_seconds, timestamp, date_str FROM reading_sessions ORDER BY timestamp DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(ReadingSession {
            id: row.get(0)?,
            book_id: row.get(1)?,
            duration_seconds: row.get(2)?,
            timestamp: row.get(3)?,
            date_str: row.get(4)?,
        })
    })?;

    let mut sessions = Vec::new();
    for s in rows {
        sessions.push(s?);
    }
    Ok(sessions)
}
