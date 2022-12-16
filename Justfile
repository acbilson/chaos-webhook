# builds a development podman image
build:
	podman build --target=dev -t acbilson/webhook-dev:alpine .

# starts the development podman image
start:
	podman run --rm \
		--expose 9000 -p 9000:9000 \
		-v /Users/alexbilson/source/chaos-content:/mnt/chaos/content \
		-v /Users/alexbilson/source/chaos-theme:/mnt/chaos/themes/chaos \
		-v /Users/alexbilson/source/chaos-webhook/etc:/etc/webhook \
		--name webhook \
		acbilson/webhook-dev:alpine

# sends a webhook to the running podman image
test:
	xh -v post localhost:9000/hooks/content-pull-webhook @tests/chaos-theme-pull.json content-type:application/json

# installs the tagparser binary to ~/.local/cargo/bin/
install:
	cargo install --path ./tagparser
