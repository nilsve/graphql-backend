use crate::repository::entity::Entity;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::delete_item::{DeleteItemError, DeleteItemOutput};
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::{PutItemError, PutItemOutput};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::Serialize;
use serde_dynamo::{from_item, to_item};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DynamoRepositoryError {
    #[error("Error putting item")]
    PutItemError(#[from] SdkError<PutItemError>),
    #[error("Error deleting item")]
    DeleteItemError(#[from] SdkError<DeleteItemError>),
    #[error("Error getting item")]
    GetItemError(#[from] SdkError<GetItemError>),
    #[error("Error deserializing item")]
    DeserializationError(#[from] serde_dynamo::Error),
    #[error("Item wasn't found during get operation")]
    ItemNotFoundError,
}

pub trait RepositoryIndex: Send + Serialize {}

#[async_trait::async_trait]
pub trait DynamoRepository<E>: 'static + Sync
where
    E: Entity,
    E::PrimaryKey: Serialize,
    E::IndexFields: Serialize,
{
    fn get_table_name(&self) -> &'static str;
    fn get_client(&self) -> &'_ aws_sdk_dynamodb::Client;

    async fn create(&self, item: E) -> Result<PutItemOutput, DynamoRepositoryError> {
        Ok(self
            .get_client()
            .put_item()
            .table_name(self.get_table_name())
            .set_item(Some(item.serialize_with_indexes()))
            .set_condition_expression(Some("attribute_not_exists(pk)".to_string()))
            .send()
            .await?)
    }

    async fn upsert(&self, item: E) -> Result<PutItemOutput, DynamoRepositoryError> {
        Ok(self
            .get_client()
            .put_item()
            .table_name(self.get_table_name())
            .set_item(Some(item.serialize_with_indexes()))
            .send()
            .await?)
    }

    async fn delete(&self, item: E) -> Result<DeleteItemOutput, DynamoRepositoryError> {
        Ok(self
            .get_client()
            .delete_item()
            .table_name(self.get_table_name())
            .set_key(Some(item.serialize_primary_key()))
            .send()
            .await?)
    }
    async fn find<Index: RepositoryIndex>(
        &self,
        index: Index,
    ) -> Result<Option<E>, DynamoRepositoryError> {
        println!("Finding item");
        Ok(
            match self
                .get_client()
                .get_item()
                .table_name(self.get_table_name())
                .set_key(Some(to_item(index)?))
                .send()
                .await
                .map_err(|err| DynamoRepositoryError::from(err))?
                .item
            {
                Some(item) => Some(from_item::<HashMap<String, AttributeValue>, E>(item)?),
                None => None,
            },
        )
    }

    async fn get<Index: RepositoryIndex>(&self, id: Index) -> Result<E, DynamoRepositoryError> {
        self.find(id)
            .await?
            .ok_or(DynamoRepositoryError::ItemNotFoundError)
    }
    async fn find_all(&self) -> Result<Vec<E>, DynamoRepositoryError> {
        todo!("Implement find_all method")
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::repository::Entity;
//     use crate::service::Model;
//
//     struct UserModel {
//         id: i32,
//         name: String,
//     }
//
//     struct UserEntity {
//         id: i32,
//         name: String,
//     }
//
//     impl Entity for UserEntity {}
//
//     impl Model for UserModel {
//         type Entity = UserEntity;
//         fn to_entity(self) -> Self::Entity {
//             UserEntity {
//                 id: self.id,
//                 name: self.name,
//             }
//         }
//
//         fn from_entity(entity: Self::Entity) -> Self {
//             UserModel {
//                 id: entity.id,
//                 name: entity.name,
//             }
//         }
//     }
//
//     #[test]
//     fn it_maps() {
//         let entity = UserEntity {
//             id: 1,
//             name: "name".to_string(),
//         };
//         let model = UserModel {
//             id: 1,
//             name: "name".to_string(),
//         };
//         let mapped_model = UserModel::from_entity(entity);
//         assert_eq!(mapped_model.id, model.id);
//         assert_eq!(mapped_model.name, model.name);
//     }
// }
