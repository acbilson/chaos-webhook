#!/bin/bash
. .env

ENVIRONMENT=$1
WITH_SERVICE=$2

case $ENVIRONMENT in

uat)

  if [ $WITH_SERVICE = 1 ]; then

    echo "creates pod in uat..."
    ssh -t ${UAT_HOST} \
      sudo podman pod create --name webhook --publish ${UAT_EXPOSED_PORT}:9000 --publish 9101:80

    echo "runs webhook container in uat..."
    ssh -t ${UAT_HOST} \
      sudo podman run --rm -d \
        -v ${CONTENT_PATH}/chaos-content:/mnt/chaos/content \
        -v ${THEME_PATH}/chaos-theme:/mnt/chaos/themes/chaos \
        -v ${SITE_PATH}/uat:/var/www/uat \
        --pod webhook \
        --name webhook-uat \
        acbilson/webhook-uat:alpine

    echo "runs chaos-theme-service container in uat..."
    ssh -t ${UAT_HOST} \
      sudo podman run --rm -d \
        -e "FLASK_SECRET_KEY=test-key" \
        -v ${CONTENT_PATH}/chaos-content:/mnt/chaos/content \
        --pod webhook \
        --name chaos-theme-service-uat \
        acbilson/chaos-theme-service-uat:alpine

  else

    echo "runs webhook container in uat..."
    ssh -t ${UAT_HOST} \
      sudo podman run --rm -d \
        --expose ${UAT_EXPOSED_PORT} -p ${UAT_EXPOSED_PORT}:9000 \
        -v ${CONTENT_PATH}/chaos-content:/mnt/chaos/content \
        -v ${THEME_PATH}/chaos-theme:/mnt/chaos/themes/chaos \
        -v ${SITE_PATH}/uat:/var/www/uat \
        --name webhook-uat \
        acbilson/webhook-uat:alpine
  fi
;;

prod)
  echo "enabling webhook pod..."
  ssh -t ${PROD_HOST} sudo systemctl daemon-reload
  ssh -t ${PROD_HOST} sudo systemctl enable --now pod-webhook.service
;;

*)
  echo "please provide one of the following as the first argument: uat, prod."
  exit 1

esac
