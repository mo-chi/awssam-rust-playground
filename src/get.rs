use http::{Method, StatusCode};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use tracing::{error, info};

use model::User;
use repository::UserRepository;

mod aws;
mod model;
mod repository;
mod response;
mod setting;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("event: {:#?}", event);

    if event.method() != Method::GET {
        return response::builder(StatusCode::METHOD_NOT_ALLOWED, Body::Empty);
    }

    // `pathParameters` が無いため パスの `/local/users/1` から, 末尾の `1` の部分を取得する
    let paths: Vec<&str> = event.uri().path().split('/').collect();
    if paths.len() < 4 {
        return response::builder(StatusCode::BAD_REQUEST, Body::Empty);
    }

    // パスから末尾の id を取得する
    let id = paths.last().unwrap();

    // ユーザリポジトリを作成する
    let user_repository = UserRepository::new().await;

    // DynamoDB の Get Item API を実行する
    let resp = match user_repository.get_item(id).await {
        Ok(resp) => {
            info!("dynamodb response ok: {:#?}", resp);
            // データ取得できなかった場合
            if resp.item.is_none() {
                let body = format!("id={} recode not found.", id).into();

                return response::builder(StatusCode::NOT_FOUND, body);
            }
            resp
        }
        Err(e) => {
            error!("dynamodb response err: {:#?}", e);
            return response::builder(StatusCode::INTERNAL_SERVER_ERROR, Body::Empty);
        }
    };

    let item = resp.item.unwrap();

    // 構造体に取得したデータを設定する
    let user: User = User::new(item);

    let body = serde_json::to_string(&user).unwrap().into();

    response::builder(StatusCode::OK, body)
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
