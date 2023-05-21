pub mod loader;
pub mod note;
pub mod schema;

use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    route, web, App, Error, HttpResponse,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sample_sql::MySqlPool;

#[route("/graphql", method = "POST")]
async fn graphql_route(
    schema: web::Data<schema::Schema>,
    req: GraphQLRequest,
) -> actix_web::Result<GraphQLResponse> {
    Ok(schema.execute(req.into_inner()).await.into())
}

#[route("/graphql", method = "GET")]
async fn playground_route() -> actix_web::Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    );
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

pub trait GraphQLAppExt {
    fn configure_graphql_api(self, pool: MySqlPool) -> Self;
}

impl<T, B> GraphQLAppExt for App<T>
where
    B: MessageBody,
    T: ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<B>,
        Error = Error,
        InitError = (),
    >,
{
    fn configure_graphql_api(self, pool: MySqlPool) -> Self {
        let schema = schema::schema(pool);
        self.app_data(web::Data::new(schema))
            .service(graphql_route)
            .service(playground_route)
    }
}
