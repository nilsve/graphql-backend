use actix_web::ResponseError;

use crate::prelude::DynamoRepositoryError;

// #[async_trait]
// trait GraphqlServer {
//     async fn start_warp_server() {
//         println!("Starting server...");
//
//         let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
//
//         warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
//     }
// }
impl ResponseError for DynamoRepositoryError {}
