pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn mutation_test(&self, ctx: &async_graphql::Context<'_>, val1: String) -> i16 {
        println!("mutation_test was called! {:?}", chrono::Utc::now());
        0
    }
}
