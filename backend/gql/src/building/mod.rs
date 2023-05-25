use async_graphql::{
    scalar, ComplexObject, Context, Enum, Error, Guard, Object, Result, SimpleObject, Union,
};
use sample_sql::{Building, User, UserRole};
use serde::{Deserialize, Serialize};

use crate::{
    loader::DataLoader,
    user::{GraphQLUser, UserId},
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub(super) struct BuildingId(pub(super) i64);
scalar!(BuildingId);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub(super) struct BuildingByUserId(pub(super) i64);
scalar!(BuildingByUserId);

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub(super) struct GraphQLBuilding {
    #[graphql(guard = "AdminGuard.or(TenantGuard).or(LessorGuard::new(self.user_id))")]
    pub id: i64,

    #[graphql(guard = "AdminGuard.or(TenantGuard).or(LessorGuard::new(self.user_id))")]
    pub address: String,

    #[graphql(guard = "AdminGuard")]
    pub building_code: i64,

    #[graphql(guard = "AdminGuard.or(TenantGuard).or(LessorGuard::new(self.user_id))")]
    pub rent: i64,

    #[graphql(guard = "AdminGuard.or(LessorGuard::new(self.user_id))")]
    pub minimum_rent: Option<i64>,

    #[graphql(skip)]
    pub(super) user_id: UserId,
}

#[ComplexObject]
impl GraphQLBuilding {
    #[graphql(guard = "AdminGuard.or(TenantGuard).or(LessorGuard::new(self.user_id))")]
    async fn user(&self, ctx: &Context<'_>) -> Result<GraphQLUser> {
        let loader = ctx.data::<DataLoader>()?;
        let user = loader
            .load_one(self.user_id)
            .await?
            .ok_or_else(|| Error::from("invalid user"))?;
        Ok(user)
    }
}

impl From<Building> for GraphQLBuilding {
    fn from(val: Building) -> Self {
        let Building {
            id,
            user_id,
            address,
            building_code,
            rent,
            minimum_rent,
        } = val;
        Self {
            id,
            address,
            building_code,
            rent,
            minimum_rent,
            user_id: UserId(user_id),
        }
    }
}

pub(super) struct LessorGuard {
    user_id: UserId,
}
impl LessorGuard {
    fn new(user_id: UserId) -> Self {
        LessorGuard { user_id }
    }
}
#[async_trait::async_trait]
impl Guard for LessorGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // 貸主でなければだめ
        let user = ctx
            .data::<Option<User>>()?
            .as_ref()
            .ok_or_else(|| Error::from("unauthorized"))?;
        if user.role != UserRole::Lessor {
            return Err(Error::from("unauthorized"));
        }

        // 貸主は自分の持っているビルしか見れない
        if user.id != self.user_id.0 {
            return Err(Error::from("no valid building"));
        }
        Ok(())
    }
}

pub(super) struct AdminGuard;
#[async_trait::async_trait]
impl Guard for AdminGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // adminでなければだめ
        let user = ctx
            .data::<Option<User>>()?
            .as_ref()
            .ok_or_else(|| Error::from("unauthorized"))?;
        if user.role != UserRole::Admin {
            return Err(Error::from("unauthorized"));
        }
        Ok(())
    }
}

pub(super) struct TenantGuard;
#[async_trait::async_trait]
impl Guard for TenantGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // adminでなければだめ
        let user = ctx
            .data::<Option<User>>()?
            .as_ref()
            .ok_or_else(|| Error::from("unauthorized"))?;
        if user.role != UserRole::Tenant {
            return Err(Error::from("unauthorized"));
        }
        Ok(())
    }
}
#[derive(SimpleObject)]
pub(super) struct Area {
    sqm: Option<i64>,
    tsubo: Option<i64>,
}

#[derive(SimpleObject)]
pub(super) struct Other {
    note: String,
}
