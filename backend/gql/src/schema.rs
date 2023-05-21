use crate::loader::dataloader;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Result};
use sample_sql::MySqlPool;
pub(super) type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema(pool: MySqlPool) -> Schema {
    let mut builder = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(dataloader(pool.clone()))
        .data(pool);
    builder.finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn test(&self, ctx: &Context<'_>) -> Result<bool> {
        Ok(true)
    }
}
