#!/bin/bash

# ========================================
# 🧹 Meal Tracker Pro - Cleanup Script
# ========================================
# Complete cleanup of all Meal Tracker resources
# Use with caution - this will delete all data!

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="meal-tracker-pro"
BACKUP_DIR="./backups/$(date +%Y%m%d_%H%M%S)"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "\n${CYAN}========================================${NC}"
    echo -e "${CYAN} $1 ${NC}"
    echo -e "${CYAN}========================================${NC}\n"
}

# Function to ask for confirmation
confirm_action() {
    local message=$1
    local default=${2:-"n"}
    
    if [ "$default" = "y" ]; then
        local prompt="$message [Y/n]: "
    else
        local prompt="$message [y/N]: "
    fi
    
    while true; do
        read -p "$prompt" response
        case $response in
            [Yy]* ) return 0;;
            [Nn]* ) return 1;;
            "" ) 
                if [ "$default" = "y" ]; then
                    return 0
                else
                    return 1
                fi
                ;;
            * ) echo "Please answer yes or no.";;
        esac
    done
}

# Function to backup database
backup_database() {
    print_header "Database Backup"
    
    if docker ps | grep -q "mealtracker_postgres"; then
        print_status "Creating database backup..."
        
        mkdir -p "$BACKUP_DIR"
        
        # Backup database
        docker exec mealtracker_postgres pg_dump -U mealtracker mealtracker > "$BACKUP_DIR/database_backup.sql" 2>/dev/null || {
            print_warning "Database backup failed - container may not be running properly"
        }
        
        # Backup environment file
        if [ -f .env ]; then
            cp .env "$BACKUP_DIR/env_backup"
            print_success "Environment file backed up"
        fi
        
        print_success "Database backup created at: $BACKUP_DIR"
    else
        print_warning "PostgreSQL container not running - skipping database backup"
    fi
}

# Function to stop all services
stop_services() {
    print_header "Stopping Services"
    
    print_status "Stopping Docker Compose services..."
    docker-compose down --remove-orphans 2>/dev/null || {
        print_warning "docker-compose down failed - continuing with manual cleanup"
    }
    
    # Stop containers by name pattern
    local containers=$(docker ps -a -q --filter "name=mealtracker_" 2>/dev/null || true)
    if [ -n "$containers" ]; then
        print_status "Stopping remaining containers..."
        docker stop $containers 2>/dev/null || true
        print_success "Containers stopped"
    fi
}

# Function to remove containers
remove_containers() {
    print_header "Removing Containers"
    
    # Remove containers by name pattern
    local containers=$(docker ps -a -q --filter "name=mealtracker_" 2>/dev/null || true)
    if [ -n "$containers" ]; then
        print_status "Removing containers..."
        docker rm -f $containers 2>/dev/null || true
        print_success "Containers removed"
    else
        print_status "No containers to remove"
    fi
    
    # Clean up any orphaned containers
    docker container prune -f >/dev/null 2>&1 || true
}

# Function to remove images
remove_images() {
    print_header "Removing Images"
    
    # Remove project-specific images
    local images=$(docker images -q --filter "reference=*meal*" --filter "reference=*nutrition*" --filter "reference=*analytics*" 2>/dev/null || true)
    if [ -n "$images" ]; then
        print_status "Removing project images..."
        docker rmi -f $images 2>/dev/null || true
        print_success "Project images removed"
    fi
    
    # Remove dangling images
    print_status "Removing dangling images..."
    docker image prune -f >/dev/null 2>&1 || true
    print_success "Dangling images cleaned up"
}

# Function to remove volumes
remove_volumes() {
    print_header "Removing Volumes"
    
    # Remove named volumes
    local volumes=$(docker volume ls -q --filter "name=meal" 2>/dev/null || true)
    if [ -n "$volumes" ]; then
        print_status "Removing project volumes..."
        docker volume rm -f $volumes 2>/dev/null || true
        print_success "Project volumes removed"
    fi
    
    # Remove anonymous volumes
    print_status "Removing unused volumes..."
    docker volume prune -f >/dev/null 2>&1 || true
    print_success "Unused volumes cleaned up"
}

# Function to remove networks
remove_networks() {
    print_header "Removing Networks"
    
    # Remove project networks
    local networks=$(docker network ls -q --filter "name=meal" 2>/dev/null || true)
    if [ -n "$networks" ]; then
        print_status "Removing project networks..."
        docker network rm $networks 2>/dev/null || true
        print_success "Project networks removed"
    fi
    
    # Clean up unused networks
    print_status "Removing unused networks..."
    docker network prune -f >/dev/null 2>&1 || true
    print_success "Unused networks cleaned up"
}

# Function to clean up ports
cleanup_ports() {
    print_header "Cleaning Up Ports"
    
    local ports=(38080 38081 38082 3000 35432 36379 9090 3001)
    
    if command -v lsof >/dev/null 2>&1; then
        for port in "${ports[@]}"; do
            local pid=$(lsof -ti:$port 2>/dev/null || true)
            if [ -n "$pid" ]; then
                print_status "Killing process on port $port (PID: $pid)"
                kill -9 $pid 2>/dev/null || true
            fi
        done
        print_success "Port cleanup completed"
    else
        print_warning "lsof not available - unable to clean up ports automatically"
    fi
}

# Function to remove files
remove_files() {
    print_header "File Cleanup"
    
    if confirm_action "Remove environment file (.env)?"; then
        if [ -f .env ]; then
            rm .env
            print_success "Environment file removed"
        fi
    fi
    
    if confirm_action "Remove Docker Compose override file?"; then
        if [ -f docker-compose.override.yml ]; then
            rm docker-compose.override.yml
            print_success "Docker Compose override file removed"
        fi
    fi
    
    if confirm_action "Remove log files?"; then
        find . -name "*.log" -type f -delete 2>/dev/null || true
        print_success "Log files removed"
    fi
    
    # Clean up temporary files
    print_status "Removing temporary files..."
    find . -name "*.tmp" -type f -delete 2>/dev/null || true
    find . -name ".DS_Store" -type f -delete 2>/dev/null || true
    print_success "Temporary files cleaned up"
}

# Function to perform Docker system cleanup
docker_system_cleanup() {
    print_header "Docker System Cleanup"
    
    if confirm_action "Perform aggressive Docker system cleanup? (This will remove ALL unused Docker resources)"; then
        print_status "Performing system-wide Docker cleanup..."
        docker system prune -af --volumes >/dev/null 2>&1 || true
        print_success "Docker system cleanup completed"
    else
        print_status "Performing conservative Docker cleanup..."
        docker system prune -f >/dev/null 2>&1 || true
        print_success "Conservative cleanup completed"
    fi
}

# Function to show cleanup summary
show_cleanup_summary() {
    print_header "Cleanup Summary"
    
    echo -e "${GREEN}🧹 Cleanup completed successfully!${NC}\n"
    
    # Show Docker resource usage after cleanup
    echo -e "${CYAN}Docker Resource Usage After Cleanup:${NC}"
    docker system df 2>/dev/null || true
    
    echo -e "\n${CYAN}Remaining Project Resources:${NC}"
    
    # Check for remaining containers
    local remaining_containers=$(docker ps -a --filter "name=mealtracker_" --format "table {{.Names}}\t{{.Status}}" 2>/dev/null | grep -v "NAMES" || true)
    if [ -n "$remaining_containers" ]; then
        echo -e "${YELLOW}Remaining containers:${NC}"
        echo "$remaining_containers"
    else
        echo -e "${GREEN}✓ No remaining containers${NC}"
    fi
    
    # Check for remaining images
    local remaining_images=$(docker images --filter "reference=*meal*" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}" 2>/dev/null | grep -v "REPOSITORY" || true)
    if [ -n "$remaining_images" ]; then
        echo -e "${YELLOW}Remaining images:${NC}"
        echo "$remaining_images"
    else
        echo -e "${GREEN}✓ No remaining images${NC}"
    fi
    
    # Check for remaining volumes
    local remaining_volumes=$(docker volume ls --filter "name=meal" --format "table {{.Name}}\t{{.Driver}}" 2>/dev/null | grep -v "VOLUME NAME" || true)
    if [ -n "$remaining_volumes" ]; then
        echo -e "${YELLOW}Remaining volumes:${NC}"
        echo "$remaining_volumes"
    else
        echo -e "${GREEN}✓ No remaining volumes${NC}"
    fi
    
    # Show backup location if created
    if [ -d "$BACKUP_DIR" ]; then
        echo -e "\n${CYAN}Backup Location:${NC}"
        echo -e "$BACKUP_DIR"
        echo -e "$(ls -la "$BACKUP_DIR" 2>/dev/null || true)"
    fi
    
    echo -e "\n${CYAN}To redeploy the application:${NC}"
    echo -e "./deploy.sh"
    
    echo -e "\n${CYAN}To restore from backup:${NC}"
    echo -e "docker exec -i mealtracker_postgres psql -U mealtracker -d mealtracker < $BACKUP_DIR/database_backup.sql"
}

# Function for quick cleanup (no prompts)
quick_cleanup() {
    print_header "Quick Cleanup Mode"
    print_warning "Running in quick mode - no confirmation prompts"
    
    stop_services
    remove_containers
    remove_images
    remove_volumes
    remove_networks
    cleanup_ports
    
    # Light Docker cleanup
    docker system prune -f >/dev/null 2>&1 || true
    
    print_success "Quick cleanup completed"
}

# Function for full cleanup with prompts
full_cleanup() {
    print_header "Full Cleanup Mode"
    
    # Ask for backup first
    if confirm_action "Create database backup before cleanup?"; then
        backup_database
    fi
    
    # Cleanup steps
    stop_services
    remove_containers
    remove_images
    remove_volumes
    remove_networks
    cleanup_ports
    remove_files
    docker_system_cleanup
    
    show_cleanup_summary
}

# Function to show help
show_help() {
    echo -e "${CYAN}Meal Tracker Pro - Cleanup Script${NC}\n"
    echo -e "${BLUE}Usage:${NC}"
    echo -e "  $0 [OPTION]\n"
    echo -e "${BLUE}Options:${NC}"
    echo -e "  -h, --help     Show this help message"
    echo -e "  -q, --quick    Quick cleanup without prompts"
    echo -e "  -f, --full     Full cleanup with prompts (default)"
    echo -e "  -b, --backup   Create backup only"
    echo -e "  -s, --stop     Stop services only\n"
    echo -e "${BLUE}Examples:${NC}"
    echo -e "  $0              # Full cleanup with prompts"
    echo -e "  $0 --quick      # Quick cleanup without prompts"
    echo -e "  $0 --backup     # Create backup only"
    echo -e "  $0 --stop       # Stop services only"
}

# Main function
main() {
    local mode="full"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -q|--quick)
                mode="quick"
                shift
                ;;
            -f|--full)
                mode="full"
                shift
                ;;
            -b|--backup)
                mode="backup"
                shift
                ;;
            -s|--stop)
                mode="stop"
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    print_header "🧹 Meal Tracker Pro - Cleanup Script"
    echo -e "${BLUE}Cleanup started at $(date)${NC}\n"
    
    case $mode in
        "quick")
            quick_cleanup
            ;;
        "full")
            full_cleanup
            ;;
        "backup")
            backup_database
            ;;
        "stop")
            stop_services
            ;;
    esac
    
    print_success "Cleanup operation completed!"
}

# Trap to handle script interruption
trap 'print_error "Cleanup interrupted by user"; exit 1' INT TERM

# Check if Docker is available
if ! command -v docker >/dev/null 2>&1; then
    print_error "Docker is not installed or not in PATH"
    exit 1
fi

# Run main function
main "$@"
