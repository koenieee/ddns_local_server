#!/bin/bash

# DDNS Updater - Backup Cleanup Script
# Cleans up old backup files to prevent disk space issues

set -e

# Default values
BACKUP_DIR="/var/backups/nginx"
DAYS=30
VERBOSE=false
DRY_RUN=false

# Function to show usage
show_help() {
    cat << EOF
DDNS Updater - Backup Cleanup Script

Usage: $0 [OPTIONS]

Options:
  --backup-dir DIR    Directory containing backup files (default: /var/backups/nginx)
  --days DAYS         Delete files older than DAYS days (default: 30)
  --verbose           Enable verbose output
  --dry-run           Show what would be deleted without actually deleting
  --help              Show this help message

Examples:
  $0 --backup-dir /var/backups/nginx --days 7
  $0 --days 14 --verbose
  $0 --dry-run --verbose

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --backup-dir)
            BACKUP_DIR="$2"
            shift 2
            ;;
        --days)
            DAYS="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            show_help
            exit 1
            ;;
    esac
done

# Validate inputs
if [[ ! -d "$BACKUP_DIR" ]]; then
    echo "Error: Backup directory '$BACKUP_DIR' does not exist" >&2
    exit 1
fi

if ! [[ "$DAYS" =~ ^[0-9]+$ ]] || [[ "$DAYS" -lt 1 ]]; then
    echo "Error: Days must be a positive integer" >&2
    exit 1
fi

# Log function
log() {
    if [[ "$VERBOSE" == "true" ]]; then
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
    fi
}

# Main cleanup function
cleanup_backups() {
    log "Starting backup cleanup in: $BACKUP_DIR"
    log "Deleting files older than: $DAYS days"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        log "DRY RUN MODE - No files will be actually deleted"
    fi
    
    # Count files before cleanup
    total_files=$(find "$BACKUP_DIR" -type f -name "*.bak" -o -name "*.backup" -o -name "*~" 2>/dev/null | wc -l)
    old_files=$(find "$BACKUP_DIR" -type f \( -name "*.bak" -o -name "*.backup" -o -name "*~" \) -mtime +$DAYS 2>/dev/null | wc -l)
    
    log "Found $total_files total backup files"
    log "Found $old_files files older than $DAYS days"
    
    if [[ "$old_files" -eq 0 ]]; then
        log "No old backup files to clean up"
        return 0
    fi
    
    # Calculate space that will be freed
    if command -v du >/dev/null 2>&1; then
        old_size=$(find "$BACKUP_DIR" -type f \( -name "*.bak" -o -name "*.backup" -o -name "*~" \) -mtime +$DAYS -print0 2>/dev/null | du -ch --files0-from=- 2>/dev/null | tail -1 | cut -f1 || echo "unknown")
        if [[ "$old_size" != "unknown" ]]; then
            log "Space to be freed: $old_size"
        fi
    fi
    
    # Perform cleanup
    if [[ "$DRY_RUN" == "true" ]]; then
        log "Files that would be deleted:"
        find "$BACKUP_DIR" -type f \( -name "*.bak" -o -name "*.backup" -o -name "*~" \) -mtime +$DAYS -print 2>/dev/null | while read -r file; do
            echo "  - $file"
        done
    else
        deleted_count=0
        find "$BACKUP_DIR" -type f \( -name "*.bak" -o -name "*.backup" -o -name "*~" \) -mtime +$DAYS -print0 2>/dev/null | while IFS= read -r -d '' file; do
            if rm "$file" 2>/dev/null; then
                log "Deleted: $file"
                ((deleted_count++))
            else
                echo "Warning: Could not delete $file" >&2
            fi
        done
        
        log "Cleanup completed. Deleted $deleted_count files."
    fi
    
    # Clean up empty directories (if any backup files created subdirectories)
    if [[ "$DRY_RUN" == "false" ]]; then
        find "$BACKUP_DIR" -type d -empty -delete 2>/dev/null || true
    fi
}

# Run cleanup
cleanup_backups

# Summary
if [[ "$VERBOSE" == "true" ]]; then
    remaining_files=$(find "$BACKUP_DIR" -type f -name "*.bak" -o -name "*.backup" -o -name "*~" 2>/dev/null | wc -l)
    log "Backup cleanup completed"
    log "Remaining backup files: $remaining_files"
    
    if command -v df >/dev/null 2>&1; then
        disk_usage=$(df -h "$BACKUP_DIR" | tail -1 | awk '{print $5}')
        log "Backup directory disk usage: $disk_usage"
    fi
fi