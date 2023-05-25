use anyhow::Result;
use serde::Serialize;
use sqlx::{Executor, FromRow, MySql};

#[derive(Default, Serialize, FromRow, Clone, PartialEq, Debug)]
pub struct Building {
    pub id: i64,
    pub user_id: i64,
    pub address: String,
    pub building_code: i64,
    pub rent: i64,
    pub minimum_rent: Option<i64>,
}

impl Building {
    pub async fn find_by_id<'a, E>(conn: E, id: i64) -> Result<Building>
    where
        E: Executor<'a, Database = MySql>,
    {
        let building: Vec<Building> = sqlx::query_as!(
            Building,
            r#"SELECT id, user_id, address, building_code, rent, minimum_rent FROM buildings WHERE id=?"#,
            id
        )
        .fetch_all(conn)
        .await?;

        let building = building
            .first()
            .ok_or_else(|| anyhow::anyhow!("invalid note_id: {}", id))?;

        Ok(building.clone())
    }

    pub async fn find_by_ids<'a, E>(conn: E, ids: &[i64]) -> Result<Vec<Building>>
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
            r#"SELECT id, user_id, address, building_code, rent, minimum_rent FROM buildings id IN ({})"#,
            ids
        );

        let buildings: Vec<Building> = sqlx::query_as(&query).fetch_all(conn).await?;
        Ok(buildings)
    }

    pub async fn find_all<'a, E>(conn: E) -> Result<Vec<Building>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let buildings: Vec<Building> = sqlx::query_as!(
            Building,
            r#"SELECT id, user_id, address, building_code, rent, minimum_rent FROM buildings"#
        )
        .fetch_all(conn)
        .await?;

        Ok(buildings)
    }

    pub async fn find_by_user_id<'a, E>(conn: E, user_id: i64) -> Result<Vec<Building>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let buildings: Vec<Building> = sqlx::query_as!(
            Building,
            r#"SELECT id, user_id, address, building_code, rent, minimum_rent FROM buildings WHERE user_id=?"#,
            user_id
        )
        .fetch_all(conn)
        .await?;

        Ok(buildings)
    }

    pub async fn find_by_user_ids<'a, E>(conn: E, ids: &[i64]) -> Result<Vec<Building>>
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
            r#"SELECT id, user_id, address, building_code, rent, minimum_rent FROM buildings user_id IN ({})"#,
            ids
        );

        let buildings: Vec<Building> = sqlx::query_as(&query).fetch_all(conn).await?;
        Ok(buildings)
    }
}
