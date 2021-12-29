#!/bin/bash
. .env

docker run --rm \
  --expose ${EXPOSED_PORT} -p ${EXPOSED_PORT}:80 \
  -v ${SOURCE_PATH}/chaos-content:/mnt/hugo/content \
  -v ${SOURCE_PATH}/chaos-theme:/mnt/hugo/themes/chaos \
  --name webhook \
  acbilson/webhook-dev:alpine
