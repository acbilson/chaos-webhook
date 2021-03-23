.POSIX:

.PHONY: config
config: ## configures templates from env file
	. ./.env && envsubst < template/hooks-dev.json > webhook/hooks.json; \
	. ./.env && envsubst < template/build-site.sh > webhook/build-site.sh

.PHONY: build
build: config ## builds the webhook docker image
	docker build -f webhook/Dockerfile -t acbilson/webhook:alpine-3.12 .

.PHONY: start
start: build ## run the webhook docker image
	docker run --rm -d -p 9000:9000 --name webhook \
    -v ~/source/alexbilson.dev/hugo/config:/etc/hugo \
    -v ~/source/chaos-content:/mnt/chaos/content \
    -v ~/source/chaos-theme:/mnt/chaos/themes/chaos \
    acbilson/webhook:alpine-3.12 .

.PHONY: clean
clean: ## stops the webhook docker image and cleans up the file system
	docker stop webhook; \
  rm webhook/hooks.json webhook/build-site.sh

.PHONY: test
test: ## tests the webhook docker image
	curl http://localhost:9000/hooks/content-pull-webhook; \
  docker logs -f webhook

.PHONY: help
help: ## show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | \
	sort | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

