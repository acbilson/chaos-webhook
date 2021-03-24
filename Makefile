.POSIX:

.PHONY: config-dev
config-dev: ## configures dev templates from env file
	. ./.env && envsubst < template/hooks-dev.json > webhook/hooks.json; \
 	cp template/build-site.sh webhook/build-site.sh

.PHONY: config-prod
config-prod: ## configures prod templates from env file
	. ./.env && envsubst < template/hooks-prod.json > webhook/hooks.json; \
	cp template/build-site.sh webhook/build-site.sh

.PHONY: build-dev
build-dev: config-dev ## builds the webhook docker image in dev configuration
	docker build -f webhook/Dockerfile -t acbilson/webhook:alpine-3.12 .

.PHONY: build-prod
build-prod: config ## builds the webhook docker image in prod configuration
	sudo podman build -f webhook/Dockerfile -t acbilson/webhook:alpine-3.12 .

.PHONY: start-dev
start-dev: build-dev ## run the webhook docker image in development
	docker run --rm -d -p 9000:9000 --name webhook \
		-v ~/source/alexbilson.dev/hugo/config:/etc/hugo \
		-v ~/source/chaos-content:/mnt/chaos/content \
		-v ~/source/chaos-theme:/mnt/chaos/themes/chaos \
		acbilson/webhook:alpine-3.12 .

.PHONY: start-prod
start-prod: ## run the webhook docker image in production
	echo 'production webhook will be deployed across multiple containers'

.PHONY: test
test: ## tests the webhook dev docker image
	curl http://localhost:9000/hooks/content-pull-webhook; \
	docker logs -f webhook

.PHONY: help
help: ## show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | \
	sort | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

