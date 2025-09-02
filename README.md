# Rust Monolithic Application

A comprehensive Rust monolithic application featuring gRPC and REST APIs, database integrations (PostgreSQL with Diesel ORM and MongoDB), and cloud service integrations (AWS and Huawei Cloud).

## Features

- **Dual API Support**: Both gRPC and REST APIs running concurrently
- **Database Integration**: PostgreSQL with Diesel ORM and MongoDB support
- **Cloud Services**: AWS SDK and Huawei Cloud integration
- **Authentication**: JWT-based authentication middleware
- **Logging**: Structured logging with tracing
- **Configuration**: Environment-based configuration management
- **Error Handling**: Comprehensive error handling with custom error types
- **Validation**: Input validation utilities
- **Security**: Password hashing and encryption utilities

## Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root with app initialization
├── config/              # Configuration management
├── database/            # Database connections and migrations
│   ├── postgres.rs      # PostgreSQL with Diesel ORM
│   └── mongodb.rs       # MongoDB integration
├── grpc/                # gRPC server and services
│   ├── proto.rs         # Protocol buffer definitions
│   └── services.rs      # gRPC service implementations
├── rest/                # REST API server
│   ├── handlers/        # Request handlers
│   ├── middleware/      # Custom middleware
│   └── routes/          # Route definitions
├── models/              # Data models and structures
├── services/            # Business logic services
├── cloud/               # Cloud service integrations
│   ├── aws.rs           # AWS SDK integration
│   └── huawei.rs        # Huawei Cloud integration
└── utils/               # Utility functions
    ├── validation.rs    # Input validation
    ├── encryption.rs    # Encryption and hashing
    ├── date_time.rs     # Date/time utilities
    └── error.rs         # Error handling
```

## Prerequisites

- Rust 1.70+ (2021 edition)
- PostgreSQL database
- MongoDB database
- AWS account (optional, for cloud features)
- Huawei Cloud account (optional, for cloud features)

## Setup

1. **Clone the repository**

   ```bash
   git clone <repository-url>
   cd stander_monlothic_rust
   ```

2. **Install dependencies**

   ```bash
   cargo build
   ```

3. **Set up environment variables**

   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Set up databases**

   **PostgreSQL:**

   ```bash
   # Install diesel CLI
   cargo install diesel_cli --no-default-features --features postgres

   # Run migrations
   diesel migration run
   ```

   **MongoDB:**
   - Ensure MongoDB is running on the configured URI

5. **Run the application**

   ```bash
   cargo run
   ```

## Configuration

The application uses environment variables for configuration. Copy `.env.example` to `.env` and update the values:

### Server Configuration

- `SERVER_HOST`: Server bind address (default: 0.0.0.0)
- `SERVER_REST_PORT`: REST API port (default: 8080)
- `SERVER_GRPC_PORT`: gRPC server port (default: 50051)

### Database Configuration

- `DATABASE_URL`: PostgreSQL connection string
- `MONGODB_URI`: MongoDB connection URI
- `MONGODB_DATABASE`: MongoDB database name

### Cloud Configuration

- AWS: `AWS_REGION`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`
- Huawei: `HUAWEI_REGION`, `HUAWEI_ACCESS_KEY`, `HUAWEI_SECRET_KEY`

## API Endpoints

### REST API (Port 8080)

- `GET /health` - Health check endpoint
- `GET /api/v1/examples` - List examples
- `POST /api/v1/examples` - Create example
- `GET /api/v1/examples/{id}` - Get example by ID
- `PUT /api/v1/examples/{id}` - Update example
- `DELETE /api/v1/examples/{id}` - Delete example

### gRPC API (Port 50051)

- Example service with CRUD operations
- Protocol buffer definitions in `src/grpc/proto.rs`

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Database Migrations

```bash
# Create new migration
diesel migration generate <migration_name>

# Run migrations
diesel migration run

# Revert migrations
diesel migration revert
```

## Docker Support

To run with Docker:

```bash
# Build image
docker build -t rust-monolithic-app .

# Run container
docker run -p 8080:8080 -p 50051:50051 --env-file .env rust-monolithic-app
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run tests and ensure they pass
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
