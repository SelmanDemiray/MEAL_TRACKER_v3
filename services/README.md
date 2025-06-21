# Microservices Architecture

This directory contains the backend microservices for the Meal Tracker application.

## Services Overview

### API Gateway
- Entry point for all client requests
- Handles authentication and authorization
- Routes requests to appropriate services
- Implements API rate limiting and monitoring
- Port: 8080

### Nutrition Service
- Manages food and recipe nutrition data
- Analyzes recipes for nutritional content
- Provides nutrition recommendations
- Handles meal planning optimization
- Port: 8081

### Analytics Service
- Processes user behavior data
- Generates insights from meal tracking
- Provides personalized recommendations
- Handles reporting and visualization data
- Port: 8082

## Development Guidelines

Each service follows these principles:

1. **Self-Contained**: Each service can operate independently
2. **API-First**: Clear API contracts and documentation
3. **Health Checks**: Endpoint for monitoring service health
4. **Metrics**: Prometheus metrics for observability
5. **Logging**: Structured logging with tracing information

## Communication Patterns

- **Synchronous**: RESTful APIs with JSON
- **Asynchronous**: When appropriate, using message passing via Redis

## Deployment

All services are containerized using Docker and can be deployed together using docker-compose or individually.
- Predictive modeling for health outcomes
- Business intelligence dashboards
- Performance metrics and KPIs

**Analytics Features**:
- Real-time trend analysis
- Predictive insights
- Custom report generation
- Data visualization support

## 🚀 Getting Started

### Prerequisites
- Rust 1.82 or later
- PostgreSQL 15+
- Redis 7+
- Docker (optional)

### Development Setup

1. **Install Dependencies**
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required tools
cargo install sqlx-cli
cargo install cargo-watch
```

2. **Environment Setup**
```bash
# Copy environment template
cp .env.example .env

# Set database URL
export DATABASE_URL="postgresql://mealprep:mealprep_secure_2024@localhost:5432/mealprep"
export REDIS_URL="redis://localhost:6379"
```

3. **Database Setup**
```bash
# Run migrations (from api-gateway directory)
cd api-gateway
sqlx migrate run
```

### Running Services

#### Option 1: Individual Services
```bash
# Terminal 1 - API Gateway
cd api-gateway
cargo run

# Terminal 2 - Nutrition Service
cd nutrition-service
cargo run

# Terminal 3 - Analytics Service
cd analytics-service
cargo run
```

#### Option 2: With Docker Compose
```bash
# From project root
docker-compose up -d
```

#### Option 3: Development Mode
```bash
# Auto-reload on changes
cd api-gateway
cargo watch -x run

# In separate terminals for other services
cd nutrition-service
cargo watch -x run
```

## 🔗 Service Communication

### Internal Communication
Services communicate via HTTP REST APIs:

```
┌─────────────────┐    HTTP/REST    ┌───────────────────┐
│   API Gateway   │◄───────────────►│ Nutrition Service │
│                 │                 │                   │
│                 │    HTTP/REST    ┌───────────────────┐
│                 │◄───────────────►│ Analytics Service │
└─────────────────┘                 └───────────────────┘
```

### External Communication
- **Frontend ↔ API Gateway**: REST API + WebSocket
- **Services ↔ Database**: PostgreSQL connection pool
- **Services ↔ Cache**: Redis connection

## 📝 API Conventions

### Request/Response Format
```json
{
  "data": {}, // Actual response data
  "meta": {
    "timestamp": "2024-01-01T00:00:00Z",
    "request_id": "uuid",
    "version": "1.0"
  },
  "errors": [] // Only present if errors occurred
}
```

### Error Handling
```json
{
  "errors": [
    {
      "code": "VALIDATION_ERROR",
      "message": "Invalid input provided",
      "field": "email",
      "details": {}
    }
  ]
}
```

### Status Codes
- `200` - Success
- `201` - Created
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `422` - Validation Error
- `500` - Internal Server Error

## 🧪 Testing

### Unit Tests
```bash
# Test all services
for service in */; do
  cd "$service"
  cargo test
  cd ..
done

# Test specific service
cd api-gateway
cargo test
```

### Integration Tests
```bash
# Run integration tests
cargo test --test integration_tests

# With database
TEST_DATABASE_URL="postgresql://test:test@localhost:5433/test" cargo test
```

### Load Testing
```bash
# Install tools
cargo install drill

# Run load tests
drill --benchmark tests/load_test.yml
```

## 📊 Monitoring

### Health Checks
All services expose health check endpoints:
- `GET /health` - Service health status
- `GET /metrics` - Prometheus metrics

### Metrics
Each service collects:
- Request rate, latency, and error rate
- Database connection pool status
- Memory and CPU usage
- Custom business metrics

### Logging
Structured logging with tracing:
```rust
use tracing::{info, warn, error};

info!(user_id = %user.id, "User logged in");
warn!(service = "nutrition", "Rate limit exceeded");
error!(error = %e, "Database connection failed");
```

## 🔧 Development Guidelines

### Code Structure
```
service-name/
├── src/
│   ├── main.rs              # Application entry point
│   ├── handlers/            # Request handlers
│   ├── models/              # Data models
│   ├── services/            # Business logic
│   ├── database/            # Database operations
│   └── utils/               # Utilities
├── tests/                   # Test files
├── Cargo.toml              # Dependencies
├── Dockerfile              # Container definition
└── README.md               # Service documentation
```

### Best Practices
1. **Error Handling**: Use `anyhow` for application errors, `thiserror` for library errors
2. **Async**: Prefer `async/await` for I/O operations
3. **Database**: Use connection pooling and prepared statements
4. **Caching**: Cache frequently accessed data in Redis
5. **Validation**: Validate all input data
6. **Security**: Never log sensitive information

### Performance Optimization
- Use `#[derive(Clone)]` sparingly
- Prefer `&str` over `String` for function parameters
- Use `Arc<T>` for shared immutable data
- Implement connection pooling for databases
- Use async I/O for external calls

## 🔐 Security

### Authentication
- JWT tokens with RS256 algorithm
- Token refresh mechanism
- Role-based access control

### Data Protection
- Input validation and sanitization
- SQL injection prevention with sqlx
- CORS configuration
- Rate limiting per IP/user

### Secrets Management
- Environment variables for configuration
- No hardcoded secrets in code
- Secure key rotation procedures

## 🚀 Deployment

### Docker Images
```bash
# Build production images
docker build -t meal-prep/api-gateway ./api-gateway
docker build -t meal-prep/nutrition-service ./nutrition-service
docker build -t meal-prep/analytics-service ./analytics-service
```

### Environment Variables
```bash
# Production environment
RUST_LOG=info
DATABASE_URL=postgresql://...
REDIS_URL=redis://...
JWT_SECRET=...
CORS_ORIGINS=https://app.mealprep.com
```

### Health Monitoring
- Kubernetes readiness/liveness probes
- Prometheus alerting rules
- Grafana dashboards for visualization

---

For service-specific documentation, see the README in each service directory.
