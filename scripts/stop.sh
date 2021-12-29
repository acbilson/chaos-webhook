#!/bin/bash
. .env

ENVIRONMENT=$1
WITH_SERVICE=$2

case $ENVIRONMENT in

uat)

  if [ $WITH_SERVICE = 1 ]; then

    echo "stops webhook container in uat..."
    ssh -t ${UAT_HOST} \
      sudo podman stop webhook-uat

    echo "stops chaos-theme-service container in uat..."
    ssh -t ${UAT_HOST} \
      sudo podman stop chaos-theme-service-uat

    echo "removes webhook pod in uat..."
    ssh -t ${UAT_HOST} \
      sudo podman pod rm webhook

  else

    echo "stops webhook container in uat..."
    ssh -t ${UAT_HOST} \
      sudo podman stop webhook-uat
  fi
;;

prod)
  echo "stops container in production..."
  ssh -t ${PROD_HOST} sudo systemctl disable pod-webhook
  ssh -t ${PROD_HOST} sudo podman stop chaos-theme-service
  ssh -t ${PROD_HOST} sudo podman stop webhook
  ssh -t ${PROD_HOST} sudo podman pod rm webhook-pod
;;

*)
  echo "please provide one of the following as the first argument: uat, prod."
  exit 1

esac
