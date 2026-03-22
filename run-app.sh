#!/bin/bash

set -euo pipefail

BUILD_MODE="debug"

while [[ $# -gt 0 ]]; do
	case $1 in
	-r | --release)
		BUILD_MODE="release"
		shift
		;;
	-d | --debug)
		BUILD_MODE="debug"
		shift
		;;
	-h | --help)
		echo "Usage: $0 [OPTIONS]"
		echo "Options:"
		echo "  -r, --release    Run in release mode"
		echo "  -d, --debug      Run in debug mode (default)"
		echo "  -n, --name NAME  Specify binary name (default: smart-home)"
		echo "  -h, --help       Show this help message"
		exit 0
		;;
	*)
		break
		;;
	esac
done

BINARY_FOLDER_PATH="/root/workspace/smart-home/target/$BUILD_MODE"

if [[ ! -d "$BINARY_FOLDER_PATH" ]]; then
	echo "Error: binary not found at $BINARY_FOLDER_PATH"
	exit 1
fi

cd $BINARY_FOLDER_PATH
./smart-home
