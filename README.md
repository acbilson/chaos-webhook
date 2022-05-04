A webhook-triggered service for alexbilson.dev. Builds my own container to use Adnan's webhook.


# Configuration

The entire workflow can be accomplished via Make, however, you'll need to configure the scripts on which Make depends by creating a .env file. `env-example` supplies the required variables.


# Developer Process

This code flows through only two steps:

Verify -> Deploy

In verify (or <acronym title="User Acceptance Testing">UAT</acronym>), the goal is to confirm that this service is operable with its other services. It mimics the production environment.

In deploy, the goal is to deliver this service to my production server.

The processor architecture and container tooling I use in production differs from what I use locally. To avoid issues porting this service to the production server, I use the same server to conduct my UAT and production builds and tests.


# Verify

### Artifacts

Docker image: acbilson/webhook-uat:alpine
Hugo config: config-uat.toml
Hugo build script: build-site.sh
Hugo theme: [acbilson/chaos-theme](https://github.com/acbilson/chaos-theme.git)
Markdown content: [acbilson/chaos-content](https://github.com/acbilson/chaos-content.git)

### Dependencies

A production-like uat server with:

- Podman
- OpenSSH
- bash

## Build

To build a Docker verify image on the UAT server, run:

`make build-uat`

## Deploy

To deploy the image on the UAT server, run:

`make deploy-uat`

The UAT image uses a separate Github webhook to enhance security by having separate secrets.

## Test

Smoke tests are run on the UAT container with:

`make smoketest`

Right now, this only performs a health check.


# Deploy

### Artifacts

Docker image: acbilson/webhook:alpine
Hugo config: config.toml
Hugo build script: build-site.sh
Systemd service file: container-webhook.service
Hugo theme: [acbilson/chaos-theme](https://github.com/acbilson/chaos-theme.git)
Markdown content: [acbilson/chaos-content](https://github.com/acbilson/chaos-content.git)

### Dependencies

A production server with:

- Podman
- OpenSSH
- bash
- systemd
- A web proxy like Nginx to broadcast this service to the public Web

## Build

To build a Docker production image on the prod server, run:

`make build-prod`

## Run

To start the production service, run:

`make deploy`

The container is managed by systemd.
