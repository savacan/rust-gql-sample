// Re-export from sqlx
pub use sqlx::{types::Decimal, Acquire, MySqlPool};
pub type Transaction<'a> = sqlx::Transaction<'a, sqlx::MySql>;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
