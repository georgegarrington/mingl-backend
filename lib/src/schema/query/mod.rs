pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn query_test(&self, ctx: &async_graphql::Context<'_>, val1: String) -> i16 {
        println!("query_test was called!");
        0
    }
}
