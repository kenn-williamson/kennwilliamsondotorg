#!/bin/bash
# Download database backup with local testing support

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Logging functions
log() {
    echo -e "${GREEN}[$(date +'%H:%M:%S')] $1${NC}"
}

info() {
    echo -e "${BLUE}[$(date +'%H:%M:%S')] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[$(date +'%H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%H:%M:%S')] ERROR: $1${NC}"
    exit 1
}

show_help() {
    echo "Usage: $0 <server_host_or_localhost> <backup_filename> [ssh_key_path]"
    echo ""
    echo "Download database backup from remote server or local testing"
    echo ""
    echo "Arguments:"
    echo "  server_host_or_localhost  Remote server (user@host) or 'localhost' for testing"
    echo "  backup_filename          Name of backup file to download"
    echo "  ssh_key_path            Path to SSH private key (optional for remote)"
    echo ""
    echo "Examples:"
    echo "  Local test:  $0 localhost backup_file.sql.gz"
    echo "  Production:  $0 user@server.com backup_file.sql.gz ~/.ssh/key.pem"
    echo ""
    echo "Local Mode:"
    echo "  - Downloads from ./backups/ directory"
    echo "  - Useful for testing download workflow"
    echo ""
    echo "Remote Mode:"
    echo "  - Downloads from /var/backups/kennwilliamson/ on remote server"
    echo "  - SSH key authentication supported"
}

# Check for help flag first
case "$1" in
    -h|--help)
        show_help
        exit 0
        ;;
esac

# Parse arguments
if [ $# -lt 2 ] || [ $# -gt 3 ]; then
    show_help
    exit 1
fi

SERVER="$1"
BACKUP_FILE="$2"
SSH_KEY="$3"
LOCAL_DIR="$PROJECT_ROOT/backups"

# Directory validation - critical for backup operations
if [ ! -d "$LOCAL_DIR" ]; then
    warn "Backups directory not found at $LOCAL_DIR, creating..."
    mkdir -p "$LOCAL_DIR" || error "Failed to create backups directory"
fi

log "Starting backup download..."
info "Target file: $BACKUP_FILE"
info "Local directory: $LOCAL_DIR"

# Handle localhost testing vs remote server
if [ "$SERVER" == "localhost" ]; then
    info "🧪 Local testing mode - copying from local backups directory"

    SOURCE_FILE="$LOCAL_DIR/$BACKUP_FILE"
    if [ ! -f "$SOURCE_FILE" ]; then
        error "Local backup file not found: $SOURCE_FILE
        Available files in $LOCAL_DIR:"
        ls -la "$LOCAL_DIR" || echo "Directory is empty or inaccessible"
        exit 1
    fi

    # Copy with different name to distinguish from original
    DEST_FILE="$LOCAL_DIR/downloaded_$BACKUP_FILE"
    if cp "$SOURCE_FILE" "$DEST_FILE"; then
        log "✅ Local copy successful: $DEST_FILE"
        FINAL_FILE="$DEST_FILE"
    else
        error "Failed to copy local backup file"
    fi
else
    info "🌐 Remote download mode"
    info "Server: $SERVER"

    # SSH key validation
    if [ -n "$SSH_KEY" ]; then
        if [ ! -f "$SSH_KEY" ]; then
            error "SSH key file not found: $SSH_KEY"
        fi
        if [ ! -r "$SSH_KEY" ]; then
            error "SSH key file not readable: $SSH_KEY
            Check file permissions: chmod 600 $SSH_KEY"
        fi
        SCP_CMD="scp -i \"$SSH_KEY\""
        info "🔑 Using SSH key: $SSH_KEY"
    else
        SCP_CMD="scp"
        info "🔑 Using default SSH authentication"
    fi

    # Download backup file
    REMOTE_PATH="$SERVER:/var/backups/kennwilliamson/$BACKUP_FILE"
    LOCAL_PATH="$LOCAL_DIR/$BACKUP_FILE"

    info "Downloading from: $REMOTE_PATH"
    info "Downloading to: $LOCAL_PATH"

    if eval "$SCP_CMD \"$REMOTE_PATH\" \"$LOCAL_PATH\""; then
        log "✅ Backup downloaded: $LOCAL_PATH"
        FINAL_FILE="$LOCAL_PATH"
    else
        error "Download failed from $REMOTE_PATH
        Check:
        - Server connectivity and credentials
        - Remote file exists and is readable
        - SSH key permissions (chmod 600)
        - Network connectivity"
    fi
fi

# Display file information
if [ -f "$FINAL_FILE" ]; then
    FILE_SIZE=$(ls -lh "$FINAL_FILE" | awk '{print $5}')
    FILE_DATE=$(ls -lh "$FINAL_FILE" | awk '{print $6, $7, $8}')
    log "📦 Downloaded file information:"
    info "  Size: $FILE_SIZE"
    info "  Date: $FILE_DATE"
    info "  Path: $FINAL_FILE"
else
    error "Final file not found after download operation"
fi