# Dockerfile for https://github.com/adnanh/webhook
# example modified from https://almir/docker-webhook
FROM        golang:alpine3.12 AS build
ENV         WEBHOOK_VERSION 2.8.0

# add build deps
RUN         apk add --update -t build-deps curl libc-dev gcc libgcc

# build the webhook project
WORKDIR     /go/src/github.com/adnanh/webhook
RUN         curl -L --silent -o webhook.tar.gz https://github.com/adnanh/webhook/archive/${WEBHOOK_VERSION}.tar.gz && \
            tar -xzf webhook.tar.gz --strip 1 &&  \
            go get -d && \
            go build -o /usr/local/bin/webhook

# build the tagparser project
COPY        src /go/src/tagparser
WORKDIR     /go/src/tagparser
RUN         go version && go install

FROM        alpine:3.12 as base
COPY        --from=build /usr/local/bin/webhook /usr/local/bin/webhook
COPY        --from=build /go/bin/tagparser /usr/local/bin/tagparser
RUN         apk add hugo git

# adds hugo configs
COPY        dist/config-prod.toml /etc/hugo/
COPY        dist/config-uat.toml /etc/hugo/

# adds site build script
COPY        dist/build-site.sh /usr/local/bin/
RUN         chmod +x /usr/local/bin/build-site.sh

# adds hooks
COPY        dist/hooks.json /etc/webhook/

FROM        base as uat
ENTRYPOINT  ["/usr/local/bin/webhook", "-debug", "-verbose", "-hooks", "/etc/webhook/hooks.json"]

FROM        base as prod
ENTRYPOINT  ["/usr/local/bin/webhook", "-verbose", "-hooks", "/etc/webhook/hooks.json"]
