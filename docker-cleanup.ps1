# Docker Cleanup Script for Meal_Prep_3 Project
# This script performs a complete wipe of all Docker resources

Write-Host "Starting Docker cleanup for Meal_Prep_3 project..." -ForegroundColor Yellow

# Stop all running containers
Write-Host "Stopping all running containers..." -ForegroundColor Cyan
docker stop $(docker ps -q) 2>$null

# Remove all containers (including stopped ones)
Write-Host "Removing all containers..." -ForegroundColor Cyan
docker rm $(docker ps -aq) 2>$null

# Remove all images associated with the project
Write-Host "Removing project images..." -ForegroundColor Cyan
docker rmi $(docker images -q --filter reference="meal*") 2>$null
docker rmi $(docker images -q --filter reference="*meal*") 2>$null

# Remove all volumes
Write-Host "Removing all volumes..." -ForegroundColor Cyan
docker volume rm $(docker volume ls -q) 2>$null

# Remove all custom networks (except default ones)
Write-Host "Removing custom networks..." -ForegroundColor Cyan
docker network rm $(docker network ls -q --filter type=custom) 2>$null

# Clean up dangling images, containers, networks, and build cache
Write-Host "Performing system prune..." -ForegroundColor Cyan
docker system prune -af --volumes

# Additional cleanup for Docker Compose
Write-Host "Cleaning up Docker Compose resources..." -ForegroundColor Cyan
if (Test-Path "docker-compose.yml") {
    docker-compose down --volumes --rmi all --remove-orphans 2>$null
}
if (Test-Path "docker-compose.yaml") {
    docker-compose down --volumes --rmi all --remove-orphans 2>$null
}

Write-Host "Docker cleanup completed!" -ForegroundColor Green
Write-Host "Verifying cleanup..." -ForegroundColor Yellow

# Verify cleanup
Write-Host "`nRemaining containers:" -ForegroundColor Magenta
docker ps -a

Write-Host "`nRemaining images:" -ForegroundColor Magenta
docker images

Write-Host "`nRemaining volumes:" -ForegroundColor Magenta
docker volume ls

Write-Host "`nRemaining networks:" -ForegroundColor Magenta
docker network ls

Write-Host "`nDocker disk usage:" -ForegroundColor Magenta
docker system df
