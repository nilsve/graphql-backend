use async_trait::async_trait;
use warp::Filter;

#[async_trait]
trait GraphqlServer {
    async fn start_warp_server() {
        println!("Starting server...");

        let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

        warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
    }
}
