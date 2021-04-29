#!/bin/bash
. .env

ENVIRONMENT=$1

case $ENVIRONMENT in

uat)
  echo "runs container in uat..."
  ssh -t ${UAT_HOST} \
    sudo podman run --rm -d \
      --expose ${UAT_EXPOSED_PORT} -p ${UAT_EXPOSED_PORT}:9000 \
      -v ${CONTENT_PATH}/chaos-content:/mnt/chaos/content \
      -v ${THEME_PATH}/chaos-theme:/mnt/chaos/themes/chaos \
      -v ${SITE_PATH}/uat:/var/www/uat \
      --name webhook-uat \
      acbilson/webhook-uat:alpine-3.12
;;

prod)
  echo "enabling micropub service..."
  ssh -t ${PROD_HOST} sudo systemctl daemon-reload
  ssh -t ${PROD_HOST} sudo systemctl enable --now container-webhook.service
;;

*)
  echo "please provide one of the following as the first argument: uat, prod."
  exit 1

esac
