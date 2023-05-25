use async_graphql::{scalar, ComplexObject, Context, Enum, Error, Result, SimpleObject};
use sample_sql::User;
use serde::{Deserialize, Serialize};

use crate::{
    building::{BuildingByUserId, GraphQLBuilding},
    loader::DataLoader,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub(super) struct UserId(pub(super) i64);
scalar!(UserId);

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub(super) struct GraphQLUser {
    pub id: i64,
    pub name: String,
    pub role: GQLUserRole,
}

#[ComplexObject]
impl GraphQLUser {
    async fn buildings(&self, ctx: &Context<'_>) -> Result<Vec<GraphQLBuilding>> {
        let loader = ctx.data::<DataLoader>()?;
        let buildings = loader
            .load_one(BuildingByUserId(self.id))
            .await?
            .ok_or_else(|| Error::from("invalid user"))?;
        Ok(buildings)
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "sample_sql::UserRole")]
pub(super) enum GQLUserRole {
    Admin,
    Lessor,
    Tenant,
}

impl From<User> for GraphQLUser {
    fn from(val: User) -> Self {
        let User { id, name, role } = val;
        Self {
            id,
            name,
            role: role.into(),
        }
    }
}
