mod schema;

use async_graphql::{EmptySubscription, http::GraphiQLSource};
use async_graphql_poem::GraphQL;
use poem::{
    EndpointExt, IntoResponse, Route, Server, get, handler, listener::TcpListener, web::Html,
};
pub use schema::{Mutation, Query};
use sqlx::postgres::PgPoolOptions;

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // let conn = sqlx::Connection::connect("").await;

    // println!("CREATING POOL...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        // TODO: do NOT use hardcoded secrets!
        .connect("postgresql://postgres:YJNhvX4pf9VjNu4f@db.zuejypwiozqqmmwvtsza.supabase.co:5432/postgres")
        .await.unwrap();

    // println!("FETCHING...");

    // let (email, username): (String, String) =
    //     sqlx::query_as("SELECT email, username FROM mingl_io.users")
    //         .fetch_one(&pool)
    //         .await
    //         .unwrap();

    // println!("EMAIL: {email}");
    // println!("UNAME: {username}");

    let schema = async_graphql::Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish();

    let route = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

    Server::new(TcpListener::bind("0.0.0.0:8003"))
        .run(route)
        .await?;

    // GraphQL::new(schema);
    Ok(())
    // Ok(())
}
