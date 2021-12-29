#!/bin/bash
. .env

ENVIRONMENT=$1

case $ENVIRONMENT in

dev)

  echo "creates files from template..."
  mkdir -p dist && \
    envsubst < template/hooks-dev.json > dist/hooks.json && \
    envsubst < template/config-dev.toml > dist/config-dev.toml && \
    cp template/test-site.sh dist/test-site.sh && \
    cp -r tagparser dist/src

  echo "builds DEV image"
    docker build -f Dockerfile \
      --target=dev \
    --build-arg EXPOSED_PORT=${EXPOSED_PORT} \
      -t acbilson/webhook-dev:alpine .
;;

uat)

  echo "creates files from template..."
  echo "${UAT_BRANCH}"
  mkdir -p dist/dist && \
    envsubst < template/hooks.json > dist/dist/hooks.json && \
    envsubst < template/config-uat.toml > dist/dist/config-uat.toml && \
    envsubst < template/config-prod.toml > dist/dist/config-prod.toml && \
    cp -r tagparser dist/src && \
    cp template/build-site.sh dist/dist/build-site.sh

  echo "copies files to distribute..."
  cp Dockerfile dist/

  echo "distributes dist/ folder..."
  scp -r dist ${UAT_HOST}:/mnt/msata/build/uat

  echo "builds image on UAT"
  ssh -t ${UAT_HOST} \
    sudo podman build \
      -f /mnt/msata/build/uat/Dockerfile \
      --target=uat \
      -t acbilson/webhook-uat:alpine \
      /mnt/msata/build/uat
;;

prod)
  echo "creates files from template..."
  mkdir -p dist/dist && \
    envsubst < template/hooks.json > dist/dist/hooks.json && \
    envsubst < template/config-prod.toml > dist/dist/config-prod.toml && \
    envsubst < template/config-uat.toml > dist/dist/config-uat.toml && \
    cp -r tagparser dist/src && \
    cp template/build-site.sh dist/dist/build-site.sh && \
    envsubst < template/container-webhook.service > dist/container-webhook.service

  echo "copies files to distribute..."
  cp Dockerfile dist/

  echo "distributes dist/ folder..."
  scp -r dist ${PROD_HOST}:/mnt/msata/build/prod

  echo "builds image on production"
  ssh -t ${PROD_HOST} \
    sudo podman build \
      -f /mnt/msata/build/prod/Dockerfile \
      --target=prod \
      -t acbilson/webhook:alpine \
      /mnt/msata/build/prod
;;

*)
  echo "please provide one of the following as the first argument: uat, prod."
  exit 1

esac
