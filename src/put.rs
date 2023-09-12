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

    if event.method() != Method::PUT {
        return response::builder(StatusCode::METHOD_NOT_ALLOWED, Body::Empty);
    }

    // パス `/local/users/1` から末尾の 1 を取得する
    let paths: Vec<&str> = event.uri().path().split('/').collect();
    if paths.len() < 4 {
        return response::builder(StatusCode::BAD_REQUEST, Body::Empty);
    }

    // データ更新用のユーザオブジェクトを生成する
    let mut user: User = Default::default();

    // event の body からユーザ名を取得する
    let body = event.body();
    if let Body::Text(json_string) = body {
        user = serde_json::from_str(json_string).unwrap();
    }

    // パスから末尾の id を取得する
    let id = paths.last().unwrap();
    user.id = Some(id.to_string());

    // ユーザリポジトリを作成する
    let user_repository = UserRepository::new().await;

    // 対象データの存在チェックを行う
    match user_repository.get_item(id).await {
        Ok(resp) => {
            info!("dynamodb get response ok: {:#?}", resp);
            if resp.item.is_none() {
                let body = format!("id={} recode not found", id).into();

                return response::builder(StatusCode::NOT_FOUND, body);
            }
        }
        Err(e) => {
            error!("dynamodb get response err: {:#?}", e);
            return response::builder(StatusCode::INTERNAL_SERVER_ERROR, Body::Empty);
        }
    }

    // 対象データの更新を行う
    match user_repository.update_item(user).await {
        Ok(resp) => {
            info!("dynamodb update response ok: {:#?}", resp);
        }
        Err(e) => {
            error!("dynamodb update response err: {:#?}", e);
            return response::builder(StatusCode::INTERNAL_SERVER_ERROR, Body::Empty);
        }
    }

    response::builder(StatusCode::NO_CONTENT, Body::Empty)
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
