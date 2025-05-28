# 🍳 Meal Prep Pro - AI-Powered Nutrition & Meal Planning Platform

A comprehensive, intelligent meal preparation and nutrition tracking platform built with modern technologies and AI-driven insights.

## 🌟 Features

### 🧠 AI-Powered Intelligence
- **Smart Meal Recommendations** - AI analyzes your preferences, nutrition goals, and available ingredients
- **Predictive Nutrition Analytics** - Forecast potential deficiencies and health trends
- **Automated Meal Planning** - Generate complete meal plans optimized for nutrition, cost, and time
- **Intelligent Recipe Scaling** - Automatically adjust recipes for different serving sizes
- **Real-time Nutrition Analysis** - Instant nutritional breakdown of any meal or recipe

### 📊 Advanced Nutrition Tracking
- **Comprehensive Macro & Micronutrient Tracking** - Track all essential vitamins, minerals, and compounds
- **Bioavailability Calculations** - Account for nutrient absorption rates and food interactions
- **Goal Setting & Progress Monitoring** - Set and track custom nutrition goals with detailed analytics
- **Health Insights Dashboard** - Visualize nutrition trends and receive personalized recommendations
- **Integration with Health Apps** - Sync with fitness trackers and health monitoring devices

### 🗓️ Smart Meal Planning
- **Drag & Drop Calendar Interface** - Intuitive meal planning with visual calendar
- **Automated Shopping Lists** - Generate optimized shopping lists from meal plans
- **Prep Time Optimization** - Minimize total prep time through intelligent scheduling
- **Cost Analysis** - Track and optimize meal costs with budget-friendly alternatives
- **Seasonal Adjustments** - Adapt meal plans based on seasonal ingredient availability

### 🔧 Advanced Features
- **Real-time Collaboration** - Share and collaborate on meal plans with family/roommates
- **Voice Commands** - Add meals and log nutrition through voice interface
- **Inventory Management** - Track pantry items and get expiration reminders
- **Recipe Import** - Import recipes from websites with automatic nutrition analysis
- **Custom Recipe Creation** - Build and share your own recipes with nutrition calculations

## 🏗️ Architecture

### Microservices Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (React)                        │
├─────────────────────────────────────────────────────────────┤
│                   API Gateway (Rust)                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐ ┌─────────────────┐ ┌───────────────┐ │
│  │  Nutrition AI   │ │   Analytics     │ │   Real-time   │ │
│  │    Service      │ │    Service      │ │   WebSocket   │ │
│  └─────────────────┘ └─────────────────┘ └───────────────┘ │
├─────────────────────────────────────────────────────────────┤
│            PostgreSQL + Redis + Monitoring                 │
└─────────────────────────────────────────────────────────────┘
```

### Technology Stack

#### Backend Services (Rust)
- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL 15 with SQLX
- **Cache**: Redis 7 for session management and caching
- **AI/ML**: Custom Rust implementations with ndarray and polars
- **Authentication**: JWT-based with bcrypt password hashing
- **API Documentation**: OpenAPI/Swagger integration

#### Frontend (React + TypeScript)
- **Framework**: React 18 with TypeScript
- **State Management**: Redux Toolkit + React Query
- **UI Components**: Material-UI (MUI) with custom themes
- **Charts & Visualization**: Chart.js and D3.js integration
- **Real-time**: WebSocket integration for live updates
- **PWA**: Progressive Web App capabilities

#### Infrastructure
- **Containerization**: Docker + Docker Compose
- **Monitoring**: Prometheus + Grafana
- **Load Balancing**: Nginx with SSL termination
- **CI/CD**: GitHub Actions (ready for deployment)
- **Database Migrations**: sqlx-migrate for version control

## 🚀 Quick Start

### Prerequisites
- Docker and Docker Compose
- Node.js 18+ (for local frontend development)
- Rust 1.82+ (for local backend development)

### 🐳 Docker Setup (Recommended)
```bash
# 1. Clone the repository
git clone https://github.com/yourusername/meal-prep-pro.git
cd meal-prep-pro

# 2. Start all services
docker-compose up -d

# 3. Access the application
# Frontend: http://localhost:39000
# API Gateway: http://localhost:38080
# Grafana Monitoring: http://localhost:39091 (admin/admin123)
```

### 🛠️ Local Development Setup

#### Backend Services
```bash
# Start database and Redis
docker-compose up -d postgres redis

# Navigate to API Gateway
cd services/api-gateway

# Install dependencies and run migrations
cargo build
export DATABASE_URL="postgresql://mealprep:mealprep_secure_2024@localhost:35432/mealprep"
sqlx migrate run

# Start the API Gateway
cargo run

# In separate terminals, start other services
cd ../nutrition-service && cargo run
cd ../analytics-service && cargo run
```

#### Frontend Development
```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm start

# Access at http://localhost:3000
```

## 📊 API Documentation

### Authentication Endpoints
```http
POST /api/auth/register     # User registration
POST /api/auth/login        # User login
POST /api/auth/refresh      # Token refresh
```

### Nutrition API
```http
POST /api/nutrition/analyze/meal      # Analyze meal nutrition
POST /api/nutrition/analyze/daily     # Daily nutrition summary
GET  /api/nutrition/trends            # Nutrition trends analysis
POST /api/nutrition/recommendations   # AI meal recommendations
```

### Meal Planning API
```http
GET    /api/meal-plans               # List meal plans
POST   /api/meal-plans               # Create meal plan
POST   /api/meal-plans/generate      # AI-generated meal plan
PUT    /api/meal-plans/:id           # Update meal plan
```

### Recipe Management
```http
GET    /api/recipes                  # List recipes
POST   /api/recipes                  # Create recipe
GET    /api/recipes/:id              # Get recipe details
POST   /api/recipes/:id/scale        # Scale recipe servings
POST   /api/recipes/import           # Import from URL
```

For complete API documentation, visit `/docs` when running the API Gateway.

## 🧪 Testing

### Backend Testing
```bash
# Run all backend tests
cd services/api-gateway
cargo test

# Run with coverage
cargo test --release --all-features
```

### Frontend Testing
```bash
cd frontend

# Run unit tests
npm test

# Run e2e tests
npm run test:e2e

# Generate coverage report
npm run test:coverage
```

### Integration Testing
```bash
# Start test environment
docker-compose -f docker-compose.test.yml up -d

# Run integration tests
./scripts/run-integration-tests.sh
```

## 📈 Performance & Monitoring

### Metrics Dashboard
Access Grafana at `http://localhost:39091` to view:
- **API Performance**: Request latency, throughput, error rates
- **Database Metrics**: Connection pool usage, query performance
- **System Resources**: CPU, memory, disk usage
- **Business Metrics**: User engagement, feature usage analytics

### Performance Benchmarks
- **API Response Time**: < 100ms for 95% of requests
- **Database Queries**: Optimized with proper indexing
- **Memory Usage**: < 512MB per service container
- **Concurrent Users**: Tested for 1000+ simultaneous users

## 🛡️ Security Features

### Data Protection
- **Password Security**: bcrypt hashing with salt
- **JWT Tokens**: Secure token-based authentication
- **HTTPS Only**: SSL/TLS encryption for all communications
- **Input Validation**: Comprehensive input sanitization
- **SQL Injection Protection**: Parameterized queries with SQLX

### Privacy Compliance
- **GDPR Ready**: User data export and deletion capabilities
- **Data Minimization**: Only collect necessary user information
- **Audit Logging**: Comprehensive activity logging for security
- **Role-Based Access**: Fine-grained permission system

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test && npm test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Code Standards
- **Rust**: Follow standard Rust conventions with `cargo fmt` and `cargo clippy`
- **TypeScript**: ESLint configuration with Prettier formatting
- **Documentation**: Comprehensive docs for all public APIs
- **Testing**: Minimum 80% code coverage requirement

## 📋 Roadmap

### Version 1.1 (Q2 2024)
- [ ] Mobile app (React Native)
- [ ] Advanced AI nutrition coaching
- [ ] Integration with grocery delivery services
- [ ] Social features and meal sharing

### Version 1.2 (Q3 2024)
- [ ] Multi-language support
- [ ] Advanced meal prep time optimization
- [ ] Integration with smart kitchen appliances
- [ ] Enhanced accessibility features

### Version 2.0 (Q4 2024)
- [ ] Machine learning-powered health predictions
- [ ] Integration with healthcare providers
- [ ] Advanced dietary restriction handling
- [ ] Enterprise team features

## 🐛 Troubleshooting

### Common Issues

**Services won't start**
```bash
# Check if ports are in use
netstat -tlnp | grep :38080

# Reset Docker environment
docker-compose down -v
docker-compose up -d
```

**Database connection errors**
```bash
# Check database status
docker-compose logs postgres

# Recreate database
docker-compose down postgres
docker volume rm meal-prep-pro_postgres_data
docker-compose up -d postgres
```

**Frontend build failures**
```bash
# Clear npm cache
npm cache clean --force
rm -rf node_modules package-lock.json
npm install
```

### Getting Help
- 📖 Check our [Documentation](docs/)
- 🐛 [Report Issues](https://github.com/yourusername/meal-prep-pro/issues)
- 💬 [Join Discord Community](https://discord.gg/mealprep-pro)
- 📧 Email: support@mealprep.pro

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community** for the amazing ecosystem
- **React Team** for the excellent frontend framework
- **PostgreSQL** for robust data storage
- **OpenAI** for AI/ML inspiration and techniques
- **Contributors** who make this project possible

---

**Built with ❤️ and 🦀 Rust by the Meal Prep Pro Team**

For more information, visit our [website](https://mealprep.pro) or follow us on [Twitter](https://twitter.com/mealpreppro).
