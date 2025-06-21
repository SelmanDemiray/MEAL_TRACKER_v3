#!/bin/bash

# ANSI color codes for better readability
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}===============================================${NC}"
echo -e "${BLUE}   Meal Tracker Docker Cleanup Script         ${NC}"
echo -e "${BLUE}===============================================${NC}"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
  echo -e "${RED}Error: Docker is not running. Please start Docker first.${NC}"
  exit 1
fi

echo -e "\n${YELLOW}Looking for project containers...${NC}"

# Define project-specific container patterns
PROJECT_PATTERNS=("meal" "nutrition-service" "api-gateway" "analytics-service" "postgres" "redis" "frontend" "mealprep")

# Initialize container count
CONTAINER_COUNT=0

# Find and stop running containers
echo -e "\n${YELLOW}Stopping containers...${NC}"
for pattern in "${PROJECT_PATTERNS[@]}"; do
  containers=$(docker ps -q --filter "name=$pattern")
  if [ -n "$containers" ]; then
    echo -e "${GREEN}Stopping containers matching pattern: ${pattern}${NC}"
    docker stop $containers
    CONTAINER_COUNT=$((CONTAINER_COUNT + $(echo "$containers" | wc -w)))
  fi
done

# Remove all containers (both stopped and running)
echo -e "\n${YELLOW}Removing containers...${NC}"
for pattern in "${PROJECT_PATTERNS[@]}"; do
  containers=$(docker ps -a -q --filter "name=$pattern")
  if [ -n "$containers" ]; then
    echo -e "${GREEN}Removing containers matching pattern: ${pattern}${NC}"
    docker rm -f $containers
  fi
  
  # Also try to find containers by image name
  containers=$(docker ps -a -q --filter "ancestor=$pattern")
  if [ -n "$containers" ]; then
    echo -e "${GREEN}Removing containers using image: ${pattern}${NC}"
    docker rm -f $containers
  fi
done

# Remove all related images
echo -e "\n${YELLOW}Removing images...${NC}"
IMAGE_COUNT=0
for pattern in "${PROJECT_PATTERNS[@]}"; do
  images=$(docker images --format "{{.ID}}" --filter "reference=*${pattern}*")
  if [ -n "$images" ]; then
    echo -e "${GREEN}Removing images matching pattern: ${pattern}${NC}"
    docker rmi -f $images
    IMAGE_COUNT=$((IMAGE_COUNT + $(echo "$images" | wc -w)))
  fi
done

# Remove dangling volumes
echo -e "\n${YELLOW}Removing unused volumes...${NC}"
docker volume prune -f

# Remove dangling images
echo -e "\n${YELLOW}Removing dangling images...${NC}"
docker image prune -f

# Show disk space before cleanup
echo -e "\n${YELLOW}Disk space before cleanup:${NC}"
df -h | grep /dev/

# Remove unused volumes
echo -e "\n${YELLOW}Cleaning unused volumes...${NC}"
docker volume prune -f

# Show disk space after cleanup
echo -e "\n${YELLOW}Disk space after cleanup:${NC}"
df -h | grep /dev/

echo -e "\n${BLUE}===============================================${NC}"
echo -e "${GREEN}Cleanup Complete!${NC}"
echo -e "${GREEN}Removed approximately:${NC}"
echo -e "${GREEN}- Containers: ${CONTAINER_COUNT}${NC}"
echo -e "${GREEN}- Images: ${IMAGE_COUNT}${NC}"
echo -e "${BLUE}===============================================${NC}"

echo -e "\n${YELLOW}Current Docker status:${NC}"
docker ps -a
echo -e "\n${YELLOW}Remaining images:${NC}"
docker images

echo -e "\n${BLUE}Remember to remove any project-specific Docker networks if needed:${NC}"
echo -e "${YELLOW}docker network prune${NC}"

# Stop all containers
echo "Stopping all containers..."
docker-compose down

# Remove all containers
echo "Removing containers..."
docker-compose rm -f

# Clean up unused volumes (optional, use with caution)
echo "Cleaning up volumes..."
docker volume prune -f

# Clean up unused networks (optional)
echo "Cleaning up networks..."
docker network prune -f

# Remove dangling images
echo "Removing dangling images..."
docker image prune -f

echo "Cleanup complete!"
echo "To restart the application, run: docker-compose up -d --build"

# Make the script executable with: chmod +x docker-cleanup.sh
