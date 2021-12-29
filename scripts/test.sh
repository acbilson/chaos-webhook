#!/bin/bash
. .env

xh -v post localhost:9000/hooks/content-pull-webhook
