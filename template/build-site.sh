#!/bin/sh
/usr/bin/hugo \
	-d /var/www/site \
	--config /etc/hugo/config.toml \
	--contentDir /mnt/chaos/content \
	--themesDir /mnt/chaos/themes \
	--cleanDestinationDir
