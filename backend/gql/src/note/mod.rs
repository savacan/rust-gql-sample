use async_graphql::{scalar, SimpleObject};
use sample_sql::Note;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub(super) struct NoteId(pub(super) i64);
scalar!(NoteId);

#[derive(SimpleObject, Clone, Default)]
pub(super) struct GraphQLNote {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: bool,
}
impl From<Note> for GraphQLNote {
    fn from(val: Note) -> Self {
        let Note {
            id,
            title,
            content,
            category,
            published,
        } = val;
        Self {
            id,
            title,
            content,
            category,
            published: published == 1,
        }
    }
}
