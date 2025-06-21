#!/bin/bash

# ========================================
# 🏥 Meal Tracker Pro - Health Check Script
# ========================================
# Comprehensive health check for all services

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
HOST_IP=$(hostname -I | awk '{print $1}')
SERVICES=(
    "Frontend:3000:/health"
    "API Gateway:38080:/health"
    "Nutrition Service:38081:/health"
    "Analytics Service:38082:/health"
    "Prometheus:9090:/metrics"
    "Grafana:3001:/api/health"
)

CONTAINERS=(
    "mealtracker_frontend"
    "mealtracker_api_gateway"
    "mealtracker_nutrition_service"
    "mealtracker_analytics_service"
    "mealtracker_postgres"
    "mealtracker_redis"
    "mealtracker_prometheus"
    "mealtracker_grafana"
)

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[⚠]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

print_header() {
    echo -e "\n${CYAN}========================================${NC}"
    echo -e "${CYAN} $1 ${NC}"
    echo -e "${CYAN}========================================${NC}\n"
}

# Function to check if a service is responsive
check_service_health() {
    local service_name=$1
    local port=$2
    local endpoint=$3
    local url="http://$HOST_IP:$port$endpoint"
    
    if curl -s --max-time 5 "$url" >/dev/null 2>&1; then
        print_success "$service_name is healthy"
        return 0
    else
        print_error "$service_name is not responding"
        return 1
    fi
}

# Function to check container status
check_container_status() {
    local container_name=$1
    
    if docker ps --format "table {{.Names}}\t{{.Status}}" | grep -q "$container_name.*Up"; then
        local status=$(docker ps --format "table {{.Names}}\t{{.Status}}" | grep "$container_name" | awk '{print $2,$3,$4}')
        print_success "$container_name: $status"
        return 0
    elif docker ps -a --format "table {{.Names}}\t{{.Status}}" | grep -q "$container_name"; then
        local status=$(docker ps -a --format "table {{.Names}}\t{{.Status}}" | grep "$container_name" | awk '{print $2,$3,$4}')
        print_error "$container_name: $status"
        return 1
    else
        print_error "$container_name: Not found"
        return 1
    fi
}

# Function to check database connectivity
check_database() {
    print_status "Checking database connectivity..."
    
    if docker exec mealtracker_postgres pg_isready -U mealtracker >/dev/null 2>&1; then
        print_success "PostgreSQL is accessible"
        
        # Check if database exists and has tables
        local table_count=$(docker exec mealtracker_postgres psql -U mealtracker -d mealtracker -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" 2>/dev/null | tr -d ' ' || echo "0")
        if [ "$table_count" -gt 0 ]; then
            print_success "Database has $table_count tables"
        else
            print_warning "Database exists but has no tables (migrations may be needed)"
        fi
        return 0
    else
        print_error "PostgreSQL is not accessible"
        return 1
    fi
}

# Function to check Redis connectivity
check_redis() {
    print_status "Checking Redis connectivity..."
    
    if docker exec mealtracker_redis redis-cli ping >/dev/null 2>&1; then
        print_success "Redis is accessible"
        
        # Check Redis info
        local redis_info=$(docker exec mealtracker_redis redis-cli info server | grep redis_version | cut -d: -f2 | tr -d '\r')
        print_success "Redis version: $redis_info"
        return 0
    else
        print_error "Redis is not accessible"
        return 1
    fi
}

# Function to check disk space
check_disk_space() {
    print_status "Checking disk space..."
    
    local disk_usage=$(df -h / | awk 'NR==2 {print $5}' | sed 's/%//')
    
    if [ "$disk_usage" -lt 80 ]; then
        print_success "Disk usage: ${disk_usage}% (healthy)"
    elif [ "$disk_usage" -lt 90 ]; then
        print_warning "Disk usage: ${disk_usage}% (getting high)"
    else
        print_error "Disk usage: ${disk_usage}% (critically high)"
    fi
}

# Function to check memory usage
check_memory() {
    print_status "Checking memory usage..."
    
    local memory_info=$(free -m | awk 'NR==2{printf "%.1f%%", $3*100/$2 }')
    print_success "Memory usage: $memory_info"
}

# Function to check Docker resources
check_docker_resources() {
    print_status "Checking Docker resource usage..."
    
    echo -e "${CYAN}Docker System Info:${NC}"
    docker system df --format "table {{.Type}}\t{{.Total}}\t{{.Active}}\t{{.Size}}\t{{.Reclaimable}}"
}

# Function to check network connectivity
check_network() {
    print_status "Checking network connectivity..."
    
    # Check if ports are open
    local open_ports=0
    local total_ports=0
    
    for service in "${SERVICES[@]}"; do
        local port=$(echo "$service" | cut -d: -f2)
        total_ports=$((total_ports + 1))
        
        if nc -z "$HOST_IP" "$port" 2>/dev/null; then
            open_ports=$((open_ports + 1))
        fi
    done
    
    print_success "$open_ports/$total_ports ports are accessible"
}

# Function to generate health report
generate_health_report() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local report_file="health_report_$(date '+%Y%m%d_%H%M%S').txt"
    
    {
        echo "Meal Tracker Pro - Health Report"
        echo "Generated: $timestamp"
        echo "Host: $HOST_IP"
        echo "=================================="
        echo ""
        
        echo "Container Status:"
        docker ps -a --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep mealtracker
        echo ""
        
        echo "Service Health:"
        for service in "${SERVICES[@]}"; do
            local name=$(echo "$service" | cut -d: -f1)
            local port=$(echo "$service" | cut -d: -f2)
            local endpoint=$(echo "$service" | cut -d: -f3)
            local url="http://$HOST_IP:$port$endpoint"
            
            if curl -s --max-time 5 "$url" >/dev/null 2>&1; then
                echo "$name: HEALTHY"
            else
                echo "$name: UNHEALTHY"
            fi
        done
        echo ""
        
        echo "System Resources:"
        df -h /
        echo ""
        free -h
        echo ""
        
        echo "Docker Resources:"
        docker system df
        
    } > "$report_file"
    
    print_success "Health report saved to: $report_file"
}

# Main health check function
main() {
    print_header "🏥 Meal Tracker Pro - Health Check"
    
    echo -e "${BLUE}Health check started at $(date)${NC}"
    echo -e "${BLUE}Host IP: $HOST_IP${NC}\n"
    
    local failed_checks=0
    local total_checks=0
    
    # Check container status
    print_header "Container Status"
    for container in "${CONTAINERS[@]}"; do
        total_checks=$((total_checks + 1))
        if ! check_container_status "$container"; then
            failed_checks=$((failed_checks + 1))
        fi
    done
    
    # Check service health
    print_header "Service Health"
    for service in "${SERVICES[@]}"; do
        local name=$(echo "$service" | cut -d: -f1)
        local port=$(echo "$service" | cut -d: -f2)
        local endpoint=$(echo "$service" | cut -d: -f3)
        
        total_checks=$((total_checks + 1))
        if ! check_service_health "$name" "$port" "$endpoint"; then
            failed_checks=$((failed_checks + 1))
        fi
    done
    
    # Check database
    print_header "Database Health"
    total_checks=$((total_checks + 1))
    if ! check_database; then
        failed_checks=$((failed_checks + 1))
    fi
    
    # Check Redis
    print_header "Cache Health"
    total_checks=$((total_checks + 1))
    if ! check_redis; then
        failed_checks=$((failed_checks + 1))
    fi
    
    # System checks
    print_header "System Health"
    check_disk_space
    check_memory
    check_network
    
    # Docker resources
    print_header "Docker Resources"
    check_docker_resources
    
    # Summary
    print_header "Health Check Summary"
    
    local success_rate=$(( (total_checks - failed_checks) * 100 / total_checks ))
    
    if [ $failed_checks -eq 0 ]; then
        print_success "All checks passed! ($total_checks/$total_checks) - System is healthy"
    elif [ $failed_checks -lt 3 ]; then
        print_warning "Some issues detected ($((total_checks - failed_checks))/$total_checks passed) - System is partially healthy"
    else
        print_error "Multiple issues detected ($((total_checks - failed_checks))/$total_checks passed) - System needs attention"
    fi
    
    echo -e "\n${CYAN}Success Rate: ${success_rate}%${NC}"
    
    # Generate report if requested
    if [[ "$1" == "--report" ]]; then
        generate_health_report
    fi
    
    # Quick fix suggestions
    if [ $failed_checks -gt 0 ]; then
        echo -e "\n${YELLOW}Quick Fix Suggestions:${NC}"
        echo -e "1. Restart failed services: docker-compose restart"
        echo -e "2. Check logs: docker-compose logs -f"
        echo -e "3. Full restart: docker-compose down && docker-compose up -d"
        echo -e "4. Run cleanup: ./cleanup.sh --quick && ./deploy.sh"
    fi
    
    return $failed_checks
}

# Show help
show_help() {
    echo -e "${CYAN}Meal Tracker Pro - Health Check Script${NC}\n"
    echo -e "${BLUE}Usage:${NC}"
    echo -e "  $0 [OPTION]\n"
    echo -e "${BLUE}Options:${NC}"
    echo -e "  -h, --help     Show this help message"
    echo -e "  --report       Generate detailed health report"
    echo -e "  --quick        Quick health check (containers only)"
    echo -e "  --services     Check services only"
    echo -e "  --system       Check system resources only"
}

# Quick health check
quick_check() {
    print_header "Quick Health Check"
    
    local failed=0
    for container in "${CONTAINERS[@]}"; do
        if ! check_container_status "$container"; then
            failed=$((failed + 1))
        fi
    done
    
    if [ $failed -eq 0 ]; then
        print_success "All containers are running"
    else
        print_error "$failed containers have issues"
    fi
    
    return $failed
}

# Parse command line arguments
case "${1:-}" in
    -h|--help)
        show_help
        exit 0
        ;;
    --quick)
        quick_check
        exit $?
        ;;
    --report)
        main --report
        exit $?
        ;;
    --services)
        print_header "Service Health Check"
        failed=0
        for service in "${SERVICES[@]}"; do
            local name=$(echo "$service" | cut -d: -f1)
            local port=$(echo "$service" | cut -d: -f2)
            local endpoint=$(echo "$service" | cut -d: -f3)
            
            if ! check_service_health "$name" "$port" "$endpoint"; then
                failed=$((failed + 1))
            fi
        done
        exit $failed
        ;;
    --system)
        print_header "System Health Check"
        check_disk_space
        check_memory
        check_docker_resources
        exit 0
        ;;
    "")
        main
        exit $?
        ;;
    *)
        print_error "Unknown option: $1"
        show_help
        exit 1
        ;;
esac
