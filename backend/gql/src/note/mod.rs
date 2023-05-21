use async_graphql::{scalar, ComplexObject, Context, Error, Result, SimpleObject};
use sample_sql::Note;
use serde::{Deserialize, Serialize};

use crate::{
    loader::DataLoader,
    user::{GraphQLUser, UserId},
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub(super) struct NoteId(pub(super) i64);
scalar!(NoteId);

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub(super) struct GraphQLNote {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: bool,

    #[graphql(skip)]
    pub(super) user_id: UserId,
}

#[ComplexObject]
impl GraphQLNote {
    async fn user(&self, ctx: &Context<'_>) -> Result<GraphQLUser> {
        let loader = ctx.data::<DataLoader>()?;
        let user = loader
            .load_one(self.user_id)
            .await?
            .ok_or_else(|| Error::from("invalid user"))?;
        Ok(user)
    }
}

impl From<Note> for GraphQLNote {
    fn from(val: Note) -> Self {
        let Note {
            id,
            user_id,
            title,
            content,
            category,
            published,
        } = val;
        Self {
            id,
            user_id: UserId(user_id),
            title,
            content,
            category,
            published: published == 1,
        }
    }
}
