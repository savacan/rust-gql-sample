use std::collections::HashMap;

use async_graphql::{dataloader::Loader, Result};
use sample_sql::{Building, MySqlPool, Note, User};

use crate::{
    building::{BuildingByUserId, BuildingId, GraphQLBuilding},
    note::{GraphQLNote, NoteId},
    user::{GraphQLUser, UserId},
};

pub(crate) type DataLoader = async_graphql::dataloader::DataLoader<GraphQLLoader>;
pub(super) fn dataloader(pool: MySqlPool) -> DataLoader {
    async_graphql::dataloader::DataLoader::new(GraphQLLoader { pool }, actix_web::rt::spawn)
}

pub(crate) struct GraphQLLoader {
    pool: MySqlPool,
}

#[async_trait::async_trait]
impl Loader<NoteId> for GraphQLLoader {
    type Value = GraphQLNote;
    type Error = async_graphql::Error;
    async fn load(&self, keys: &[NoteId]) -> Result<HashMap<NoteId, Self::Value>> {
        let ids = keys.iter().map(|e| e.0).collect::<Vec<_>>();
        let notes = Note::find_by_ids(&self.pool, &ids).await?;
        let map = notes
            .into_iter()
            .map(|note| (NoteId(note.id), GraphQLNote::from(note)))
            .collect();
        Ok(map)
    }
}

#[async_trait::async_trait]
impl Loader<UserId> for GraphQLLoader {
    type Value = GraphQLUser;
    type Error = async_graphql::Error;
    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, Self::Value>> {
        let ids = keys.iter().map(|e| e.0).collect::<Vec<_>>();
        let notes = User::find_by_ids(&self.pool, &ids).await?;
        let map = notes
            .into_iter()
            .map(|user| (UserId(user.id), GraphQLUser::from(user)))
            .collect();
        Ok(map)
    }
}

#[async_trait::async_trait]
impl Loader<BuildingId> for GraphQLLoader {
    type Value = GraphQLBuilding;
    type Error = async_graphql::Error;
    async fn load(&self, keys: &[BuildingId]) -> Result<HashMap<BuildingId, Self::Value>> {
        let ids = keys.iter().map(|e| e.0).collect::<Vec<_>>();
        let buildings = Building::find_by_ids(&self.pool, &ids).await?;
        let map = buildings
            .into_iter()
            .map(|building| (BuildingId(building.id), GraphQLBuilding::from(building)))
            .collect();
        Ok(map)
    }
}
#[async_trait::async_trait]
impl Loader<BuildingByUserId> for GraphQLLoader {
    type Value = Vec<GraphQLBuilding>;
    type Error = async_graphql::Error;
    async fn load(
        &self,
        keys: &[BuildingByUserId],
    ) -> Result<HashMap<BuildingByUserId, Self::Value>> {
        let ids = keys.iter().map(|e| e.0).collect::<Vec<_>>();
        let buildings = Building::find_by_user_ids(&self.pool, &ids).await?;
        let mut map = HashMap::new();
        for building in buildings {
            map.entry(BuildingByUserId(building.user_id))
                .or_insert_with(Vec::new)
                .push(building.into());
        }
        for key in keys {
            map.entry(*key).or_insert_with(Vec::new);
        }
        Ok(map)
    }
}
