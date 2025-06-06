#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🧹 Meal Prep Pro - Docker Cleanup Script${NC}"
echo "=================================================="

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        echo -e "${RED}❌ Docker is not running. Please start Docker and try again.${NC}"
        exit 1
    fi
}

# Function to stop and remove containers
cleanup_containers() {
    echo -e "${YELLOW}🛑 Stopping all Meal Prep Pro containers...${NC}"
    
    if docker-compose down --remove-orphans; then
        echo -e "${GREEN}✅ Containers stopped successfully${NC}"
    else
        echo -e "${RED}❌ Failed to stop containers${NC}"
        return 1
    fi
}

# Function to remove images
cleanup_images() {
    echo -e "${YELLOW}🗑️  Removing Meal Prep Pro images...${NC}"
    
    # Remove project images
    docker images --format "table {{.Repository}}:{{.Tag}}" | grep -E "(meal_tracker_v3|mealprep)" | while read image; do
        if [ "$image" != "REPOSITORY:TAG" ]; then
            echo "Removing image: $image"
            docker rmi "$image" --force
        fi
    done
    
    echo -e "${GREEN}✅ Images cleaned up${NC}"
}

# Function to remove volumes
cleanup_volumes() {
    echo -e "${YELLOW}💾 Removing data volumes...${NC}"
    
    # Remove named volumes
    docker volume ls --format "{{.Name}}" | grep "meal_tracker_v3" | while read volume; do
        echo "Removing volume: $volume"
        docker volume rm "$volume" --force
    done
    
    echo -e "${GREEN}✅ Volumes cleaned up${NC}"
}

# Function to remove networks
cleanup_networks() {
    echo -e "${YELLOW}🌐 Removing networks...${NC}"
    
    # Remove project networks
    docker network ls --format "{{.Name}}" | grep "meal_tracker_v3" | while read network; do
        echo "Removing network: $network"
        docker network rm "$network"
    done
    
    echo -e "${GREEN}✅ Networks cleaned up${NC}"
}

# Function to prune unused Docker resources
docker_system_prune() {
    echo -e "${YELLOW}🧽 Pruning unused Docker resources...${NC}"
    
    docker system prune -f --volumes
    
    echo -e "${GREEN}✅ System pruned${NC}"
}

# Main cleanup function
main() {
    check_docker
    
    echo -e "${YELLOW}Starting automatic cleanup of all Meal Prep Pro Docker resources...${NC}"
    echo -e "${RED}⚠️  WARNING: This will delete all data in the containers!${NC}"
    echo ""
    
    cleanup_containers
    cleanup_images
    cleanup_volumes
    cleanup_networks
    docker_system_prune
    
    echo ""
    echo -e "${GREEN}🎉 Automatic cleanup completed successfully!${NC}"
    echo -e "${YELLOW}💡 You can now run 'docker-compose up --build' to start fresh.${NC}"
}

# Show usage if help requested
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --help, -h          Show this help message"
    echo "  --confirm           Prompt for confirmation before cleanup"
    echo ""
    echo "This script will automatically:"
    echo "  1. Stop all running containers"
    echo "  2. Remove all project containers"
    echo "  3. Remove all project images"
    echo "  4. Remove all project volumes (data will be lost!)"
    echo "  5. Remove all project networks"
    echo "  6. Prune unused Docker resources"
    echo ""
    echo "⚠️  WARNING: This will delete all data in the containers!"
    echo "🚀 By default, this script runs automatically without confirmation."
    echo "💡 Use --confirm flag if you want to be prompted before cleanup."
    exit 0
fi

# Only prompt for confirmation if --confirm flag is used
if [[ "$1" == "--confirm" ]]; then
    check_docker
    echo -e "${YELLOW}This will remove all Meal Prep Pro containers, images, volumes, and networks.${NC}"
    read -p "Are you sure you want to continue? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cleanup_containers
        cleanup_images
        cleanup_volumes
        cleanup_networks
        docker_system_prune
        
        echo ""
        echo -e "${GREEN}🎉 Cleanup completed successfully!${NC}"
        echo -e "${YELLOW}💡 You can now run 'docker-compose up --build' to start fresh.${NC}"
    else
        echo -e "${YELLOW}🚫 Cleanup cancelled.${NC}"
    fi
else
    # Default behavior: run automatically
    main
fi