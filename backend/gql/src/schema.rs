use crate::{loader::dataloader, note::GraphQLNote};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Result};
use sample_sql::{MySqlPool, Note, User};
pub(super) type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema(pool: MySqlPool) -> Schema {
    let builder = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(dataloader(pool.clone()))
        .data(pool);
    builder.finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<GraphQLNote>> {
        let pool = ctx.data::<MySqlPool>()?;
        let notes = Note::find_all(pool).await?;
        Ok(notes.into_iter().map(GraphQLNote::from).collect())
    }
}
