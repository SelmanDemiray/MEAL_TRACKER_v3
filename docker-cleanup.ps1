# Meal Prep Pro - Docker Cleanup Script for Windows
# This script automatically cleans up all Docker resources for the Meal Prep Pro project

param(
    [switch]$Confirm,
    [switch]$Help
)

# Colors for output
$colors = @{
    Red = 'Red'
    Green = 'Green' 
    Yellow = 'Yellow'
    Cyan = 'Cyan'
}

function Write-ColorOutput {
    param([string]$Message, [string]$Color = 'White')
    Write-Host $Message -ForegroundColor $Color
}

function Show-Help {
    Write-ColorOutput "🧹 Meal Prep Pro - Docker Cleanup Script" $colors.Cyan
    Write-ColorOutput "=================================================="
    Write-ColorOutput ""
    Write-ColorOutput "Usage: .\docker-cleanup.ps1 [OPTIONS]"
    Write-ColorOutput ""
    Write-ColorOutput "Options:"
    Write-ColorOutput "  -Confirm      Prompt for confirmation before cleanup"
    Write-ColorOutput "  -Help         Show this help message"
    Write-ColorOutput ""
    Write-ColorOutput "This script will automatically:"
    Write-ColorOutput "  1. Stop all running containers"
    Write-ColorOutput "  2. Remove all project containers"
    Write-ColorOutput "  3. Remove all project images"
    Write-ColorOutput "  4. Remove all project volumes (data will be lost!)"
    Write-ColorOutput "  5. Remove all project networks"
    Write-ColorOutput "  6. Prune unused Docker resources"
    Write-ColorOutput ""
    Write-ColorOutput "⚠️  WARNING: This will delete all data in the containers!" $colors.Red
    Write-ColorOutput "🚀 By default, this script runs automatically without confirmation." $colors.Yellow
    Write-ColorOutput "💡 Use -Confirm flag if you want to be prompted before cleanup." $colors.Yellow
}

function Test-DockerRunning {
    try {
        docker info | Out-Null
        return $true
    }
    catch {
        Write-ColorOutput "❌ Docker is not running. Please start Docker and try again." $colors.Red
        exit 1
    }
}

function Stop-MealPrepContainers {
    Write-ColorOutput "🛑 Stopping all Meal Prep Pro containers..." $colors.Yellow
    
    try {
        docker-compose down --remove-orphans
        Write-ColorOutput "✅ Containers stopped successfully" $colors.Green
        return $true
    }
    catch {
        Write-ColorOutput "❌ Failed to stop containers: $_" $colors.Red
        return $false
    }
}

function Remove-MealPrepImages {
    Write-ColorOutput "🗑️  Removing Meal Prep Pro images..." $colors.Yellow
    
    try {
        $images = docker images --format "{{.Repository}}:{{.Tag}}" | Where-Object { $_ -match "(meal_tracker_v3|mealprep)" }
        
        foreach ($image in $images) {
            if ($image -ne "REPOSITORY:TAG") {
                Write-ColorOutput "Removing image: $image"
                docker rmi $image --force
            }
        }
        
        Write-ColorOutput "✅ Images cleaned up" $colors.Green
        return $true
    }
    catch {
        Write-ColorOutput "❌ Failed to remove images: $_" $colors.Red
        return $false
    }
}

function Remove-MealPrepVolumes {
    Write-ColorOutput "💾 Removing data volumes..." $colors.Yellow
    
    try {
        $volumes = docker volume ls --format "{{.Name}}" | Where-Object { $_ -match "meal_tracker_v3" }
        
        foreach ($volume in $volumes) {
            Write-ColorOutput "Removing volume: $volume"
            docker volume rm $volume --force
        }
        
        Write-ColorOutput "✅ Volumes cleaned up" $colors.Green
        return $true
    }
    catch {
        Write-ColorOutput "❌ Failed to remove volumes: $_" $colors.Red
        return $false
    }
}

function Remove-MealPrepNetworks {
    Write-ColorOutput "🌐 Removing networks..." $colors.Yellow
    
    try {
        $networks = docker network ls --format "{{.Name}}" | Where-Object { $_ -match "meal_tracker_v3" }
        
        foreach ($network in $networks) {
            Write-ColorOutput "Removing network: $network"
            docker network rm $network
        }
        
        Write-ColorOutput "✅ Networks cleaned up" $colors.Green
        return $true
    }
    catch {
        Write-ColorOutput "❌ Failed to remove networks: $_" $colors.Red
        return $false
    }
}

function Invoke-DockerSystemPrune {
    Write-ColorOutput "🧽 Pruning unused Docker resources..." $colors.Yellow
    
    try {
        docker system prune -f --volumes
        Write-ColorOutput "✅ System pruned" $colors.Green
        return $true
    }
    catch {
        Write-ColorOutput "❌ Failed to prune system: $_" $colors.Red
        return $false
    }
}

function Start-AutomaticCleanup {
    Test-DockerRunning
    
    Write-ColorOutput "🧹 Meal Prep Pro - Docker Cleanup Script" $colors.Cyan
    Write-ColorOutput "=================================================="
    Write-ColorOutput "Starting automatic cleanup of all Meal Prep Pro Docker resources..." $colors.Yellow
    Write-ColorOutput "⚠️  WARNING: This will delete all data in the containers!" $colors.Red
    Write-ColorOutput ""
    
    $success = $true
    $success = (Stop-MealPrepContainers) -and $success
    $success = (Remove-MealPrepImages) -and $success
    $success = (Remove-MealPrepVolumes) -and $success
    $success = (Remove-MealPrepNetworks) -and $success
    $success = (Invoke-DockerSystemPrune) -and $success
    
    if ($success) {
        Write-ColorOutput ""
        Write-ColorOutput "🎉 Automatic cleanup completed successfully!" $colors.Green
        Write-ColorOutput "💡 You can now run 'docker-compose up --build' to start fresh." $colors.Yellow
    } else {
        Write-ColorOutput ""
        Write-ColorOutput "⚠️  Cleanup completed with some errors. Check the output above." $colors.Yellow
    }
}

function Start-ConfirmedCleanup {
    Test-DockerRunning
    
    Write-ColorOutput "🧹 Meal Prep Pro - Docker Cleanup Script" $colors.Cyan
    Write-ColorOutput "=================================================="
    Write-ColorOutput "This will remove all Meal Prep Pro containers, images, volumes, and networks." $colors.Yellow
    $confirmation = Read-Host "Are you sure you want to continue? (y/N)"
    
    if ($confirmation -match '^[Yy]$') {
        Write-ColorOutput ""
        $success = $true
        $success = (Stop-MealPrepContainers) -and $success
        $success = (Remove-MealPrepImages) -and $success
        $success = (Remove-MealPrepVolumes) -and $success
        $success = (Remove-MealPrepNetworks) -and $success
        $success = (Invoke-DockerSystemPrune) -and $success
        
        if ($success) {
            Write-ColorOutput ""
            Write-ColorOutput "🎉 Cleanup completed successfully!" $colors.Green
            Write-ColorOutput "💡 You can now run 'docker-compose up --build' to start fresh." $colors.Yellow
        } else {
            Write-ColorOutput ""
            Write-ColorOutput "⚠️  Cleanup completed with some errors. Check the output above." $colors.Yellow
        }
    } else {
        Write-ColorOutput "🚫 Cleanup cancelled." $colors.Yellow
    }
}

# Main execution
if ($Help) {
    Show-Help
    exit 0
}

# Run with confirmation if -Confirm flag is used, otherwise run automatically
if ($Confirm) {
    Start-ConfirmedCleanup
} else {
    Start-AutomaticCleanup
}
