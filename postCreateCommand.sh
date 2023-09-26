#!/bin/bash

USER="vscode"

echo "Post create command running..."
sudo chown -R $USER:$USER /usr/local/cargo/registry
echo "Post create command successfully finished!"
