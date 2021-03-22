.POSIX:

.PHONY: build
build: ## builds the webhook docker image
	docker build -f webhook/Dockerfile -t acbilson/webhook:alpine-3.12 .

.PHONY: run
run: ## run the webhook docker image
	docker run --rm acbilson/webhook:alpine-3.12 .

.PHONY: help
help: ## show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | \
	sort | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

