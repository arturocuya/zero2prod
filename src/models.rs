use diesel::prelude::*;
use diesel_async::{ RunQueryDsl, AsyncConnection, AsyncPgConnection };
use dotenvy::dotenv;
use std::env;

use crate::schema::subscriptions;

#[derive(Queryable)]
#[derive(serde::Serialize)]
pub struct Subscription {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: chrono::DateTime<chrono::Utc>
}

#[derive(Insertable)]
#[diesel(table_name = subscriptions)]
pub struct NewSubscription<'a> {
    pub email: &'a str,
    pub name: &'a str,
}

pub async fn establish_connnection() -> AsyncPgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("env var DATABASE_URL must be set");

    return AsyncPgConnection::establish(&database_url).await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn create_subscription(conn: &mut AsyncPgConnection, email: &str, name: &str) -> Subscription {
    let new_sub = NewSubscription { email, name };

    return diesel::insert_into(subscriptions::table)
        .values(&new_sub)
        .get_result(conn)
        .await
        .expect("Error saving new subscription");
}