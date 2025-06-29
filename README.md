# 🍽️ Meal Prep Pro - AI-Powered Meal Planning Platform

An advanced, AI-powered meal preparation and nutrition tracking platform built with modern web technologies.

## 🌟 Features

### 🤖 AI-Powered Intelligence
- **Smart Meal Recommendations** - Personalized meal suggestions based on preferences, health goals, and dietary restrictions
- **Nutritional Analysis** - Real-time nutritional breakdown with bioavailability calculations
- **Deficiency Prediction** - Early warning system for potential nutrient deficiencies
- **Recipe Optimization** - AI-driven recipe improvements for nutrition, taste, and cost

### 📊 Comprehensive Nutrition Tracking
- **Macro & Micronutrient Monitoring** - Track all essential nutrients with precision
- **Goal Setting & Progress** - Personalized nutrition goals with achievement tracking
- **Health Insights** - Predictive analytics for health trends and patterns
- **Meal Timing Optimization** - Optimal meal timing for health and performance

### 🗓️ Advanced Meal Planning
- **Weekly/Monthly Planning** - Flexible meal planning with drag-and-drop interface
- **Smart Shopping Lists** - Automatically generated and optimized shopping lists
- **Batch Cooking Optimization** - Efficient meal prep scheduling
- **Cost Optimization** - Budget-friendly meal planning with cost tracking

### 📱 Modern User Experience
- **Responsive Design** - Seamless experience across all devices
- **Real-time Updates** - Live synchronization with WebSocket connections
- **Offline Support** - Progressive Web App with offline capabilities
- **Voice Commands** - Hands-free interaction while cooking

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    Frontend     │    │   API Gateway   │    │  Microservices  │
│   (React TS)    │◄──►│     (Rust)      │◄──►│     (Rust)      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                        │
┌─────────────────────────────────────────┐    ┌─────────────────┐
│            Database Layer              │    │   Monitoring    │
│         PostgreSQL + Redis             │    │ Prometheus +    │
│                                       │    │    Grafana      │
└─────────────────────────────────────────┘    └─────────────────┘
```

### Technology Stack

**Frontend**
- React 18 with TypeScript
- Material-UI (MUI) for components
- Redux Toolkit for state management
- React Query for data fetching
- Framer Motion for animations

**Backend**
- Rust with Axum framework
- PostgreSQL 15 for data storage
- Redis 7 for caching and sessions
- JSON Web Tokens for authentication

**AI/ML**
- Custom Rust-native ML implementations
- ndarray for numerical computing
- polars for data processing
- Advanced nutrition algorithms

**Infrastructure**
- Docker & Docker Compose
- Prometheus & Grafana monitoring
- Nginx reverse proxy
- GitHub Actions CI/CD

## 🚀 Quick Start

### Prerequisites
- Docker & Docker Compose
- Git
- Node.js 18+ (for local development)
- Rust 1.82+ (for local development)

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/meal-prep-pro.git
cd meal-prep-pro
```

### 2. Environment Setup
```bash
# Copy environment files
cp .env.example .env
cp frontend/.env.example frontend/.env

# Edit configuration as needed
nano .env
```

### 3. Start with Docker Compose
```bash
# Build and start all services
docker-compose up --build -d

# Check service status
docker-compose ps
```

### 4. Access the Application
- **Frontend**: http://localhost:39000
- **API Gateway**: http://localhost:38080
- **Grafana Monitoring**: http://localhost:39091 (admin/admin)
- **Prometheus**: http://localhost:39090

### 5. Initial Setup
```bash
# Create admin user (optional)
curl -X POST http://localhost:38080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "email": "admin@example.com",
    "password": "secure_password_123"
  }'
```

## 🛠️ Development

### Local Development Setup
```bash
# 1. Start infrastructure services
docker-compose up postgres redis -d

# 2. Backend development
cd services/api-gateway
cargo run

# In another terminal
cd services/nutrition-service
cargo run

# 3. Frontend development
cd frontend
npm install
npm start
```

### Environment Variables
```bash
# Database
DATABASE_URL=postgresql://mealprep:mealprep_secure_2024@localhost:35432/mealprep
REDIS_URL=redis://localhost:36379

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key
JWT_EXPIRATION=3600

# Service URLs
NUTRITION_SERVICE_URL=http://localhost:8081
ANALYTICS_SERVICE_URL=http://localhost:8082

# Frontend
REACT_APP_API_URL=http://localhost:38080/api
REACT_APP_WS_URL=ws://localhost:38080/ws
```

### Running Tests
```bash
# Backend tests
cd services/api-gateway
cargo test

# Frontend tests
cd frontend
npm test

# Integration tests
./scripts/run-integration-tests.sh
```

## 📊 Monitoring & Analytics

### Health Checks
```bash
# Check all services
curl http://localhost:38080/health
curl http://localhost:38081/health
curl http://localhost:38082/health
```

### Metrics & Monitoring
- **Grafana Dashboards**: Pre-configured dashboards for all services
- **Prometheus Metrics**: Custom metrics for business logic
- **Logging**: Structured logging with tracing
- **Alerting**: Real-time alerts for critical issues

### Performance Metrics
- API response times
- Database query performance
- AI model inference speed
- User engagement analytics
- System resource utilization

## 🚢 Deployment

### Production Deployment
```bash
# Build production images
docker-compose -f docker-compose.prod.yml build

# Deploy to production
docker-compose -f docker-compose.prod.yml up -d

# Run migrations
docker-compose exec api-gateway sqlx migrate run
```

### Scaling Services
```bash
# Scale specific services
docker-compose up --scale nutrition-service=3 -d
docker-compose up --scale analytics-service=2 -d
```

### Backup & Recovery
```bash
# Database backup
docker-compose exec postgres pg_dump -U mealprep mealprep > backup.sql

# Restore from backup
docker-compose exec -T postgres psql -U mealprep mealprep < backup.sql
```

## 🧪 Testing

### Test Coverage
- Unit tests for all business logic
- Integration tests for API endpoints
- End-to-end tests for critical user flows
- Performance tests for AI models
- Load tests for scalability

### Running Tests
```bash
# All tests
make test

# Specific test suites
make test-backend
make test-frontend
make test-integration
make test-performance
```

## 📚 API Documentation

### Authentication
```bash
# Register user
POST /api/auth/register
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "secure_password"
}

# Login
POST /api/auth/login
{
  "email": "john@example.com",
  "password": "secure_password"
}
```

### Nutrition Analysis
```bash
# Analyze meal
POST /api/nutrition/analyze/meal
{
  "ingredients": [
    {"name": "chicken breast", "amount": 200, "unit": "g"},
    {"name": "brown rice", "amount": 150, "unit": "g"}
  ],
  "portion_size": 1.0,
  "cooking_method": "grilled"
}
```

### Meal Planning
```bash
# Generate AI meal plan
POST /api/meal-plans/generate
{
  "duration_days": 7,
  "dietary_preferences": ["high_protein", "low_carb"],
  "budget_max": 100.00,
  "prep_time_max": 120
}
```

## 🤝 Contributing

### Development Workflow
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`make test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Code Standards
- **Rust**: Follow rustfmt and clippy recommendations
- **TypeScript**: Use ESLint and Prettier configurations
- **Commits**: Follow conventional commit format
- **Documentation**: Update docs for all changes

### Review Process
- All PRs require review from maintainers
- Automated tests must pass
- Code coverage must not decrease
- Performance benchmarks must not regress

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

### Getting Help
- **Documentation**: Check the `/docs` directory
- **Issues**: Open an issue on GitHub
- **Discussions**: Join our GitHub Discussions
- **Email**: support@mealpreppro.com

### Common Issues
- [Database Connection Issues](docs/troubleshooting/database.md)
- [Authentication Problems](docs/troubleshooting/auth.md)
- [Performance Optimization](docs/troubleshooting/performance.md)
- [Docker Issues](docs/troubleshooting/docker.md)

## 🗺️ Roadmap

### Version 1.1 (Q2 2024)
- [ ] Mobile app (React Native)
- [ ] Advanced meal prep scheduling
- [ ] Social features (meal sharing)
- [ ] Integration with fitness trackers

### Version 1.2 (Q3 2024)
- [ ] Grocery store integrations
- [ ] Recipe video generation
- [ ] Advanced dietary analysis
- [ ] Multi-language support

### Version 2.0 (Q4 2024)
- [ ] Marketplace for meal plans
- [ ] Professional chef features
- [ ] Enterprise meal planning
- [ ] Advanced AI nutritionist

## 🙏 Acknowledgments

- Open source libraries and their maintainers
- Nutrition databases (USDA, FoodData Central)
- The Rust and React communities
- Beta testers and early adopters

---

**Made with ❤️ for healthier eating and better meal planning**

# 🍲 Meal Tracker v3

A comprehensive meal planning and nutrition tracking application built with Rust microservices and React.

## 🏗️ Architecture

This application uses a microservices architecture with the following components:

- **API Gateway**: Entry point for all client requests, handles authentication and request routing
- **Nutrition Service**: Manages nutrition data, recipe analysis, and meal planning
- **Analytics Service**: Processes user data for insights and recommendations
- **Frontend**: React-based user interface
- **PostgreSQL**: Primary database for persistent storage
- **Redis**: Cache and message broker
- **Prometheus**: Monitoring and metrics collection

## 🚀 Quick Start

### Prerequisites

- Docker & Docker Compose
- Git

### Setup and Run

1. Clone the repository (if not already done):
   ```
   git clone https://github.com/yourusername/meal-tracker-v3.git
   cd meal-tracker-v3
   ```

2. Start the application:
   ```
   docker-compose up -d
   ```

3. Access the application:
   - Frontend: http://localhost:3000
   - API Gateway: http://localhost:38080
   - Prometheus: http://localhost:9090

## 📂 Project Structure

```
meal-tracker-v3/
├── services/
│   ├── api-gateway/         # API Gateway service (Rust/Axum)
│   ├── nutrition-service/    # Nutrition analysis service (Rust)
│   └── analytics-service/    # Data analytics service (Rust)
├── frontend/                # React frontend
├── database/                # Database migrations and schemas
├── monitoring/              # Prometheus configuration
├── docker-compose.yml       # Docker Compose configuration
└── .env                     # Environment variables
```

## 🔧 Development

### Running in Development Mode

For development with hot-reloading:

```
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

### Running Tests

```
# Run all tests
docker-compose exec api-gateway cargo test
docker-compose exec nutrition-service cargo test
docker-compose exec analytics-service cargo test

# Frontend tests
docker-compose exec frontend npm test
```

## 📦 Technologies

- **Backend**:
  - Rust
  - Axum web framework
  - SQLx for database access
  - Redis for caching
  - Tokio for async runtime

- **Frontend**:
  - React with TypeScript
  - Material-UI components
  - Redux Toolkit for state management
  - React Query for data fetching

- **DevOps**:
  - Docker & Docker Compose
  - Prometheus for monitoring
  - GitHub Actions for CI/CD

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

# Meal Tracker Pro

## Authentication

- **Register:** Create a new account via the Register page.
- **Login:** Use your email and password.
- **Demo:** Click "Try Free Demo" on the login page to explore the app without registration.

## API Endpoints

- `POST /api/auth/register` — Register a new user
- `POST /api/auth/login` — Login (use `demo@mealprep.com` / `demopass` for demo)
