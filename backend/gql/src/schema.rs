use crate::{building::GraphQLBuilding, loader::dataloader, note::GraphQLNote};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Error, Object, Result};
use sample_sql::{Building, MySqlPool, Note, User};
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
    async fn buildings(&self, ctx: &Context<'_>) -> Result<Vec<GraphQLBuilding>> {
        let pool = ctx.data::<MySqlPool>()?;
        let user = ctx
            .data::<Option<User>>()?
            .as_ref()
            .ok_or_else(|| Error::from("unauthorized"))?;
        let buildings = match user.role {
            sample_sql::UserRole::Admin => Building::find_all(pool).await?,
            sample_sql::UserRole::Tenant => Building::find_all(pool).await?,
            sample_sql::UserRole::Lessor => Building::find_by_user_id(pool, user.id).await?,
        };
        let buildings = buildings.into_iter().map(GraphQLBuilding::from).collect();
        Ok(buildings)
    }
}
