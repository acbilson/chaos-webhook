#!/bin/bash
. .env

ENVIRONMENT=$1

case $ENVIRONMENT in

uat)
  echo "runs container in uat..."
  ssh -t ${UAT_HOST} \
    sudo podman run --rm -d \
      --expose ${UAT_EXPOSED_PORT} -p ${UAT_EXPOSED_PORT}:9000 \
      -v ${UAT_CONTENT_PATH}/chaos-content:/mnt/chaos/content \
      -v ${UAT_THEME_PATH}/chaos-theme:/mnt/chaos/themes/chaos \
      -v ${UAT_SITE_PATH}/uat:/var/www/site \
      --name webhook-uat \
      acbilson/webhook-uat:alpine-3.12
;;

prod)
  echo "deploying systemctl service file..."
  scp dist/container-webhook.service ${PROD_HOST}:/etc/system/systemd/

  echo "enabling micropub service..."
  ssh -t ${PROD_HOST} sudo systemctl daemon-reload && sudo systemctl enable container-webhook.service
;;

*)
  echo "please provide one of the following as the first argument: uat, prod."
  exit 1

esac
