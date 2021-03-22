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
            go build -o /usr/local/bin/webhook && \
            apk del --purge build-deps && \
            rm -rf /var/cache/apk/* && \
            rm -rf /go

FROM        alpine:3.12 as prod
COPY        --from=build /usr/local/bin/webhook /usr/local/bin/webhook
WORKDIR     /etc/webhook
COPY        webhook/hooks.json .

EXPOSE      9000
ENTRYPOINT  ["/usr/local/bin/webhook"]
