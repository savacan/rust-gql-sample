use anyhow::Result;
use serde::Serialize;
use sqlx::{Acquire, Executor, FromRow, MySql, MySqlConnection};

#[derive(Default, Serialize, FromRow, Clone, PartialEq, Debug)]
pub struct Note {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: i8,
}

impl Note {
    pub async fn find_by_id<'a, E>(conn: E, id: i64) -> Result<Note>
    where
        E: Executor<'a, Database = MySql>,
    {
        let note: Vec<Note> = sqlx::query_as!(
            Note,
            r#"SELECT id, user_id, title, content, category, published FROM notes WHERE id=?"#,
            id
        )
        .fetch_all(conn)
        .await?;

        let note = note
            .first()
            .ok_or_else(|| anyhow::anyhow!("invalid note_id: {}", id))?;

        Ok(note.clone())
    }

    pub async fn find_by_ids<'a, E>(conn: E, ids: &[i64]) -> Result<Vec<Note>>
    where
        E: Executor<'a, Database = MySql>,
    {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let ids = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let query = format!(
            r#"SELECT id, user_id, title, content, category, published FROM notes WHERE id IN ({})"#,
            ids
        );

        let notes: Vec<Note> = sqlx::query_as(&query).fetch_all(conn).await?;
        Ok(notes)
    }

    pub async fn find_all<'a, E>(conn: E) -> Result<Vec<Note>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let notes: Vec<Note> = sqlx::query_as!(
            Note,
            r#"SELECT id, user_id, title, content, category, published FROM notes"#
        )
        .fetch_all(conn)
        .await?;

        Ok(notes)
    }

    // acquireを使ってErrorになるやつを再現したい
    pub async fn find_by_ids_acq<'a, A>(conn: A, note_ids: &[i64]) -> Result<Vec<Note>>
    where
        A: Acquire<'a, Database = MySql>,
    {
        let mut conn = conn.acquire().await?;
        let mut odd_ids = vec![];
        let mut even_ids = vec![];
        for &i in note_ids {
            if i % 2 == 0 {
                even_ids.push(i);
            } else {
                odd_ids.push(i);
            }
        }
        let notes_odd = Note::find_by_ids_acq_odd(&mut *conn, &odd_ids).await?;
        let notes_even = Note::find_by_ids_acq_even(&mut *conn, &even_ids).await?;
        Ok(notes_even
            .into_iter()
            .chain(notes_odd.into_iter())
            .collect())
    }

    pub async fn find_by_ids_acq_odd<'a, A>(conn: A, note_ids: &[i64]) -> Result<Vec<Note>>
    where
        A: Acquire<'a, Database = MySql>,
    {
        let mut conn = conn.acquire().await?;
        let notes = Note::find_by_ids(&mut *conn, note_ids).await?;
        Ok(notes)
    }

    pub async fn find_by_ids_acq_even<'a, A>(conn: A, note_ids: &[i64]) -> Result<Vec<Note>>
    where
        A: Acquire<'a, Database = MySql>,
    {
        let mut conn = conn.acquire().await?;
        let notes = Note::find_by_ids(&mut *conn, note_ids).await?;
        Ok(notes)
    }

    // acquireを使ってErrorになるやつを再現したい
    pub async fn find_by_ids_con(
        conn: &mut MySqlConnection,
        note_ids: &[i64],
    ) -> Result<Vec<Note>> {
        let conn = conn.acquire().await?;
        let mut odd_ids = vec![];
        let mut even_ids = vec![];
        for &i in note_ids {
            if i % 2 == 0 {
                even_ids.push(i);
            } else {
                odd_ids.push(i);
            }
        }
        let notes_odd = Note::find_by_ids_con_odd(&mut *conn, &odd_ids).await?;
        let notes_even = Note::find_by_ids_con_even(&mut *conn, &even_ids).await?;
        Ok(notes_even
            .into_iter()
            .chain(notes_odd.into_iter())
            .collect())
    }

    pub async fn find_by_ids_con_odd(
        conn: &mut MySqlConnection,
        note_ids: &[i64],
    ) -> Result<Vec<Note>> {
        let conn = conn.acquire().await?;
        let notes = Note::find_by_ids(&mut *conn, note_ids).await?;
        Ok(notes)
    }

    pub async fn find_by_ids_con_even(
        conn: &mut MySqlConnection,
        note_ids: &[i64],
    ) -> Result<Vec<Note>> {
        let conn = conn.acquire().await?;
        let notes = Note::find_by_ids(&mut *conn, note_ids).await?;
        Ok(notes)
    }
}
