use crate::repository::notes::repository::NotesRepository;
use crate::repository::notes::service::NotesService;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use orm::prelude::*;
use std::env;
use async_graphql::{Context, Object};
use std::convert::Infallible;


use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};

mod repository;
mod schema;

struct Query;
#[Object]
impl Query {
    async fn hello(&self, name: String, ctx: &Context<'_>) -> String {
        format!("Hello, {}!", name)
    }
}

#[actix_web::main]
async fn main() -> QueryResult<()> {
    let connection_pool = get_connection_pool();

    let repository = NotesRepository::new(connection_pool);
    let service = NotesService::new(repository);

    let notes = service.find_all()?;
    println!("Displaying {} notes", notes.len());
    for note in notes {
        println!("{}", note.title);
        println!("----------\n");
        println!("{}", note.body);
    }

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(service)
        .finish();

    println!("GraphiQL IDE: http://localhost:8080");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/").finish())
    });

    let routes = graphiql
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
