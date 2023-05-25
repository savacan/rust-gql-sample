// Re-export from sqlx
pub use sqlx::{types::Decimal, Acquire, MySqlPool};
pub type Transaction<'a> = sqlx::Transaction<'a, sqlx::MySql>;

mod note;
pub use note::Note;

mod user;
pub use user::{User, UserRole};

mod buildings;
pub use buildings::Building;
