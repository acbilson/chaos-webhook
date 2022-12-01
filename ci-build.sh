#!/bin/bash

selected_rev=${1:0:7}
current_rev=$(git rev-parse --short HEAD)

if [ $selected_rev = $current_rev ]; then
   podman build --target=prod -t acbilson/webhook-$current_rev:alpine -t acbilson/webhook:latest .;
else
	echo "the current git commit ($current_rev) and the selected commit ($selected_rev) do not match";
fi
