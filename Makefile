all: get post put delete

init: ## Run install required libraries
	pip install -r requirements.txt
	cargo install cargo-lambda@0.20.4

build: ## Build AWS SAM function code
	sam build --beta-features

validate: ## Validate an AWS SAM Template
	sam validate --lint

deploy: build ## The sam deploy command creates a Cloudformation Stack and deploys your resources.
	sam deploy --no-confirm-changeset --no-fail-on-empty-changeset

get: ## Invoke GetFunction locally
	sam local invoke GetFunction --event events/get.json --env-vars envs/local.json

post: ## Invoke PostFunction locally
	sam local invoke PostFunction --event events/post.json --env-vars envs/local.json

put: ## Invoke PutFunction locally
	sam local invoke PutFunction --event events/put.json --env-vars envs/local.json

delete: ## Invoke DeleteFunction locally
	sam local invoke DeleteFunction --event events/delete.json --env-vars envs/local.json

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: help init build validate deploy get post put delete
.DEFAULT_GOAL := help
