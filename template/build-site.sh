#!/bin/sh

REF=$1
REPO=$2

BRANCH=unset
PATH=unset
DIST_PATH=unset

case $REF in

  refs/heads/master)
    BRANCH=master
    DIST_PATH=/var/www/site
  ;;

  refs/heads/uat)
    BRANCH=uat
    DIST_PATH=/var/www/uat
  ;;

  *)
    echo "$REF was not a valid git reference"
    echo "################"
    return 1
esac

case $REPO in

  chaos-content)
    PATH=/mnt/chaos/content
  ;;

  chaos-theme)
    PATH=/mnt/chaos/themes/chaos
  ;;

  *)
    echo "$REPO was not a valid git repo"
    echo "################"
    return 1
  ;;
esac

echo "checking out $BRANCH for content and themes"
echo "################"
cd /mnt/chaos/content
/usr/bin/git checkout $BRANCH
cd /mnt/chaos/themes/chaos
/usr/bin/git checkout $BRANCH

echo "pulling $BRANCH branch at $PATH"
echo "################"
cd $PATH
/usr/bin/git pull

echo "building site from $BRANCH to $DIST_PATH"
echo "################"
/usr/bin/hugo \
  -d $DIST_PATH \
  --config /etc/hugo/config.toml \
  --contentDir /mnt/chaos/content \
  --themesDir /mnt/chaos/themes \
  --cleanDestinationDir
