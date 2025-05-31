#!/usr/bin/env pwsh

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

# Meal Prep Pro - Docker Cleanup Script
# This script helps clean up Docker containers, images, and volumes for development

Write-Host "🧹 Meal Prep Pro - Docker Cleanup Script" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan

# Function to ask for user confirmation
function Get-Confirmation {
    param([string]$message)
    $response = Read-Host "$message (y/N)"
    return ($response -eq "y" -or $response -eq "Y" -or $response -eq "yes")
}

# Stop all running containers
if (Get-Confirmation "Stop all Meal Prep containers?") {
    Write-Host "Stopping containers..." -ForegroundColor Yellow
    docker-compose down
}

# Remove containers
if (Get-Confirmation "Remove all Meal Prep containers?") {
    Write-Host "Removing containers..." -ForegroundColor Yellow
    docker-compose rm -f
}

# Remove images
if (Get-Confirmation "Remove Meal Prep images?") {
    Write-Host "Removing images..." -ForegroundColor Yellow
    docker images | Select-String "mealprep|meal-prep" | ForEach-Object {
        $imageName = ($_ -split '\s+')[0] + ":" + ($_ -split '\s+')[1]
        docker rmi $imageName -f
    }
}

# Remove volumes (WARNING: This will delete all data!)
if (Get-Confirmation "🚨 DANGER: Remove all volumes? This will DELETE ALL DATA!") {
    Write-Host "Removing volumes..." -ForegroundColor Red
    docker volume rm mealprep_postgres_data mealprep_redis_data mealprep_prometheus_data mealprep_grafana_data mealprep_ai_models 2>$null
}

# Clean up Docker system
if (Get-Confirmation "Clean up Docker system (remove unused containers, networks, images)?") {
    Write-Host "Cleaning up Docker system..." -ForegroundColor Yellow
    docker system prune -f
}

# Optional: Clean up everything including volumes
if (Get-Confirmation "🚨 NUCLEAR OPTION: Remove everything including all volumes?") {
    Write-Host "Nuclear cleanup..." -ForegroundColor Red
    docker system prune -a --volumes -f
}

Write-Host "✅ Cleanup completed!" -ForegroundColor Green
Write-Host ""
Write-Host "To rebuild and start fresh:" -ForegroundColor Cyan
Write-Host "docker-compose up --build -d" -ForegroundColor White
