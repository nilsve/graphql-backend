use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Default)]
pub struct Indexes {
    pub pk: String,
    pub sk: String,
    pub gsi_1_pk: Option<String>,
    pub gsi_1_sk: Option<String>,
}

impl Indexes {
    pub fn dynamo_serialize(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("pk".to_string(), AttributeValue::S(self.pk.clone()));
        item.insert("sk".to_string(), AttributeValue::S(self.sk.clone()));

        if let Some(gsi_1_pk) = &self.gsi_1_pk {
            item.insert("gsi_1_pk".to_string(), AttributeValue::S(gsi_1_pk.clone()));
        }

        if let Some(gsi_1_sk) = &self.gsi_1_sk {
            item.insert("gsi_1_sk".to_string(), AttributeValue::S(gsi_1_sk.clone()));
        }

        item
    }
}
