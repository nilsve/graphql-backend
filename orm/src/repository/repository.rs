use std::collections::HashMap;

use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::delete_item::{DeleteItemError, DeleteItemOutput};
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::{PutItemError, PutItemOutput};
use aws_sdk_dynamodb::operation::query::{QueryError, QueryOutput};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::Serialize;
use serde_dynamo::{to_item, Item};
use thiserror::Error;

use crate::repository::entity::Entity;

#[derive(Error, Debug)]
pub enum DynamoRepositoryError {
    #[error("Error putting item")]
    PutItemError(#[from] SdkError<PutItemError>),
    #[error("Error deleting item")]
    DeleteItemError(#[from] SdkError<DeleteItemError>),
    #[error("Error getting item")]
    QueryError(#[from] SdkError<QueryError>),
    #[error("Error querying items")]
    GetItemError(#[from] SdkError<GetItemError>),
    #[error("Error deserializing item")]
    DeserializationError(#[from] serde_dynamo::Error),
    #[error("Item wasn't found during get operation")]
    ItemNotFoundError,
}

#[derive(Debug)]
pub struct QueryResult<E: Entity> {
    pub items: Vec<E>,
    pub last_evaluated_key: Option<LastEvaluatedKey>,
}

impl<E: Entity> TryFrom<QueryOutput> for QueryResult<E> {
    type Error = DynamoRepositoryError;

    fn try_from(query_output: QueryOutput) -> Result<Self, Self::Error> {
        let items = query_output
            .items
            .ok_or(DynamoRepositoryError::ItemNotFoundError)?
            .into_iter()
            .map(|item| E::from_attribute_values(item))
            .collect::<Result<Vec<E>, _>>()?;

        Ok(QueryResult {
            items,
            last_evaluated_key: query_output.last_evaluated_key,
        })
    }
}

#[derive(Debug, Clone)]
pub struct QueryData<Index: RepositoryIndex> {
    index: Index,
    last_evaluated_key: Option<LastEvaluatedKey>,
}

struct ExpressionData {
    key_condition_expression: String,
    expression_attribute_values: HashMap<String, AttributeValue>,
}

pub type LastEvaluatedKey = HashMap<String, AttributeValue>;

impl<Index: RepositoryIndex> QueryData<Index> {
    pub fn new(index: Index, last_evaluated_key: Option<LastEvaluatedKey>) -> Self {
        Self {
            index,
            last_evaluated_key,
        }
    }

    pub fn get_expression_data(&self) -> ExpressionData {
        let serialized: Item = to_item(&self.index).expect("Failed to serialize index");

        let mut key_condition_expression = Vec::new();
        let mut expression_attribute_values = HashMap::new();

        for (key, value) in serialized.iter() {
            key_condition_expression.push(format!("{} = :{}", key, key));
            expression_attribute_values.insert(format!(":{}", key), value.clone().into());
        }

        ExpressionData {
            key_condition_expression: key_condition_expression.join(" AND "),
            expression_attribute_values,
        }
    }
}

pub trait RepositoryIndex: Send + Serialize + Clone {
    fn to_key(&self) -> HashMap<String, AttributeValue> {
        to_item(self).expect("Failed to serialize index")
    }
}

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
                .set_key(Some(index.to_key()))
                .send()
                .await
                .map_err(|err| DynamoRepositoryError::from(err))?
                .item
            {
                Some(item) => Some(E::from_attribute_values(item)?),
                None => None,
            },
        )
    }

    async fn get<Index: RepositoryIndex>(&self, id: Index) -> Result<E, DynamoRepositoryError> {
        self.find(id)
            .await?
            .ok_or(DynamoRepositoryError::ItemNotFoundError)
    }

    async fn query<Index: RepositoryIndex>(
        &self,
        query_data: QueryData<Index>,
    ) -> Result<QueryResult<E>, DynamoRepositoryError> {
        let expression_data = query_data.clone().get_expression_data();
        Ok(self
            .get_client()
            .query()
            .set_exclusive_start_key(query_data.last_evaluated_key.clone())
            .set_expression_attribute_values(Some(expression_data.expression_attribute_values))
            .key_condition_expression(expression_data.key_condition_expression)
            .limit(1)
            .table_name(self.get_table_name())
            .send()
            .await
            .map_err(|err| DynamoRepositoryError::from(err))?
            .try_into()?)
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
