#!/bin/bash

# ========================================
# 🚀 Meal Tracker Pro - Quick Start Guide
# ========================================

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_header() {
    echo -e "\n${CYAN}========================================${NC}"
    echo -e "${CYAN} $1 ${NC}"
    echo -e "${CYAN}========================================${NC}\n"
}

print_header "🍽️ Meal Tracker Pro - Quick Start Guide"

echo -e "${BLUE}Welcome to Meal Tracker Pro!${NC}\n"

echo -e "${CYAN}Available Scripts:${NC}"
echo -e "${GREEN}./deploy.sh${NC}     - Deploy the complete application stack"
echo -e "${GREEN}./cleanup.sh${NC}    - Clean up all resources"
echo -e "${GREEN}./quickstart.sh${NC} - Show this help guide"

echo -e "\n${CYAN}Deployment Options:${NC}"
echo -e "${YELLOW}Basic Deployment:${NC}"
echo -e "  ./deploy.sh"

echo -e "\n${YELLOW}Production Deployment:${NC}"
echo -e "  DEPLOY_ENV=production ./deploy.sh"

echo -e "\n${CYAN}Cleanup Options:${NC}"
echo -e "${YELLOW}Quick Cleanup (no prompts):${NC}"
echo -e "  ./cleanup.sh --quick"

echo -e "\n${YELLOW}Full Cleanup (with backup):${NC}"
echo -e "  ./cleanup.sh --full"

echo -e "\n${YELLOW}Stop Services Only:${NC}"
echo -e "  ./cleanup.sh --stop"

echo -e "\n${CYAN}Docker Management:${NC}"
echo -e "${YELLOW}View running services:${NC}"
echo -e "  docker-compose ps"

echo -e "\n${YELLOW}View logs:${NC}"
echo -e "  docker-compose logs -f"

echo -e "\n${YELLOW}Restart services:${NC}"
echo -e "  docker-compose restart"

echo -e "\n${CYAN}Default Access URLs:${NC}"
HOST_IP=$(hostname -I | awk '{print $1}' 2>/dev/null || echo "localhost")
echo -e "Frontend:               http://$HOST_IP:3000"
echo -e "API Gateway:            http://$HOST_IP:38080"
echo -e "Nutrition Service:      http://$HOST_IP:38081"
echo -e "Analytics Service:      http://$HOST_IP:38082"
echo -e "Prometheus:             http://$HOST_IP:9090"
echo -e "Grafana:                http://$HOST_IP:3001"

echo -e "\n${CYAN}Getting Started:${NC}"
echo -e "1. Run: ${GREEN}./deploy.sh${NC}"
echo -e "2. Wait for deployment to complete"
echo -e "3. Open http://$HOST_IP:3000 in your browser"
echo -e "4. Enjoy your Meal Tracker Pro application!"

echo -e "\n${YELLOW}Need help? Check the README.md or run:${NC}"
echo -e "  ./deploy.sh --help"
echo -e "  ./cleanup.sh --help"

echo -e "\n${GREEN}Happy meal tracking! 🍽️${NC}"
