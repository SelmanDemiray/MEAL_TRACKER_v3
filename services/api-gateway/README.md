# API Gateway Service

The API Gateway service acts as the entry point for all client requests to the Meal Tracker backend.

## Features

- Authentication and Authorization
- Request routing to microservices
- API documentation
- Rate limiting
- Request/Response logging
- CORS handling
- Error standardization

## API Routes

- `/api/auth/*` - Authentication endpoints
- `/api/users/*` - User management endpoints
- `/api/recipes/*` - Recipe endpoints (forwarded to Nutrition Service)
- `/api/meal-plans/*` - Meal planning endpoints (forwarded to Nutrition Service)
- `/api/nutrition/*` - Nutrition data endpoints (forwarded to Nutrition Service)
- `/api/analytics/*` - Analytics and reporting endpoints (forwarded to Analytics Service)

## Development

### Prerequisites
- Rust 1.70+
- PostgreSQL 15
- Redis 7.0+

### Setup
```bash
# Build the service
cargo build

# Run the service
cargo run

# Run tests
cargo test
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| DATABASE_URL | PostgreSQL connection URL | postgres://mealprep:password@localhost:5432/mealprep |
| REDIS_URL | Redis connection URL | redis://localhost:6379 |
| PORT | Service port | 8080 |
| NUTRITION_SERVICE_URL | URL of the Nutrition Service | http://nutrition-service:8081 |
| ANALYTICS_SERVICE_URL | URL of the Analytics Service | http://analytics-service:8082 |
| JWT_SECRET | Secret for signing JWT tokens | (required) |
| JWT_EXPIRATION_HOURS | JWT token expiration in hours | 24 |

### API Documentation

When the service is running, API documentation is available at:
- Swagger UI: http://localhost:8080/swagger-ui/
- OpenAPI JSON: http://localhost:8080/api-docs/openapi.json
│   ├── services.rs            # Service orchestration
│   ├── models/                # Data models
│   │   └── mod.rs             # Shared data structures
│   ├── auth.rs                # Authentication logic
│   ├── database.rs            # Database operations
│   ├── cache.rs               # Redis caching
│   └── metrics.rs             # Prometheus metrics
├── migrations/                 # Database migrations
│   └── 001_initial.sql        # Initial schema
├── tests/                     # Test files
├── Cargo.toml                 # Dependencies
├── Dockerfile                 # Container definition
└── README.md                  # This file
```

## 🛠️ Installation & Setup

### Prerequisites
- Rust 1.82+
- PostgreSQL 15+
- Redis 7+

### Local Development
```bash
# 1. Clone and navigate
cd services/api-gateway

# 2. Install dependencies
cargo build

# 3. Set up environment
cp .env.example .env
# Edit .env with your configuration

# 4. Run database migrations
sqlx migrate run

# 5. Start the service
cargo run
```

### Environment Variables
```bash
# Database
DATABASE_URL=postgresql://mealprep:mealprep_secure_2024@localhost:5432/mealprep

# Redis
REDIS_URL=redis://localhost:6379

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key
JWT_EXPIRATION=3600

# Service URLs
NUTRITION_SERVICE_URL=http://localhost:8081
ANALYTICS_SERVICE_URL=http://localhost:8082

# CORS
CORS_ORIGINS=http://localhost:3000,http://localhost:39000

# Logging
RUST_LOG=debug
```

## 📡 API Endpoints

### Authentication
```http
POST /api/auth/register
POST /api/auth/login
POST /api/auth/refresh
```

### User Management
```http
GET    /api/users/profile
PUT    /api/users/profile
GET    /api/users/preferences
PUT    /api/users/preferences
```

### Meal Management
```http
GET    /api/meals
POST   /api/meals
GET    /api/meals/:id
PUT    /api/meals/:id
DELETE /api/meals/:id
GET    /api/meals/search
GET    /api/meals/recommendations
```

### Meal Planning
```http
GET    /api/meal-plans
POST   /api/meal-plans
GET    /api/meal-plans/:id
PUT    /api/meal-plans/:id
POST   /api/meal-plans/generate
```

### Nutrition Tracking
```http
POST   /api/nutrition/log
GET    /api/nutrition/daily
GET    /api/nutrition/weekly
GET    /api/nutrition/goals
PUT    /api/nutrition/goals
GET    /api/nutrition/analysis
```

### Recipe Management
```http
GET    /api/recipes
POST   /api/recipes
GET    /api/recipes/:id
POST   /api/recipes/:id/scale
POST   /api/recipes/import
```

### Shopping Lists
```http
GET    /api/shopping-lists
POST   /api/shopping-lists
GET    /api/shopping-lists/:id
POST   /api/shopping-lists/:id/items
POST   /api/shopping-lists/:id/optimize
```

### Analytics
```http
GET    /api/analytics/dashboard
GET    /api/analytics/trends
GET    /api/analytics/predictions
GET    /api/analytics/insights
```

### Inventory Management
```http
GET    /api/inventory
POST   /api/inventory/items
PUT    /api/inventory/items/:id
GET    /api/inventory/expiring
```

### System Endpoints
```http
GET    /health              # Health check
GET    /metrics             # Prometheus metrics
GET    /ws                  # WebSocket upgrade
```

## 🔐 Authentication

### JWT Token Structure
```json
{
  "sub": "user-uuid",
  "email": "user@example.com",
  "role": "user",
  "exp": 1704067200,
  "iat": 1704063600
}
```

### Authentication Flow
1. **Register/Login**: Client sends credentials
2. **Token Generation**: Server creates JWT with user claims
3. **Token Storage**: Client stores token securely
4. **Request Authentication**: Client includes token in Authorization header
5. **Token Validation**: Server validates token on each request
6. **Token Refresh**: Client refreshes token before expiration

### Authorization Middleware
```rust
pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract and validate JWT token
    // Add user context to request
    // Continue to next handler
}
```

## 🌐 WebSocket Management

### Connection Lifecycle
```rust
pub async fn handle_socket(socket: WebSocket, state: AppState) {
    // Connection established
    // Message routing
    // Connection cleanup
}
```

### Supported Events
- **User Notifications**: Real-time alerts and updates
- **Meal Plan Updates**: Live collaboration on meal plans
- **Nutrition Tracking**: Real-time nutrition goal updates
- **Recipe Changes**: Live recipe modifications
- **System Alerts**: Service status and maintenance notifications

## 📊 Monitoring & Metrics

### Health Check Response
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "version": "0.1.0",
  "services": {
    "database": "connected",
    "redis": "connected",
    "microservices": "operational"
  }
}
```

### Prometheus Metrics
- `http_requests_total` - Total HTTP requests
- `http_request_duration_seconds` - Request latency
- `websocket_connections_active` - Active WebSocket connections
- `database_pool_connections` - Database connection pool status
- `auth_tokens_issued_total` - JWT tokens issued
- `rate_limit_exceeded_total` - Rate limit violations

## 🧪 Testing

### Unit Tests
```bash
# Run all tests
cargo test

# Run specific test module
cargo test auth

# Run with output
cargo test -- --nocapture
```

### Integration Tests
```bash
# Test with real database
TEST_DATABASE_URL="postgresql://test:test@localhost:5433/test" cargo test

# Load testing
cargo install drill
drill --benchmark api_load_test.yml
```

### Example Test
```rust
#[tokio::test]
async fn test_user_registration() {
    let app = test_app().await;
    
    let response = app
        .post("/api/auth/register")
        .json(&serde_json::json!({
            "username": "testuser",
            "email": "test@example.com",
            "password": "securepassword"
        }))
        .await;
    
    assert_eq!(response.status(), StatusCode::CREATED);
}
```

## 🔧 Configuration

### Rate Limiting
```rust
// Configure in main.rs
RateLimitLayer::new(
    100,                    // requests
    Duration::from_secs(60) // per minute
)
```

### CORS Configuration
```rust
CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any)
```

### Database Pool
```rust
let db = PgPoolOptions::new()
    .max_connections(20)
    .acquire_timeout(Duration::from_secs(3))
    .connect(&database_url)
    .await?;
```

## 🚀 Performance Optimization

### Caching Strategy
- **Response Caching**: Frequently accessed data cached in Redis
- **Database Query Caching**: Prepared statements and connection pooling
- **Static Asset Caching**: CDN integration for static content

### Connection Management
- **Database Pool**: Configurable connection pool size
- **Redis Connection**: Connection reuse and pipelining
- **HTTP Keep-Alive**: Persistent connections for better performance

### Async Processing
- All I/O operations are asynchronous
- Non-blocking request handling
- Efficient resource utilization

## 🐛 Troubleshooting

### Common Issues

**Database Connection Failed**
```bash
# Check database is running
docker ps | grep postgres

# Verify connection string
psql "postgresql://mealprep:mealprep_secure_2024@localhost:35432/mealprep"
```

**Redis Connection Failed**
```bash
# Check Redis is running
docker ps | grep redis

# Test connection
redis-cli -h localhost -p 36379 ping
```

**WebSocket Connection Issues**
```bash
# Check WebSocket endpoint
wscat -c ws://localhost:38080/ws
```

### Logs Analysis
```bash
# View structured logs
cargo run 2>&1 | grep "ERROR\|WARN"

# Filter by component
cargo run 2>&1 | grep "auth"
```

## 🔄 Development Workflow

### Hot Reloading
```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

### Database Migrations
```bash
# Create new migration
sqlx migrate add create_new_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Code Quality
```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Security audit
cargo audit
```

---

## 📞 Support

For issues specific to the API Gateway:
1. Check the logs for error messages
2. Verify environment configuration
3. Test individual endpoints with curl/Postman
4. Check service dependencies (database, Redis)

For general platform issues, see the main project README.
