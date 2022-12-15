#!/bin/sh

echo "\n\nrunning tagparser at /mnt/chaos/content"
/bin/mkdir -p /mnt/chaos/data
/usr/local/bin/tagparser /mnt/chaos/content /mnt/chaos/data

if [ -d /tmp/hugo_cache ]; then
  echo "\n\nclearing cache from last build"
  echo "################"
  rm /tmp/hugo_cache/filecache/getjson/*
fi

echo "\n\nbuilding site to $DIST_PATH"
echo "################"
/usr/bin/hugo \
  -d /var/www/site \
  --config /etc/webhook/config.toml \
  --contentDir /mnt/chaos/content \
  --themesDir /mnt/chaos/themes \
  --cleanDestinationDir
