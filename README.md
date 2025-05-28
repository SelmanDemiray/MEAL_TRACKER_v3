# 🍽️ Meal Prep Pro - Advanced AI-Powered Meal Planning Platform

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docker](https://img.shields.io/badge/Docker-Ready-blue.svg)](https://www.docker.com/)
[![Rust](https://img.shields.io/badge/Rust-1.82-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-18.2-blue.svg)](https://reactjs.org/)

An enterprise-grade, AI-powered meal preparation and nutrition tracking platform built with modern microservices architecture.

## 🚀 Features

### Core Functionality
- **AI-Powered Meal Planning**: Intelligent meal recommendations based on nutrition goals, preferences, and dietary restrictions
- **Advanced Nutrition Analysis**: Comprehensive nutritional tracking with micronutrient analysis and bioavailability calculations
- **Smart Recipe Management**: Recipe scaling, nutritional analysis, and AI-powered optimization suggestions
- **Intelligent Shopping Lists**: Automated list generation with cost optimization and store organization
- **Real-time Analytics**: Advanced insights into nutrition trends, goal adherence, and health predictions

### AI & Intelligence
- **Personalized Recommendations**: Machine learning-driven meal and supplement suggestions
- **Predictive Analytics**: Health trend analysis and deficiency prediction
- **Smart Optimization**: Meal plan optimization for nutrition, cost, and prep time
- **Adaptive Learning**: Continuously improving recommendations based on user behavior

### Advanced Features
- **Real-time Collaboration**: WebSocket-powered live updates and notifications
- **Progressive Web App**: Offline capabilities and mobile-first design
- **Enterprise Monitoring**: Comprehensive observability with Prometheus and Grafana
- **Microservices Architecture**: Scalable, maintainable, and fault-tolerant design

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Frontend      │────│   API Gateway    │────│  Microservices  │
│   React/TS      │    │   Rust/Axum     │    │    Rust/Axum    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │
                       ┌────────┴────────┐
                       │                 │
                  ┌─────────┐       ┌─────────┐
                  │ Redis   │       │ Postgres│
                  │ Cache   │       │ Database│
                  └─────────┘       └─────────┘
```

### Services Overview

| Service | Port | Purpose | Technology |
|---------|------|---------|------------|
| **API Gateway** | 8080 | Request routing, auth, rate limiting | Rust + Axum |
| **Nutrition Service** | 8081 | AI nutrition analysis and recommendations | Rust + AI/ML |
| **Analytics Service** | 8082 | Data analytics and insights | Rust + Analytics |
| **Frontend** | 3000 | User interface | React + TypeScript |
| **Prometheus** | 9090 | Metrics collection | Prometheus |
| **Grafana** | 3001 | Monitoring dashboards | Grafana |

## 🛠️ Technology Stack

### Backend
- **Language**: Rust 1.82
- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL 15
- **Cache**: Redis 7
- **AI/ML**: Custom Rust implementations with ndarray, polars
- **Authentication**: JWT + bcrypt
- **Monitoring**: Prometheus + Grafana

### Frontend
- **Framework**: React 18.2 with TypeScript
- **UI Library**: Material-UI (MUI) 5
- **State Management**: Redux Toolkit + React Query
- **Routing**: React Router 6
- **Animations**: Framer Motion
- **Build Tool**: Create React App

### Infrastructure
- **Containerization**: Docker + Docker Compose
- **Database**: PostgreSQL with migrations
- **Caching**: Redis for session and data caching
- **Monitoring**: Prometheus metrics + Grafana dashboards
- **Development**: Hot reload for all services

## 🚀 Quick Start

### Prerequisites
- Docker and Docker Compose
- Git

### 1. Clone the Repository
```bash
git clone <repository-url>
cd Meal_Prep_3
```

### 2. Start All Services
```bash
# Start all services with Docker Compose
docker-compose up -d

# Watch logs
docker-compose logs -f
```

### 3. Access the Application
- **Frontend**: http://localhost:39000
- **API Gateway**: http://localhost:38080
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:39091 (admin/admin123)

### 4. Development Setup
```bash
# For local development without Docker
./scripts/dev-setup.sh
```

## 📁 Project Structure

```
Meal_Prep_3/
├── services/                    # Microservices
│   ├── api-gateway/            # Main API gateway
│   ├── nutrition-service/      # AI nutrition analysis
│   └── analytics-service/      # Data analytics
├── frontend/                   # React frontend
├── database/                   # Database schemas and migrations
├── monitoring/                 # Prometheus & Grafana config
├── scripts/                    # Development and deployment scripts
├── docs/                       # Documentation
└── docker-compose.yml         # Multi-service orchestration
```

## 🔧 Development

### Running Individual Services

#### API Gateway
```bash
cd services/api-gateway
cargo run
```

#### Nutrition Service
```bash
cd services/nutrition-service
cargo run
```

#### Frontend
```bash
cd frontend
npm install
npm start
```

### Environment Variables
Create `.env` files in each service directory:

```bash
# Database
DATABASE_URL=postgresql://mealprep:mealprep_secure_2024@localhost:35432/mealprep

# Redis
REDIS_URL=redis://localhost:36379

# API Gateway
JWT_SECRET=your-secret-key
CORS_ORIGINS=http://localhost:3000
```

## 🧪 Testing

```bash
# Run all tests
./scripts/test-all.sh

# Test specific service
cd services/api-gateway
cargo test

# Frontend tests
cd frontend
npm test
```

## 📊 Monitoring

### Metrics
Access Prometheus at http://localhost:9090 to view metrics:
- Request rates and latencies
- Service health and uptime
- Database performance
- Custom business metrics

### Dashboards
Grafana dashboards available at http://localhost:39091:
- API Gateway performance
- Service health overview
- Database metrics
- Custom business intelligence

## 🗄️ Database

### Migrations
```bash
# Run migrations
cd services/api-gateway
sqlx migrate run

# Create new migration
sqlx migrate add <migration_name>
```

### Schema
The database includes tables for:
- Users and authentication
- Recipes and meal plans
- Nutrition tracking
- Analytics data
- Shopping lists and inventory

## 🚢 Deployment

### Production Deployment
```bash
# Build production images
docker-compose -f docker-compose.prod.yml build

# Deploy
docker-compose -f docker-compose.prod.yml up -d
```

### Environment Configuration
Set production environment variables:
- Database credentials
- Redis configuration
- JWT secrets
- External API keys

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style
- Rust: Follow `rustfmt` and `clippy` recommendations
- TypeScript: ESLint + Prettier configuration
- Commit messages: Conventional Commits format

## 📖 API Documentation

### Authentication
```bash
# Register
POST /api/auth/register
Content-Type: application/json

{
  "username": "user",
  "email": "user@example.com",
  "password": "securepassword"
}

# Login
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

### Core Endpoints
- `GET /api/meals` - List meals
- `POST /api/meal-plans` - Create meal plan
- `GET /api/nutrition/daily` - Daily nutrition summary
- `POST /api/recipes` - Create recipe
- `GET /api/analytics/dashboard` - Analytics dashboard

## 🔐 Security

- JWT-based authentication
- Password hashing with bcrypt
- CORS protection
- Rate limiting
- Input validation and sanitization
- Security headers

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Documentation**: [docs/](docs/)
- **Examples**: [examples/](examples/)

## 🙏 Acknowledgments

- Built with love using Rust and React
- Inspired by modern nutrition science and AI research
- Community-driven development approach

---

**Meal Prep Pro** - Revolutionizing meal planning with AI-powered intelligence 🚀
