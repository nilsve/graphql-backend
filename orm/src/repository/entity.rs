use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item, to_item, Item};

pub trait Entity: Serialize + for<'a> Deserialize<'a> + Send + 'static {
    type PrimaryKey;
    type IndexFields;

    fn get_primary_key(&self) -> Self::PrimaryKey;

    fn get_index_fields(&self) -> Self::IndexFields;

    fn from_attribute_values(
        values: HashMap<String, AttributeValue>,
    ) -> Result<Self, serde_dynamo::Error> {
        let item: Item = values.into();
        let entity: Self = from_item(item)?;

        Ok(entity)
    }

    fn serialize_primary_key(&self) -> HashMap<String, AttributeValue>
    where
        Self::PrimaryKey: Serialize,
    {
        to_item::<Self::PrimaryKey, Item>(self.get_primary_key())
            .expect("Failed to serialize primary key")
            .into()
    }

    fn serialize_index_fields(&self) -> HashMap<String, AttributeValue>
    where
        Self::IndexFields: Serialize,
    {
        to_item::<Self::IndexFields, Item>(self.get_index_fields())
            .expect("Failed to serialize index fields")
            .into()
    }

    fn serialize(&self) -> HashMap<String, AttributeValue>
    where
        Self::PrimaryKey: Serialize,
        Self::IndexFields: Serialize,
    {
        let serialized: Item = to_item(self).expect("Failed to serialize entity");

        serialized.into()
    }

    fn serialize_with_indexes(&self) -> HashMap<String, AttributeValue>
    where
        Self::PrimaryKey: Serialize,
        Self::IndexFields: Serialize,
    {
        let serialized: Item = to_item(self).expect("Failed to serialize entity");

        let mut casted_entity: HashMap<String, AttributeValue> = serialized.into();
        let casted_primary_key: HashMap<String, AttributeValue> =
            to_item::<Self::PrimaryKey, Item>(self.get_primary_key())
                .expect("Failed to serialize primary key")
                .into();
        let casted_index_fields: HashMap<String, AttributeValue> =
            to_item::<Self::IndexFields, Item>(self.get_index_fields())
                .expect("Failed to serialize index fields")
                .into();

        casted_entity.extend(casted_index_fields);
        casted_entity.extend(casted_primary_key);

        casted_entity
    }
}

// Unit test for serialization
#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use aws_sdk_dynamodb::types::AttributeValue;
    use serde::{Deserialize, Serialize};
    use serde_dynamo::{to_item, Item};

    use crate::prelude::Entity;

    #[derive(Serialize, Deserialize)]
    struct TestEntity {
        id: i32,
        title: String,
        body: String,
    }

    #[derive(Serialize, Deserialize, Default)]
    struct PrimaryKey {
        pk: String,
        sk: String,
    }

    #[derive(Serialize, Deserialize, Default)]
    struct Indexes {
        gsi_1_pk: String,
        gsi_1_sk: String,
    }

    impl Entity for TestEntity {
        type PrimaryKey = PrimaryKey;
        type IndexFields = Indexes;

        fn get_primary_key(&self) -> Self::PrimaryKey {
            PrimaryKey::default()
        }

        fn get_index_fields(&self) -> Self::IndexFields {
            Indexes::default()
        }
    }

    #[test]
    fn bla() {
        let entity = TestEntity {
            id: 1,
            title: "Hello".to_string(),
            body: "World".to_string(),
        };

        let item: Item = to_item(&entity).unwrap();

        let dynamo_item: HashMap<String, AttributeValue> = item.into();
    }
}
