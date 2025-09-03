mod schema;

use async_graphql::{EmptySubscription, http::GraphiQLSource};
use async_graphql_poem::GraphQL;
use poem::{IntoResponse, Route, Server, get, handler, listener::TcpListener, web::Html};
pub use schema::{Mutation, Query};

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let schema = async_graphql::Schema::build(Query, Mutation, EmptySubscription).finish();

    let route = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

    Server::new(TcpListener::bind("0.0.0.0:8003"))
        .run(route)
        .await?;

    // GraphQL::new(schema);
    Ok(())
    // Ok(())
}
