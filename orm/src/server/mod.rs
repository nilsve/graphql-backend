use std::ops::Deref;
use std::sync::Mutex;
use async_graphql::Schema;
use async_trait::async_trait;
use warp::Filter;

/*#[async_trait]
pub trait GraphqlServer {
    async fn start_warp_server<Query: Send, Mutation: Send, Subscription: Send>(&self, schema: Mutex<Schema<Query,Mutation, Subscription>>) {
        let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

        warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
    }
}
*/
