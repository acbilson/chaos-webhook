#!/bin/bash
. .env

TEST_TYPE=$1

case $TEST_TYPE in

smoke)
  echo "confirms health check"
	curl ${UAT_SITE} --verbose
;;

*)
  echo "please provide one of the following as the first argument: unit, smoke."
  exit 1

esac
