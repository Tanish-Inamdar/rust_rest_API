# Production REST API

A high-performance, asynchronous REST API built with Rust, designed for scalability and maintainability. This service provides endpoints for health monitoring and product value predictions, utilizing a PostgreSQL backend.

## Features

* **Asynchronous Runtime**: Powered by `tokio` for high-concurrency performance.
* **Web Framework**: Built with `axum` for a type-safe, ergonomic routing layer.
* **Database Layer**: Uses `sqlx` with compile-time SQL verification and connection pooling.
* **Structured Logging**: Ready for integration with `tracing` for production-grade observability.
* **Environment Management**: Configuration via `dotenv` for seamless deployment across environments.

## 🛠 Tech Stack

* **Language**: [Rust](https://www.rust-lang.org/)
* **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
* **Database**: [PostgreSQL](https://www.postgresql.org/)
* **SQL Toolkit**: [SQLx](https://github.com/launchbadge/sqlx)
* **Async Runtime**: [Tokio](https://tokio.rs/)
* **Serialization**: [Serde](https://serde.rs/)

## Prerequisites

Before you begin, ensure you have the following installed:
* [Rust](https://www.rust-lang.org/tools/install) (latest stable)
* [PostgreSQL](https://www.postgresql.org/download/)
* [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (optional, for migrations)

## Setup & Installation

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd production-api
    ```

2.  **Configure Environment Variables:**
    Create a `.env` file in the root directory and set the database connection string:
    ```env
    DATABASE_URL=postgres://username:password@localhost:5432/database_name
    ```

3.  **Initialize the Database:**
    Ensure your PostgreSQL instance is running and the database specified in `DATABASE_URL` exists.

4.  **Build the Project:**
    ```bash
    cargo build
    ```

## Running the Application

To start the server in development mode:
```bash
cargo run
```
The server binds to `0.0.0.0:3000`.

## 🛣 API Endpoints

### Health Check
* **URL**: `/health`
* **Method**: `GET`
* **Description**: Returns a simple status message to verify the server is running.

### Get Prediction
* **URL**: `/predictions`
* **Method**: `GET`
* **Query Parameters**:
    * `product_id` (string, required): The unique identifier for the product.
* **Success Response**:
    * **Code**: 200 OK
    * **Content**:
        ```json
        {
          "product_id": "BTC/USD",
          "value": 80000.3
        }
        ```
* **Error Responses**:
    * **404 Not Found**: If the `product_id` does not exist in the database.
    * **500 Internal Server Error**: For database or server-side failures.

## Project Structure

```text
.
├── src/
│   ├── main.rs          # Server entry point and configuration
│   ├── http/            # Route handlers and HTTP logic
│   │   ├── health.rs    # Health check handler
│   │   └── prediction.rs # Prediction domain logic and DB queries
│   └── lib.rs           # Library exports
├── .env                 # Environment configuration (local)
└── Cargo.toml           # Project dependencies and metadata
```
