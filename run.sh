#!/bin/bash
# Script to run the OCC Recruitment CLI Application

# Check if a URL was provided as an argument
if [ $# -eq 0 ]; then
    echo "Usage: $0 <url>"
    echo "Example: $0 https://discord.gg/occ"
    echo "If no URL is provided, the default URL will be used."
    echo ""
    cargo run
else
    cargo run "$1"
fi