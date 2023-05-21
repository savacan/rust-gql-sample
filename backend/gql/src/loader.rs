use sample_sql::MySqlPool;

pub(crate) type DataLoader = async_graphql::dataloader::DataLoader<GraphQLLoader>;
pub(super) fn dataloader(pool: MySqlPool) -> DataLoader {
    async_graphql::dataloader::DataLoader::new(GraphQLLoader { pool }, actix_web::rt::spawn)
}

pub(crate) struct GraphQLLoader {
    pool: MySqlPool,
}
