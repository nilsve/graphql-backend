pub mod service;

use weaviate_community::WeaviateClient;

pub fn get_weaviate_client() -> Result<WeaviateClient, Box<dyn std::error::Error>> {
    // With anonymous access
    let client = WeaviateClient::builder("http://localhost:8080").build()?;

    Ok(client)
}
