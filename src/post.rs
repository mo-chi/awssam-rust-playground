use http::{Method, StatusCode};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use tracing::{error, info};
use uuid::Uuid;

use model::User;
use repository::UserRepository;

mod aws;
mod model;
mod repository;
mod response;
mod setting;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("event: {:#?}", event);

    if event.method() != Method::POST {
        return response::builder(StatusCode::METHOD_NOT_ALLOWED, Body::Empty);
    }

    // データ登録用オブジェクトを生成する
    let mut user: User = Default::default();

    // event の body からユーザ名を取得する
    let body = event.body();
    if let Body::Text(json_string) = body {
        user = serde_json::from_str(json_string).unwrap();
    }
    // id を設定する
    let id = Uuid::new_v4();
    user.id = Some(id.to_string());

    // ユーザリポジトリを作成する
    let user_repository = UserRepository::new().await;

    // データの登録を行う
    match user_repository.put_item(user).await {
        Ok(resp) => {
            info!("dynamodb response ok: {:#?}", resp);
        }
        Err(e) => {
            error!("dynamodb response err: {:#?}", e);
            return response::builder(StatusCode::INTERNAL_SERVER_ERROR, Body::Empty);
        }
    }

    response::builder(StatusCode::CREATED, Body::Empty)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
