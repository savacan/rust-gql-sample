use std::collections::HashMap;

use async_graphql::{dataloader::Loader, Result};
use sample_sql::{MySqlPool, Note};

use crate::note::{GraphQLNote, NoteId};

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
