#/bin/sh
/usr/bin/podman run \
--conmon-pidfile /run/container-webhook.pid \
--cidfile /run/container-webhook.ctr-id \
--cgroups=no-conmon \
--replace --rm -d \
-p 9000:9000 \
--name webhook \
-v /mnt/msata/source/alexbilson.dev/hugo/config:/etc/hugo \
-v /mnt/msata/source/chaos-content:/mnt/chaos/content \
-v /mnt/msata/source/chaos-theme:/mnt/chaos/themes/chaos \
acbilson/webhook:alpine-3.12
