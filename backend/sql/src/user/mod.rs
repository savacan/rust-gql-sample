use anyhow::Result;
use serde::Serialize;
use sqlx::{Executor, FromRow, MySql};

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub role: UserRole,
}

#[derive(Default, Serialize, FromRow, Clone, PartialEq, Debug)]
pub struct RawUser {
    pub id: i64,
    pub name: String,
    pub role: i32,
}

impl From<RawUser> for User {
    fn from(value: RawUser) -> Self {
        let RawUser { id, name, role } = value;
        Self {
            id,
            name,
            role: match role {
                0 => UserRole::Admin,
                1 => UserRole::Lessor,
                2 => UserRole::Tenant,
                // TODO: try_intoにする
                _ => UserRole::Tenant,
            },
        }
    }
}

#[derive(Serialize, Clone, PartialEq, Debug)]
pub enum UserRole {
    Admin,
    Tenant,
    Lessor,
}

impl User {
    pub async fn find_by_id<'a, E>(conn: E, id: i64) -> Result<User>
    where
        E: Executor<'a, Database = MySql>,
    {
        let raw_user: Vec<RawUser> = sqlx::query_as!(
            RawUser,
            r#"SELECT id, name, role FROM users WHERE id=?"#,
            id
        )
        .fetch_all(conn)
        .await?;

        let user: User = raw_user
            .first()
            .ok_or_else(|| anyhow::anyhow!("invalid user_id: {}", id))?
            .clone()
            .into();

        Ok(user)
    }

    pub async fn find_by_ids<'a, E>(conn: E, ids: &[i64]) -> Result<Vec<User>>
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
        let query = format!(r#"SELECT id, name, role FROM users WHERE id IN ({})"#, ids);

        let users: Vec<RawUser> = sqlx::query_as(&query).fetch_all(conn).await?;
        Ok(users.into_iter().map(From::from).collect())
    }

    pub async fn find_all<'a, E>(conn: E) -> Result<Vec<User>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let users: Vec<RawUser> = sqlx::query_as!(RawUser, r#"SELECT id, name, role FROM users"#)
            .fetch_all(conn)
            .await?;

        Ok(users.into_iter().map(From::from).collect())
    }
}
