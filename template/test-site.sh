#!/bin/sh

DIST_PATH=/var/www/site
CONFIG_PATH=/etc/hugo/config-dev.toml

if [ -d /tmp/hugo_cache ]; then
  echo "\n\nclearing cache from last build"
  echo "################"
  rm /tmp/hugo_cache/filecache/getjson/*
fi

echo "\n\nbuilding site to $DIST_PATH"
echo "################"
/usr/bin/hugo \
  -d $DIST_PATH \
  --config $CONFIG_PATH \
  --contentDir /mnt/hugo/content \
  --themesDir /mnt/hugo/themes \
  --cleanDestinationDir

echo "\n\nbuilding network tags"
echo "################"
/usr/local/bin/tagparser \
  --root /mnt/hugo/content \
  --output $DIST_PATH/network/diagram.json
