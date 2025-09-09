use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct Query;

#[derive(async_graphql::SimpleObject)]
struct LoginDetails {
    user_id: String,
    username: String,
}

#[async_graphql::Object]
impl Query {
    async fn query_test(&self, ctx: &async_graphql::Context<'_>, val1: String) -> i16 {
        println!("query_test was called! {:?}", chrono::Utc::now());
        0
    }

    // TODO: this is terrible, implement proper auth and NEVER store real password stuff here :)
    async fn login(
        &self,
        ctx: &async_graphql::Context<'_>,
        email: String,
        pwd: String,
    ) -> async_graphql::Result<LoginDetails> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        let (user_id, username): (Uuid, String) = sqlx::query_as(
            "
            SELECT id, username FROM mingl_io.users
            WHERE 
            email = $1 AND password = $2
            ",
        )
        .bind(email)
        .bind(pwd)
        .fetch_one(pool)
        .await?;

        Ok(LoginDetails {
            user_id: user_id.into(),
            username,
        })
    }
}
