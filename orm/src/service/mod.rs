use crate::repository::{Entity, Repository};
use diesel::QueryResult;

pub trait Model {
    type Entity: Entity;
    fn to_entity(self) -> Self::Entity;
    fn from_entity(entity: Self::Entity) -> Self;
}

pub trait CrudService<M, NM, R>
where
    M: Model,
    NM: Model,
    R: Repository<M::Entity, NM::Entity>,
{
    fn get_repository(&self) -> &R;
    fn create(&self, model: NM) -> QueryResult<usize> {
        self.get_repository().create(model.to_entity())
    }
    fn update(&self, model: M) -> QueryResult<usize> {
        self.get_repository().update(model.to_entity())
    }
    fn delete(&self, model: M) -> QueryResult<usize> {
        self.get_repository().delete(model.to_entity())
    }
    fn find(&self, id: i32) -> QueryResult<Option<M>> {
        let entity_result = self.get_repository().find(id);
        entity_result.map(|value| value.map(|value| Model::from_entity(value)))
    }
    fn get(&self, id: i32) -> QueryResult<M> {
        Ok(Model::from_entity(self.get_repository().get(id)?))
    }
    fn find_all(&self) -> QueryResult<Vec<M>> {
        Ok(self
            .get_repository()
            .find_all()?
            .into_iter()
            .map(|value| Model::from_entity(value))
            .collect())
    }
}

#[cfg(test)]
mod service_tests {
    use crate::prelude::Entity;
    use crate::repository::Repository;
    use crate::service::{CrudService, Model};
    use diesel::QueryResult;

    struct UserEntity {
        id: i32,
        name: String,
    }

    impl Entity for UserEntity {}

    struct UserModel {
        id: i32,
        name: String,
    }

    impl Model for UserModel {
        type Entity = UserEntity;
        fn to_entity(self) -> Self::Entity {
            UserEntity {
                id: self.id,
                name: self.name,
            }
        }

        fn from_entity(entity: Self::Entity) -> Self {
            Self {
                id: entity.id,
                name: entity.name,
            }
        }
    }

    struct NewUserEntity {
        name: String,
    }

    impl Entity for NewUserEntity {}

    struct NewUserModel {
        name: String,
    }

    impl Model for NewUserModel {
        type Entity = NewUserEntity;
        fn to_entity(self) -> Self::Entity {
            NewUserEntity { name: self.name }
        }

        fn from_entity(entity: Self::Entity) -> Self {
            Self { name: entity.name }
        }
    }

    struct UserRepository;
    impl Repository<UserEntity, NewUserEntity> for UserRepository {
        type Connection = ();

        fn get_connection(&self) -> () {
            todo!()
        }

        fn create(&self, item: NewUserEntity) -> QueryResult<usize> {
            todo!()
        }

        fn update(&self, item: UserEntity) -> QueryResult<usize> {
            todo!()
        }

        fn delete(&self, item: UserEntity) -> QueryResult<usize> {
            todo!()
        }

        fn find(&self, id: i32) -> QueryResult<Option<UserEntity>> {
            Ok(Some(UserEntity {
                name: "test".to_owned(),
                id: 1,
            }))
        }

        fn find_all(&self) -> QueryResult<Vec<UserEntity>> {
            todo!()
        }
    }

    struct UserService {
        repository: UserRepository,
    }

    impl CrudService<UserModel, NewUserModel, UserRepository> for UserService {
        fn get_repository(&self) -> &UserRepository {
            &self.repository
        }
    }

    impl UserService {
        fn new() -> Self {
            Self {
                repository: UserRepository,
            }
        }
    }

    #[test]
    fn it_compiles() {
        let service = UserService::new();
        let user = service.get(1);
        assert!(user.is_ok());
    }
}
