use aws_sdk_dynamodb::operation::delete_item::DeleteItemOutput;
use aws_sdk_dynamodb::operation::put_item::PutItemOutput;
use serde::Serialize;

use crate::prelude::{DynamoRepositoryError, QueryData, QueryResult};
use crate::repository::entity::Entity;
use crate::repository::repository::{DynamoRepository, RepositoryIndex};

#[async_trait::async_trait]
pub trait CrudService<E, R>
where
    E: Entity,
    E::PrimaryKey: Serialize,
    E::IndexFields: Serialize,
    R: DynamoRepository<E>,
{
    fn get_repository(&self) -> &R;
    async fn create(&self, entity: E) -> Result<PutItemOutput, DynamoRepositoryError> {
        self.get_repository().create(entity).await
    }
    async fn upsert(&self, entity: E) -> Result<PutItemOutput, DynamoRepositoryError> {
        self.get_repository().upsert(entity).await
    }
    async fn delete(&self, entity: E) -> Result<DeleteItemOutput, DynamoRepositoryError> {
        self.get_repository().delete(entity).await
    }
    async fn find<Index: RepositoryIndex>(
        &self,
        index: Index,
    ) -> Result<Option<E>, DynamoRepositoryError> {
        self.get_repository().find(index).await
    }
    async fn get<Index: RepositoryIndex>(&self, index: Index) -> Result<E, DynamoRepositoryError> {
        self.get_repository().get(index).await
    }

    async fn query<Index: RepositoryIndex>(
        &self,
        query_data: QueryData<Index>,
    ) -> Result<QueryResult<E>, DynamoRepositoryError> {
        self.get_repository().query(query_data).await
    }

    async fn query_all<Index: RepositoryIndex>(
        &self,
        index: Index,
    ) -> Result<Vec<E>, DynamoRepositoryError> {
        // Keep querying until result of QueryResult.last_evaluated_key is None
        let mut items = Vec::new();
        let mut last_evaluated_key = None;
        loop {
            let result = self
                .get_repository()
                .query(QueryData::new(index.clone(), last_evaluated_key))
                .await?;

            items.extend(result.items);
            last_evaluated_key = result.last_evaluated_key;
            if last_evaluated_key.is_none() {
                break;
            }
        }

        Ok(items)
    }
}
//
// #[cfg(test)]
// mod service_tests {
//     use crate::prelude::Entity;
//     use crate::repository::DynamoRepository;
//     use crate::service::{CrudService, Model};
//     use diesel::QueryResult;
//
//     struct UserEntity {
//         id: i32,
//         name: String,
//     }
//
//     impl Entity for UserEntity {
//
//     }
//
//     struct UserModel {
//         id: i32,
//         name: String,
//     }
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
//             Self {
//                 id: entity.id,
//                 name: entity.name,
//             }
//         }
//     }
//
//     struct NewUserEntity {
//         name: String,
//     }
//
//     impl Entity for NewUserEntity {}
//
//     struct NewUserModel {
//         name: String,
//     }
//
//     impl Model for NewUserModel {
//         type Entity = NewUserEntity;
//         fn to_entity(self) -> Self::Entity {
//             NewUserEntity { name: self.name }
//         }
//
//         fn from_entity(entity: Self::Entity) -> Self {
//             Self { name: entity.name }
//         }
//     }
//
//     struct UserRepository;
//     impl DynamoRepository<UserEntity, NewUserEntity> for UserRepository {
//         type Connection = ();
//
//         fn get_connection(&self) -> () {
//             todo!()
//         }
//
//         fn create(&self, item: NewUserEntity) -> QueryResult<usize> {
//             todo!()
//         }
//
//         fn update(&self, item: UserEntity) -> QueryResult<usize> {
//             todo!()
//         }
//
//         fn delete(&self, item: UserEntity) -> QueryResult<usize> {
//             todo!()
//         }
//
//         fn find(&self, id: i32) -> QueryResult<Option<UserEntity>> {
//             Ok(Some(UserEntity {
//                 name: "test".to_owned(),
//                 id: 1,
//             }))
//         }
//
//         fn find_all(&self) -> QueryResult<Vec<UserEntity>> {
//             todo!()
//         }
//     }
//
//     struct UserService {
//         repository: UserRepository,
//     }
//
//     impl CrudService<UserModel, NewUserModel, UserRepository> for UserService {
//         fn get_repository(&self) -> &UserRepository {
//             &self.repository
//         }
//     }
//
//     impl UserService {
//         fn new() -> Self {
//             Self {
//                 repository: UserRepository,
//             }
//         }
//     }
//
//     #[test]
//     fn it_compiles() {
//         let service = UserService::new();
//         let user = service.get(1);
//         assert!(user.is_ok());
//     }
// }
