#!/bin/bash

# ========================================
# 🍽️ Meal Tracker Pro - Deploy Script
# ========================================
# Deploys the full Meal Tracker application stack
# and verifies all services are accessible

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
DEPLOY_ENV="${DEPLOY_ENV:-production}"
HOST_IP=$(hostname -I | awk '{print $1}')
PUBLIC_IP=""

# Ports configuration
API_GATEWAY_PORT=38080
NUTRITION_SERVICE_PORT=38081
ANALYTICS_SERVICE_PORT=38082
FRONTEND_PORT=3000
POSTGRES_PORT=35432
REDIS_PORT=36379
PROMETHEUS_PORT=9090
GRAFANA_PORT=3001

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

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get public IP
get_public_ip() {
    print_status "Detecting public IP address..."
    
    # Try multiple methods to get public IP
    PUBLIC_IP=$(curl -s ifconfig.me 2>/dev/null || curl -s icanhazip.com 2>/dev/null || curl -s ipecho.net/plain 2>/dev/null || echo "")
    
    if [ -n "$PUBLIC_IP" ]; then
        print_success "Public IP detected: $PUBLIC_IP"
    else
        print_warning "Could not detect public IP. Will test local network only."
    fi
}

# Function to check prerequisites
check_prerequisites() {
    print_header "Checking Prerequisites"
    
    local missing_deps=0
    
    if ! command_exists docker; then
        print_error "Docker is not installed"
        missing_deps=1
    else
        print_success "Docker is installed"
    fi
    
    if ! command_exists docker-compose; then
        print_error "Docker Compose is not installed"
        missing_deps=1
    else
        print_success "Docker Compose is installed"
    fi
    
    if ! command_exists curl; then
        print_error "curl is not installed"
        missing_deps=1
    else
        print_success "curl is installed"
    fi
    
    if ! command_exists nc; then
        print_warning "netcat (nc) is not installed - port checks may be limited"
    else
        print_success "netcat is installed"
    fi
    
    # Check if Docker daemon is running
    if ! docker info >/dev/null 2>&1; then
        print_error "Docker daemon is not running"
        missing_deps=1
    else
        print_success "Docker daemon is running"
    fi
    
    if [ $missing_deps -eq 1 ]; then
        print_error "Please install missing dependencies before continuing"
        exit 1
    fi
}

# Function to create environment file
create_env_file() {
    print_header "Setting Up Environment"
    
    if [ ! -f .env ]; then
        print_status "Creating .env file..."
        cat > .env << EOF
# Database Configuration
POSTGRES_USER=mealtracker
POSTGRES_PASSWORD=mealtracker_secure_$(date +%s)
POSTGRES_DB=mealtracker
DATABASE_URL=postgres://mealtracker:mealtracker_secure_$(date +%s)@postgres:5432/mealtracker

# Redis Configuration
REDIS_URL=redis://redis:6379

# Service URLs
API_GATEWAY_URL=http://api-gateway:8080
NUTRITION_SERVICE_URL=http://nutrition-service:8081
ANALYTICS_SERVICE_URL=http://analytics-service:8082

# Frontend Configuration
REACT_APP_API_URL=http://163.74.81.175:${API_GATEWAY_PORT}/api

# Monitoring Configuration
PROMETHEUS_SCRAPE_INTERVAL=5s
GRAFANA_USER=admin
GRAFANA_PASSWORD=mealtracker_admin_$(date +%s)

# JWT Secret for Authentication
JWT_SECRET=$(openssl rand -hex 32)
JWT_EXPIRATION_HOURS=24

# External APIs (add your API keys here)
NUTRITION_API_KEYS=

# Deploy Environment
DEPLOY_ENV=${DEPLOY_ENV}
EOF
        print_success "Environment file created"
    else
        print_warning ".env file already exists, using existing configuration"
    fi
}

# Function to stop existing services
stop_services() {
    print_header "Stopping Existing Services"
    
    print_status "Stopping existing containers..."
    docker-compose down --remove-orphans 2>/dev/null || true
    
    # Kill any processes using our ports
    for port in $API_GATEWAY_PORT $NUTRITION_SERVICE_PORT $ANALYTICS_SERVICE_PORT $FRONTEND_PORT $POSTGRES_PORT $REDIS_PORT $PROMETHEUS_PORT $GRAFANA_PORT; do
        if command_exists lsof; then
            local pid=$(lsof -ti:$port 2>/dev/null || true)
            if [ -n "$pid" ]; then
                print_status "Killing process on port $port (PID: $pid)"
                kill -9 $pid 2>/dev/null || true
            fi
        fi
    done
    
    print_success "Existing services stopped"
}

# Function to build and start services
build_and_start() {
    print_header "Building and Starting Services"
    
    print_status "Building Docker images..."
    docker-compose build --no-cache
    
    print_status "Starting services..."
    docker-compose up -d
    
    print_success "Services started"
}

# Function to wait for service to be ready
wait_for_service() {
    local service_name=$1
    local url=$2
    local max_attempts=30
    local attempt=1
    
    print_status "Waiting for $service_name to be ready..."
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s "$url" >/dev/null 2>&1; then
            print_success "$service_name is ready"
            return 0
        fi
        
        echo -n "."
        sleep 2
        attempt=$((attempt + 1))
    done
    
    print_error "$service_name failed to start within expected time"
    return 1
}

# Function to check port connectivity
check_port() {
    local host=$1
    local port=$2
    local service_name=$3
    
    if command_exists nc; then
        if nc -z "$host" "$port" 2>/dev/null; then
            print_success "$service_name ($host:$port) is accessible"
            return 0
        else
            print_error "$service_name ($host:$port) is not accessible"
            return 1
        fi
    else
        # Fallback to curl/telnet if nc is not available
        if timeout 5 bash -c "</dev/tcp/$host/$port" 2>/dev/null; then
            print_success "$service_name ($host:$port) is accessible"
            return 0
        else
            print_error "$service_name ($host:$port) is not accessible"
            return 1
        fi
    fi
}

# Function to verify services
verify_services() {
    print_header "Verifying Service Health"
    
    # Wait for database to be ready
    print_status "Checking database connectivity..."
    for i in {1..30}; do
        if docker exec mealtracker_postgres pg_isready -U mealtracker >/dev/null 2>&1; then
            print_success "PostgreSQL is ready"
            break
        fi
        if [ $i -eq 30 ]; then
            print_error "PostgreSQL failed to start"
            return 1
        fi
        sleep 2
    done
    
    # Wait for Redis to be ready
    print_status "Checking Redis connectivity..."
    for i in {1..30}; do
        if docker exec mealtracker_redis redis-cli ping >/dev/null 2>&1; then
            print_success "Redis is ready"
            break
        fi
        if [ $i -eq 30 ]; then
            print_error "Redis failed to start"
            return 1
        fi
        sleep 2
    done
    
    # Check individual services
    sleep 10  # Give services time to start
    
    local failed_services=0
    
    # Check API Gateway
    if wait_for_service "API Gateway" "http://localhost:$API_GATEWAY_PORT/health"; then
        print_success "API Gateway health check passed"
    else
        print_error "API Gateway health check failed"
        failed_services=$((failed_services + 1))
    fi
    
    # Check Nutrition Service
    if wait_for_service "Nutrition Service" "http://localhost:$NUTRITION_SERVICE_PORT/health"; then
        print_success "Nutrition Service health check passed"
    else
        print_error "Nutrition Service health check failed"
        failed_services=$((failed_services + 1))
    fi
    
    # Check Analytics Service
    if wait_for_service "Analytics Service" "http://localhost:$ANALYTICS_SERVICE_PORT/health"; then
        print_success "Analytics Service health check passed"
    else
        print_error "Analytics Service health check failed"
        failed_services=$((failed_services + 1))
    fi
    
    # Check Frontend
    if wait_for_service "Frontend" "http://localhost:$FRONTEND_PORT"; then
        print_success "Frontend is accessible"
    else
        print_error "Frontend is not accessible"
        failed_services=$((failed_services + 1))
    fi
    
    # Check Monitoring
    if wait_for_service "Prometheus" "http://localhost:$PROMETHEUS_PORT"; then
        print_success "Prometheus is accessible"
    else
        print_warning "Prometheus is not accessible"
    fi
    
    if wait_for_service "Grafana" "http://localhost:$GRAFANA_PORT"; then
        print_success "Grafana is accessible"
    else
        print_warning "Grafana is not accessible"
    fi
    
    return $failed_services
}

# Function to test network connectivity
test_connectivity() {
    print_header "Testing Network Connectivity"
    
    local connectivity_issues=0
    
    print_status "Testing local network connectivity..."
    
    # Test local (private network) connectivity
    check_port "$HOST_IP" "$API_GATEWAY_PORT" "API Gateway (Local)" || connectivity_issues=$((connectivity_issues + 1))
    check_port "$HOST_IP" "$FRONTEND_PORT" "Frontend (Local)" || connectivity_issues=$((connectivity_issues + 1))
    check_port "$HOST_IP" "$PROMETHEUS_PORT" "Prometheus (Local)" || connectivity_issues=$((connectivity_issues + 1))
    check_port "$HOST_IP" "$GRAFANA_PORT" "Grafana (Local)" || connectivity_issues=$((connectivity_issues + 1))
    
    # Test public connectivity if public IP is available
    if [ -n "$PUBLIC_IP" ] && [ "$PUBLIC_IP" != "$HOST_IP" ]; then
        print_status "Testing public network connectivity..."
        check_port "$PUBLIC_IP" "$API_GATEWAY_PORT" "API Gateway (Public)" || print_warning "Public access to API Gateway may be blocked by firewall"
        check_port "$PUBLIC_IP" "$FRONTEND_PORT" "Frontend (Public)" || print_warning "Public access to Frontend may be blocked by firewall"
    fi
    
    return $connectivity_issues
}

# Function to show deployment summary
show_deployment_summary() {
    print_header "Deployment Summary"
    
    echo -e "${GREEN}🎉 Meal Tracker Pro has been deployed successfully!${NC}\n"
    
    echo -e "${CYAN}Access URLs:${NC}"
    echo -e "Frontend (Main App):     http://$HOST_IP:$FRONTEND_PORT"
    echo -e "API Gateway:            http://$HOST_IP:$API_GATEWAY_PORT"
    echo -e "Nutrition Service:      http://$HOST_IP:$NUTRITION_SERVICE_PORT"
    echo -e "Analytics Service:      http://$HOST_IP:$ANALYTICS_SERVICE_PORT"
    echo -e "Prometheus Monitoring:  http://$HOST_IP:$PROMETHEUS_PORT"
    echo -e "Grafana Dashboard:      http://$HOST_IP:$GRAFANA_PORT"
    
    if [ -n "$PUBLIC_IP" ] && [ "$PUBLIC_IP" != "$HOST_IP" ]; then
        echo -e "\n${CYAN}Public Access URLs (if firewall allows):${NC}"
        echo -e "Frontend (Main App):     http://$PUBLIC_IP:$FRONTEND_PORT"
        echo -e "API Gateway:            http://$PUBLIC_IP:$API_GATEWAY_PORT"
    fi
    
    echo -e "\n${CYAN}Database Access:${NC}"
    echo -e "PostgreSQL:             $HOST_IP:$POSTGRES_PORT"
    echo -e "Redis:                  $HOST_IP:$REDIS_PORT"
    
    echo -e "\n${CYAN}Monitoring Credentials:${NC}"
    local grafana_pass=$(grep GRAFANA_PASSWORD .env | cut -d'=' -f2)
    echo -e "Grafana Username:       admin"
    echo -e "Grafana Password:       $grafana_pass"
    
    echo -e "\n${YELLOW}Next Steps:${NC}"
    echo -e "1. Open http://$HOST_IP:$FRONTEND_PORT in your browser"
    echo -e "2. Configure your firewall to allow external access if needed"
    echo -e "3. Set up SSL/TLS certificates for production use"
    echo -e "4. Configure backup schedules for the database"
    
    echo -e "\n${CYAN}Management Commands:${NC}"
    echo -e "View logs:              docker-compose logs -f"
    echo -e "Stop services:          docker-compose down"
    echo -e "Restart services:       docker-compose restart"
    echo -e "Full cleanup:           ./cleanup.sh"
}

# Function to show firewall configuration help
show_firewall_help() {
    print_header "Firewall Configuration Help"
    
    echo -e "${YELLOW}If you need external access, configure your firewall:${NC}\n"
    
    echo -e "${CYAN}For UFW (Ubuntu):${NC}"
    echo -e "sudo ufw allow $FRONTEND_PORT"
    echo -e "sudo ufw allow $API_GATEWAY_PORT"
    echo -e "sudo ufw allow $PROMETHEUS_PORT"
    echo -e "sudo ufw allow $GRAFANA_PORT"
    
    echo -e "\n${CYAN}For iptables:${NC}"
    echo -e "sudo iptables -A INPUT -p tcp --dport $FRONTEND_PORT -j ACCEPT"
    echo -e "sudo iptables -A INPUT -p tcp --dport $API_GATEWAY_PORT -j ACCEPT"
    echo -e "sudo iptables -A INPUT -p tcp --dport $PROMETHEUS_PORT -j ACCEPT"
    echo -e "sudo iptables -A INPUT -p tcp --dport $GRAFANA_PORT -j ACCEPT"
    
    echo -e "\n${CYAN}For firewalld (CentOS/RHEL):${NC}"
    echo -e "sudo firewall-cmd --permanent --add-port=$FRONTEND_PORT/tcp"
    echo -e "sudo firewall-cmd --permanent --add-port=$API_GATEWAY_PORT/tcp"
    echo -e "sudo firewall-cmd --permanent --add-port=$PROMETHEUS_PORT/tcp"
    echo -e "sudo firewall-cmd --permanent --add-port=$GRAFANA_PORT/tcp"
    echo -e "sudo firewall-cmd --reload"
}

# Main deployment function
main() {
    print_header "🍽️ Meal Tracker Pro - Deployment Script"
    
    echo -e "${BLUE}Starting deployment at $(date)${NC}"
    echo -e "${BLUE}Host IP: $HOST_IP${NC}\n"
    
    # Check prerequisites
    check_prerequisites
    
    # Get public IP
    get_public_ip
    
    # Create environment file
    create_env_file
    
    # Stop existing services
    stop_services
    
    # Build and start services
    build_and_start
    
    # Verify services are working
    if ! verify_services; then
        print_error "Some services failed to start properly"
        print_status "Checking container logs..."
        docker-compose logs --tail=20
        exit 1
    fi
    
    # Test connectivity
    test_connectivity
    
    # Show deployment summary
    show_deployment_summary
    
    # Show firewall help
    show_firewall_help
    
    print_success "Deployment completed successfully!"
    echo -e "\n${GREEN}🚀 Your Meal Tracker Pro application is now running!${NC}"
}

# Trap to handle script interruption
trap 'print_error "Deployment interrupted by user"; exit 1' INT TERM

# Run main function
main "$@"
