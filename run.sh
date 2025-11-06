#!/bin/sh

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR" || exit 1

# Binary name from Cargo.toml
BINARY_NAME="theclicker"
BINARY_PATH="target/release/$BINARY_NAME"

# Function to show help
show_help() {
    cat << EOF
Usage: $0 [OPTIONS]

Options:
    -c <path>    Set config location for config.json
    -cd          Use default config.json from repo root
    -i           Run in interactive mode
    -r, --rebuild Force rebuild of the binary
    -h, --help   Show this help message

Examples:
    $0                    # Show help
    $0 -cd                # Use default config.json from repo root
    $0 -c /path/to/config.json
    $0 -i                 # Run in interactive mode
    $0 -cd -i             # Use default config and run interactively
    $0 -r                 # Force rebuild and run
    $0 -r -cd             # Force rebuild and use default config
EOF
}

# Function to build the binary
build_binary() {
    echo "Building release binary..."
    if ! command -v cargo >/dev/null 2>&1; then
        echo "Error: cargo not found. Please install Rust toolchain or build manually."
        exit 1
    fi
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "Error: Build failed."
        exit 1
    fi
}

# Parse arguments
CONFIG_ARG=""
DEFAULT_CONFIG=false
INTERACTIVE=false
REBUILD=false

while [ $# -gt 0 ]; do
    case "$1" in
        -h|--help)
            show_help
            exit 0
            ;;
        -c)
            if [ -z "$2" ]; then
                echo "Error: -c requires a path argument"
                show_help
                exit 1
            fi
            CONFIG_ARG="--config $2"
            shift 2
            ;;
        -cd)
            DEFAULT_CONFIG=true
            shift
            ;;
        -i)
            INTERACTIVE=true
            shift
            ;;
        -r|--rebuild)
            REBUILD=true
            shift
            ;;
        *)
            echo "Error: Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Build the binary if rebuild is requested or if it doesn't exist
if [ "$REBUILD" = true ] || [ ! -f "$BINARY_PATH" ]; then
    if [ "$REBUILD" = true ] && [ -f "$BINARY_PATH" ]; then
        echo "Rebuild requested. Rebuilding..."
    fi
    build_binary
fi

# If no arguments provided, show help
if [ -z "$CONFIG_ARG" ] && [ "$DEFAULT_CONFIG" = false ] && [ "$INTERACTIVE" = false ] && [ "$REBUILD" = false ]; then
    show_help
    exit 0
fi

# Build command
CMD="$BINARY_PATH"

# Add config argument
if [ "$DEFAULT_CONFIG" = true ]; then
    CMD="$CMD --default"
elif [ -n "$CONFIG_ARG" ]; then
    CMD="$CMD $CONFIG_ARG"
fi

# Add debug/beep flags if needed (can be extended)
# For now, just run with the config/interactive flags

# Interactive mode means no command subcommand is provided
# The binary will automatically enter interactive mode when no command is given
if [ "$INTERACTIVE" = true ]; then
    # Just run without any command subcommand
    exec $CMD
else
    # If not interactive, the config will be loaded and used
    # The binary will use the config's command if available
    exec $CMD
fi

