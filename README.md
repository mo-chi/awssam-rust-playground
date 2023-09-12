# AWS SAM Rust Playground

Rust での AWS SAM の検証用リポジトリです

※ リポジトリ作成の 2023/09/12 では, Rust の AWS SAM は Beta 版の状態です

## 構成情報

- AWS SAM CLI: 1.95.0
- Cargo Lambda: 0.20.4

## 環境構築

### ライブラリのインストール

AWS SAM CLI などのライブラリをインストールする

```sh
pip install -r requirements.txt
```

### cargo-lambda のインストール

```sh
cargo install cargo-lambda@0.20.4
```

### 検証用で使う DynamoDB のテーブルを作成する

```sh
aws cloudformation create-stack \
    --stack-name cfn-playground-stack \
    --template-body file://./cfn/dynamodb.yaml
```

### イベント情報を作成する

**既存のイベント情報をコピーする**

```sh
cp events/event.json events/get.json
```

**コピーしたイベント情報を編集する**

```sh
vim events/get.json
{
    "resource": "/users/{id}",
    "path": "/users/1",
    "httpMethod": "GET | POST | PUT | DELETE",
    "pathParameters": {
        "id": "1"
    },
    "queryStringParameters": {
        "name": "alice"
    },
    "body": "{ \"name\": \"bob\" }",
    ~~~~~~~~~~~~~ 省略 ~~~~~~~~~~~~~
}
```

### 環境情報を作成する

**既存の環境情報をコピーする**

```sh
cp envs/origin.json envs/local.json
```

**環境に合わせ設定を変更する**

```sh
vim envs/local.json
{
    "Parameters": {
        "DYNAMODB_TABLE_NAME": "cfn-dev-dynamodb-table"
    }
}
```

### ビルドを行う

```sh
sam build --beta-features
```

### 実行する

```sh
sam local invoke GetFunction --event events/get.json --env-vars envs/local.json
```

## デプロイ手順

### ビルドを行う

```sh
sam build --beta-features
```

### デプロイ定義ファイルを作成する

初めての環境にデプロイする際は, 予めデプロイの定義を作成しデプロイする

```sh
sam deploy --guided
```

### 再デプロイの方法

既にデプロイの定義がある際は, 下記のコマンドでデプロイする

```sh
sam deploy
```

### API Gateway のデプロイ

AWS SAM で API Gateway の定義は変更されても, API Gateway 自体をデプロイしないと<br />
定義反映されないため、下記のコマンドで API Gateway をデプロイする

```sh
aws apigateway create-deployment --rest-api-id `<REST_API_ID>` --stage-name `<ステージ名>`
```

### API Gateway へリクエストする

```sh
curl -X GET https://<REST_API_ID>.execute-api.<リージョン名>.amazonaws.com/<ステージ名>/device
```

## ドキュメント

- [Cargo Lambda を使用した Rust Lambda 関数の構築 - AWS Serverless Application Model](https://docs.aws.amazon.com/ja_jp/serverless-application-model/latest/developerguide/building-rust.html)
- [Rust で Lambda 関数を構築する - AWS Lambda](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/lambda-rust.html)
- [Cargo Lambda | Rust functions on AWS Lambda made simple](https://www.cargo-lambda.info/)
