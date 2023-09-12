use aws_sdk_dynamodb::config::Builder;
use aws_sdk_dynamodb::Client;

pub async fn dynamodb_client() -> Client {
    let config = aws_config::load_from_env().await;

    let dynamodb = Builder::from(&config).build();

    Client::from_conf(dynamodb)
}
