#!/bin/sh

REF=$1

case $REF in

refs/heads/uat)

  echo "pulling uat branch"
  git checkout uat && git pull

  echo "building from uat"
  /usr/bin/hugo \
    -d /var/www/site \
    --config /etc/hugo/config.toml \
    --contentDir /mnt/chaos/content \
    --themesDir /mnt/chaos/themes \
    --cleanDestinationDir
;;

refs/heads/master)

  echo "pulling master branch"
  git checkout master && git pull

  echo "building from master"
  /usr/bin/hugo \
    -d /var/www/site \
    --config /etc/hugo/config.toml \
    --contentDir /mnt/chaos/content \
    --themesDir /mnt/chaos/themes \
    --cleanDestinationDir
;;

*)
  echo "$1 was not a valid git reference"
  exit 1

esac
