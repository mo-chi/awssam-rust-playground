use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[allow(unused)]
impl User {
    pub fn new(item: HashMap<String, AttributeValue>) -> User {
        let mut user: User = Default::default();
        user.set(item);
        user
    }

    pub fn set(&mut self, item: HashMap<String, AttributeValue>) {
        if let Some(attr_val) = item.get("id") {
            if let Ok(val) = attr_val.as_s() {
                self.id = Some(val.to_string());
            }
        }

        if let Some(attr_val) = item.get("name") {
            if let Ok(val) = attr_val.as_s() {
                self.name = Some(val.to_string());
            }
        }
    }
}
