use diesel::{NotFound, QueryResult};

pub trait Entity {}

pub trait Repository<E, NE>: 'static {
    type Connection;

    fn get_connection(&self) -> Self::Connection;
    fn create(&self, item: NE) -> QueryResult<usize>;
    fn update(&self, item: E) -> QueryResult<usize>;
    fn delete(&self, item: E) -> QueryResult<usize>;
    fn find(&self, id: i32) -> QueryResult<Option<E>>;
    fn get(&self, id: i32) -> QueryResult<E> {
        return if let Some(value) = self.find(id)? {
            Ok(value)
        } else {
            Err(NotFound)
        };
    }
    fn find_all(&self) -> QueryResult<Vec<E>>;
}

#[cfg(test)]
mod tests {
    use crate::repository::Entity;
    use crate::service::Model;

    struct UserModel {
        id: i32,
        name: String,
    }

    struct UserEntity {
        id: i32,
        name: String,
    }

    impl Entity for UserEntity {}

    impl Model for UserModel {
        type Entity = UserEntity;
        fn to_entity(self) -> Self::Entity {
            UserEntity {
                id: self.id,
                name: self.name,
            }
        }

        fn from_entity(entity: Self::Entity) -> Self {
            UserModel {
                id: entity.id,
                name: entity.name,
            }
        }
    }

    #[test]
    fn it_maps() {
        let entity = UserEntity {
            id: 1,
            name: "name".to_string(),
        };
        let model = UserModel {
            id: 1,
            name: "name".to_string(),
        };
        let mapped_model = UserModel::from_entity(entity);
        assert_eq!(mapped_model.id, model.id);
        assert_eq!(mapped_model.name, model.name);
    }
}
