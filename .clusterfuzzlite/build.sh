#!/bin/bash -eu
        set -euo pipefail
        cd "$(dirname "$0")/.."
        cargo fuzz build --offline
