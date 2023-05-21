use async_graphql::{scalar, Enum, SimpleObject};
use sample_sql::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub(super) struct UserId(pub(super) i64);
scalar!(UserId);

#[derive(SimpleObject, Clone)]
pub(super) struct GraphQLUser {
    pub id: i64,
    pub name: String,
    pub role: GQLUserRole,
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
