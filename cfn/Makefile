all: delopy tables put scan

delopy: ## Create dynamodb table
	aws cloudformation create-stack \
		--stack-name cfn-playground-stack \
		--template-body file://./cfn/dynamodb.yaml

tables: ## Returns an array of table names
	aws dynamodb list-tables

put: ## Creates a new item
	aws dynamodb put-item \
		--table-name cfn-dev-dynamodb-table \
		--item '{ "id": { "S": "1" }, "name": { "S": "alice" } }'

scan: ## The Scan operation returns one or more items and item
	aws dynamodb scan \
		--table-name cfn-dev-dynamodb-table

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-8s\033[0m -- %s\n", $$1, $$2}'

.PHONY: help delopy tables put scan
.DEFAULT_GOAL := help
