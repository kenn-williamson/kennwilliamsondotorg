#!/bin/bash

# Log monitoring and management script for Docker containers
# Usage: ./scripts/log-monitor.sh [command] [options]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
SERVICE=""
LINES=100
FOLLOW=false
SINCE="1h"

# Function to show usage
show_usage() {
    echo "Usage: $0 [command] [options]"
    echo ""
    echo "Commands:"
    echo "  status          Show log status for all services"
    echo "  tail [service]  Tail logs for a specific service"
    echo "  size           Show log file sizes"
    echo "  rotate         Force log rotation"
    echo "  clean          Clean old log files"
    echo "  monitor        Monitor logs in real-time"
    echo ""
    echo "Options:"
    echo "  -n, --lines N    Number of lines to show (default: 100)"
    echo "  -f, --follow     Follow logs in real-time"
    echo "  -s, --since T    Show logs since time (default: 1h)"
    echo "  -h, --help       Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 status"
    echo "  $0 tail backend"
    echo "  $0 monitor -f"
    echo "  $0 size"
}

# Function to check if Docker is running
check_docker() {
    if ! docker info >/dev/null 2>&1; then
        echo -e "${RED}❌ Docker is not running or not accessible${NC}"
        exit 1
    fi
}

# Function to get container status
get_container_status() {
    local service=$1
    local status=$(docker inspect --format='{{.State.Status}}' "kennwilliamsondotorg-${service}-1" 2>/dev/null || echo "not_found")
    echo "$status"
}

# Function to show log status for all services
show_log_status() {
    echo -e "${BLUE}📊 Log Status for All Services${NC}"
    echo "=================================="
    
    local services=("nginx" "frontend" "backend" "postgres" "redis")
    
    for service in "${services[@]}"; do
        local status=$(get_container_status "$service")
        local log_size=""
        
        if [ "$status" = "running" ]; then
            # Get log size from Docker
            log_size=$(docker logs "kennwilliamsondotorg-${service}-1" --since=1h 2>/dev/null | wc -l)
            log_size="${log_size} lines (1h)"
            echo -e "${GREEN}✅ ${service}${NC} - Running - ${log_size}"
        elif [ "$status" = "not_found" ]; then
            echo -e "${YELLOW}⚠️  ${service}${NC} - Container not found"
        else
            echo -e "${RED}❌ ${service}${NC} - ${status}"
        fi
    done
}

# Function to show log file sizes
show_log_sizes() {
    echo -e "${BLUE}📏 Docker Log File Sizes${NC}"
    echo "=========================="
    
    # Get Docker log directory
    local docker_log_dir="/var/lib/docker/containers"
    
    if [ -d "$docker_log_dir" ]; then
        echo "Container Log Files:"
        find "$docker_log_dir" -name "*.log" -exec ls -lh {} \; 2>/dev/null | \
            awk '{print $5, $9}' | \
            sort -hr | \
            head -20
    else
        echo "Docker log directory not accessible"
    fi
    
    echo ""
    echo "Docker System Info:"
    docker system df
}

# Function to tail logs for a service
tail_logs() {
    local service=$1
    local container_name="kennwilliamsondotorg-${service}-1"
    
    if [ "$(get_container_status "$service")" != "running" ]; then
        echo -e "${RED}❌ Container ${container_name} is not running${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}📋 Tailing logs for ${service}${NC}"
    echo "Container: ${container_name}"
    echo "Lines: ${LINES}"
    echo "Follow: ${FOLLOW}"
    echo "=========================="
    
    if [ "$FOLLOW" = true ]; then
        docker logs -f --tail="$LINES" "$container_name"
    else
        docker logs --tail="$LINES" --since="$SINCE" "$container_name"
    fi
}

# Function to force log rotation
force_log_rotation() {
    echo -e "${BLUE}🔄 Forcing log rotation${NC}"
    echo "========================="
    
    # Restart containers to trigger log rotation
    local services=("nginx" "frontend" "backend" "redis")
    
    for service in "${services[@]}"; do
        if [ "$(get_container_status "$service")" = "running" ]; then
            echo -e "${YELLOW}🔄 Restarting ${service}...${NC}"
            docker restart "kennwilliamsondotorg-${service}-1" >/dev/null
            sleep 2
        fi
    done
    
    echo -e "${GREEN}✅ Log rotation completed${NC}"
}

# Function to clean old logs
clean_logs() {
    echo -e "${BLUE}🧹 Cleaning old logs${NC}"
    echo "===================="
    
    # Clean Docker system
    echo "Cleaning Docker system..."
    docker system prune -f
    
    # Clean unused volumes
    echo "Cleaning unused volumes..."
    docker volume prune -f
    
    echo -e "${GREEN}✅ Log cleanup completed${NC}"
}

# Function to monitor logs in real-time
monitor_logs() {
    echo -e "${BLUE}👀 Monitoring logs in real-time${NC}"
    echo "================================="
    echo "Press Ctrl+C to stop"
    echo ""
    
    # Monitor all services simultaneously
    local services=("nginx" "frontend" "backend" "redis")
    
    for service in "${services[@]}"; do
        if [ "$(get_container_status "$service")" = "running" ]; then
            echo -e "${GREEN}📋 Starting monitor for ${service}${NC}"
            docker logs -f --tail=10 "kennwilliamsondotorg-${service}-1" &
        fi
    done
    
    # Wait for user interrupt
    wait
}

# Parse command line arguments
COMMAND=""
while [[ $# -gt 0 ]]; do
    case $1 in
        status|tail|size|rotate|clean|monitor)
            COMMAND="$1"
            shift
            ;;
        -n|--lines)
            LINES="$2"
            shift 2
            ;;
        -f|--follow)
            FOLLOW=true
            shift
            ;;
        -s|--since)
            SINCE="$2"
            shift 2
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            if [ -z "$COMMAND" ] || [ "$COMMAND" = "tail" ]; then
                SERVICE="$1"
            fi
            shift
            ;;
    esac
done

# Check Docker
check_docker

# Execute command
case $COMMAND in
    status)
        show_log_status
        ;;
    tail)
        if [ -z "$SERVICE" ]; then
            echo -e "${RED}❌ Service name required for tail command${NC}"
            echo "Available services: nginx, frontend, backend, postgres, redis"
            exit 1
        fi
        tail_logs "$SERVICE"
        ;;
    size)
        show_log_sizes
        ;;
    rotate)
        force_log_rotation
        ;;
    clean)
        clean_logs
        ;;
    monitor)
        monitor_logs
        ;;
    "")
        show_usage
        ;;
    *)
        echo -e "${RED}❌ Unknown command: $COMMAND${NC}"
        show_usage
        exit 1
        ;;
esac
