#!/bin/bash
. .env

ENVIRONMENT=$1

case $ENVIRONMENT in

dev)
  echo "backing up sensitive .env file"
  mkdir -p ~/safe/webhook && cp .env ~/safe/webhook/env.bk

  echo "removing dist/"
  rm -rf dist/
;;

uat)
  ssh -t ${UAT_HOST} \
    rm -rf /mnt/msata/build/uat
;;

prod)
  ssh -t ${PROD_HOST} \
    rm -rf /mnt/msata/build/prod
;;

*)
  echo "please provide one of the following as the first argument: dev, uat, prod."
  exit 1

esac
