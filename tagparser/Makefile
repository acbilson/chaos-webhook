.POSIX:

##################
# Additional Tasks
##################

.PHONY: help
help: ## show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | \
	sort | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

##############
# Development Workflow
##############

.PHONY: clean
clean: ## cleans results of the tagparser exec
	rm -f diagram.json

.PHONY: run
run: clean ## runs tagparser locally
	go run .

.PHONY: test
test: ## tests tagparser
	go test -v .
