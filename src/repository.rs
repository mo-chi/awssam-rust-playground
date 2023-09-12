use aws_sdk_dynamodb::operation::delete_item::{DeleteItemError, DeleteItemOutput};
use aws_sdk_dynamodb::operation::get_item::{GetItemError, GetItemOutput};
use aws_sdk_dynamodb::operation::put_item::{PutItemError, PutItemOutput};
use aws_sdk_dynamodb::operation::update_item::{UpdateItemError, UpdateItemOutput};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use aws_smithy_http::result::SdkError;
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;

use crate::aws;
use crate::model::User;
use crate::setting::GLOBAL;

pub struct UserRepository {
    client: Client,
}

#[allow(unused)]
impl UserRepository {
    pub async fn new() -> UserRepository {
        // DynamoDB のクライアントを取得する
        let client: Client = aws::dynamodb_client().await;

        UserRepository { client }
    }

    pub async fn get_item(
        &self,
        id: &&str,
    ) -> Result<GetItemOutput, SdkError<GetItemError, HttpResponse>> {
        self.client
            .get_item()
            .table_name(GLOBAL.table_name.to_string())
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
    }

    pub async fn put_item(
        &self,
        user: User,
    ) -> Result<PutItemOutput, SdkError<PutItemError, HttpResponse>> {
        self.client
            .put_item()
            .table_name(GLOBAL.table_name.to_string())
            .item("id", AttributeValue::S(user.id.unwrap().to_string()))
            .item("name", AttributeValue::S(user.name.unwrap().to_string()))
            .send()
            .await
    }

    pub async fn update_item(
        &self,
        user: User,
    ) -> Result<UpdateItemOutput, SdkError<UpdateItemError, HttpResponse>> {
        self.client
            .update_item()
            .table_name(GLOBAL.table_name.to_string())
            .key("id", AttributeValue::S(user.id.unwrap().to_string()))
            .update_expression("SET #name = :name")
            .expression_attribute_names("#name", "name")
            .expression_attribute_values(":name", AttributeValue::S(user.name.unwrap().to_string()))
            .send()
            .await
    }

    pub async fn delete_item(
        &self,
        id: &&str,
    ) -> Result<DeleteItemOutput, SdkError<DeleteItemError, HttpResponse>> {
        self.client
            .delete_item()
            .table_name(GLOBAL.table_name.to_string())
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
    }
}
