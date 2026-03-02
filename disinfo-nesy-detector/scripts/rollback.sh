#!/bin/bash
# Usage: ./scripts/rollback.sh <tag>
TAG=${1:-latest}
podman service rollback detector --to-image $IMAGE_NAME:$TAG
