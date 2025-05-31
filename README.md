# 🍽️ Meal Prep Pro - Smart Meal Planning & Nutrition Platform

A comprehensive meal planning and nutrition tracking platform built with modern web technologies, featuring AI-powered recommendations, real-time analytics, and a beautiful user interface.

## 🌟 Features

### 🎯 Core Functionality
- **Smart Meal Planning** - AI-powered weekly meal plans based on your preferences
- **Nutrition Tracking** - Comprehensive macro and micronutrient monitoring
- **Recipe Management** - Create, organize, and share your favorite recipes
- **Shopping Lists** - Auto-generated grocery lists from meal plans
- **Analytics Dashboard** - Real-time insights into your nutrition patterns

### 🤖 AI-Powered Features
- **Personalized Recommendations** - ML-driven meal suggestions
- **Nutrition Analysis** - Automated nutritional breakdown of recipes
- **Goal Optimization** - Smart nutrition goal calculation and tracking
- **Predictive Insights** - Forecast nutrition trends and achievements

### 📱 User Experience
- **Responsive Design** - Works perfectly on desktop, tablet, and mobile
- **Real-time Updates** - WebSocket-powered live data synchronization
- **Dark/Light Mode** - Customizable interface themes
- **Progressive Web App** - Install and use offline capabilities

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (React)                        │
│  Next.js • TypeScript • Material-UI • Redux Toolkit       │
└─────────────────────┬───────────────────────────────────────┘
                      │ HTTP/WebSocket
┌─────────────────────▼───────────────────────────────────────┐
│                  API Gateway (Rust)                        │
│     Axum • Authentication • Rate Limiting • Monitoring     │
└─────────────────────┬───────────────────────────────────────┘
                      │
    ┌─────────────────┼─────────────────┐
    │                 │                 │
┌───▼────┐    ┌──────▼──────┐    ┌─────▼─────┐
│Nutrition│    │ Analytics   │    │   Cache   │
│Service  │    │  Service    │    │  (Redis)  │
│(Rust)   │    │   (Rust)    │    │           │
└─────────┘    └─────────────┘    └───────────┘
    │                 │                 │
    └─────────────────┼─────────────────┘
                      │
              ┌───────▼────────┐
              │   PostgreSQL   │
              │   Database     │
              └────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Docker & Docker Compose
- Node.js 18+ (for local development)
- Rust 1.70+ (for local development)

### 🐳 Docker Setup (Recommended)

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/meal-prep-pro.git
   cd meal-prep-pro
   ```

2. **Start all services**
   ```bash
   docker-compose up -d --build
   ```

3. **Access the application**
   - Frontend: http://localhost:33000
   - API Gateway: http://localhost:38080
   - Grafana Dashboard: http://localhost:33001 (admin/admin123)
   - Prometheus: http://localhost:39090

### 🛠️ Local Development

1. **Database Setup**
   ```bash
   docker run --name mealprep-db \
     -e POSTGRES_DB=mealprep \
     -e POSTGRES_USER=mealprep \
     -e POSTGRES_PASSWORD=mealprep_secure_2024 \
     -p 35432:5432 \
     -d postgres:15-alpine
   ```

2. **Backend Services**
   ```bash
   # API Gateway
   cd services/api-gateway
   cargo run

   # Nutrition Service
   cd services/nutrition-service
   cargo run

   # Analytics Service
   cd services/analytics-service
   cargo run
   ```

3. **Frontend**
   ```bash
   cd frontend
   npm install
   npm start
   ```

## 📊 Services Overview

### 🌐 API Gateway (Port 38080)
- **Authentication & Authorization** - JWT-based security
- **Request Routing** - Intelligent service discovery
- **Rate Limiting** - Prevent API abuse
- **Monitoring** - Prometheus metrics collection
- **WebSocket Support** - Real-time communication

### 🥗 Nutrition Service (Port 38081)
- **Nutrition Analysis** - Comprehensive meal analysis
- **AI Recommendations** - ML-powered meal suggestions
- **Goal Tracking** - Progress monitoring and insights
- **Trend Analysis** - Long-term nutrition patterns

### 📈 Analytics Service (Port 38082)
- **User Behavior Analytics** - Usage pattern analysis
- **Performance Metrics** - System health monitoring
- **Predictive Modeling** - Future trend predictions
- **Real-time Dashboards** - Live data visualization

## 🗄️ Database Schema

### Core Tables
- `users` - User accounts and authentication
- `user_profiles` - Extended user information
- `recipes` - Recipe definitions and metadata
- `meal_plans` - Meal planning schedules
- `nutrition_goals` - User nutrition targets
- `daily_nutrition` - Daily intake tracking

### Analytics Tables
- `user_activities` - Behavior tracking
- `api_requests` - Usage analytics
- `performance_metrics` - System monitoring

## 🔧 Configuration

### Environment Variables

**API Gateway**
```bash
DATABASE_URL=postgresql://user:pass@host:port/db
REDIS_URL=redis://host:port
JWT_SECRET=your-secret-key
RUST_LOG=info
```

**Frontend**
```bash
REACT_APP_API_URL=http://localhost:38080
REACT_APP_WS_URL=ws://localhost:38080/ws
```

### Docker Compose Override
Create `docker-compose.override.yml` for local customizations:

```yaml
version: '3.8'
services:
  api-gateway:
    environment:
      - RUST_LOG=debug
    ports:
      - "38080:8080"
```

## 📈 Monitoring & Observability

### Prometheus Metrics
- **HTTP Request Duration** - Response time tracking
- **Database Connection Pool** - Connection health
- **Memory Usage** - Resource consumption
- **Custom Business Metrics** - Meals planned, recipes created

### Grafana Dashboards
- **System Health** - Infrastructure monitoring
- **Application Performance** - Service-level metrics
- **User Analytics** - Usage patterns and trends
- **Business Intelligence** - Key performance indicators

### Logging
- **Structured Logging** - JSON format with correlation IDs
- **Log Levels** - Debug, Info, Warn, Error
- **Centralized Collection** - ELK stack ready

## 🧪 Testing

### Backend Testing
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Load testing
cargo test --release --test load
```

### Frontend Testing
```bash
# Unit tests
npm test

# E2E tests
npm run test:e2e

# Component tests
npm run test:component
```

## 🚢 Deployment

### Production Checklist
- [ ] Environment variables configured
- [ ] Database migrations applied
- [ ] SSL certificates installed
- [ ] Monitoring dashboards configured
- [ ] Backup procedures tested
- [ ] Security scan completed

### CI/CD Pipeline
```yaml
# .github/workflows/deploy.yml
name: Deploy to Production
on:
  push:
    branches: [main]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: |
          cargo test
          npm test
  deploy:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to production
        run: |
          docker-compose -f docker-compose.prod.yml up -d
```

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit your changes** (`git commit -m 'Add amazing feature'`)
4. **Push to the branch** (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

### Development Guidelines
- Follow Rust and TypeScript best practices
- Write comprehensive tests
- Update documentation
- Follow semantic versioning

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [docs.mealpreppro.com](https://docs.mealpreppro.com)
- **Issues**: [GitHub Issues](https://github.com/yourusername/meal-prep-pro/issues)
- **Discord**: [Community Server](https://discord.gg/mealpreppro)
- **Email**: support@mealpreppro.com

## 🙏 Acknowledgments

- **Rust Community** - Amazing ecosystem and tools
- **React Team** - Revolutionary frontend framework
- **PostgreSQL** - Reliable and powerful database
- **Material-UI** - Beautiful component library
- **Docker** - Containerization made easy

---

**Built with ❤️ by the Meal Prep Pro team**
