#!/bin/sh

set -e

export DEBIAN_FRONTEND
DEBIAN_FRONTEND=noninteractive

apt-get update
apt-get install --yes --no-install-recommends "$@"
apt-get clean
rm -Rf /var/lib/apt/lists/*
