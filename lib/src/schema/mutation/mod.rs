use sqlx::{Pool, Postgres};

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn mutation_test(
        &self,
        ctx: &async_graphql::Context<'_>,
        val1: String,
    ) -> async_graphql::Result<i16> {
        println!("mutation_test was called! {:?}", chrono::Utc::now());
        Ok(0)
    }
}
