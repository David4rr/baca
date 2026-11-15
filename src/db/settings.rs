use crate::db::models::UserSettings;
use rusqlite::{params, Connection, Result};

pub fn get_settings(conn: &Connection) -> Result<UserSettings> {
    let mut stmt = conn.prepare(
        "SELECT theme, font_family, font_size, line_height, margin_horizontal, gemini_api_key, gemini_model 
         FROM settings WHERE id = 1",
    )?;

    let mut rows = stmt.query([])?;
    if let Some(row) = rows.next()? {
        Ok(UserSettings {
            theme: row.get(0)?,
            font_family: row.get(1)?,
            font_size: row.get(2)?,
            line_height: row.get(3)?,
            margin_horizontal: row.get(4)?,
            gemini_api_key: row.get(5)?,
            gemini_model: row.get(6)?,
        })
    } else {
        let default_settings = UserSettings::default();
        save_settings(conn, &default_settings)?;
        Ok(default_settings)
    }
}

pub fn save_settings(conn: &Connection, s: &UserSettings) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (id, theme, font_family, font_size, line_height, margin_horizontal, gemini_api_key, gemini_model)
         VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(id) DO UPDATE SET
            theme = excluded.theme,
            font_family = excluded.font_family,
            font_size = excluded.font_size,
            line_height = excluded.line_height,
            margin_horizontal = excluded.margin_horizontal,
            gemini_api_key = excluded.gemini_api_key,
            gemini_model = excluded.gemini_model",
        params![
            s.theme,
            s.font_family,
            s.font_size,
            s.line_height,
            s.margin_horizontal,
            s.gemini_api_key,
            s.gemini_model,
        ],
    )?;
    Ok(())
}
