#!/bin/bash

case "$1" in
    "setup")
        echo "CI build optimization setup completed"
        ;;
    "features")
        cargo build --release --quiet
        ;;
    *)
        echo "Usage: $0 {setup|features}"
        exit 1
        ;;
esac
