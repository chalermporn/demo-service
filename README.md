# Demo Service - Rust Product Management API

A REST API built with Rust for product management using SQLite database, featuring OpenAPI documentation and Swagger UI.

## 🚀 Features

- **CRUD Operations**: Complete product management with Create, Read, Update, Delete
- **SQLite Database**: Persistent storage with automatic table creation
- **OpenAPI Documentation**: Auto-generated API documentation with Swagger UI
- **Type Safety**: Rust's type system ensures memory safety and prevents runtime errors
- **Async/Await**: High-performance concurrent request handling with Tokio
- **JSON API**: RESTful JSON endpoints with automatic serialization

## ⚙️ Technology Stack

### Core Libraries
- **Axum**: Modern web framework for HTTP server and API endpoints
- **SQLx**: Async database driver with type-safe queries for SQLite
- **Tokio**: Async runtime for concurrent operations
- **Serde**: JSON serialization/deserialization
- **UUID**: Unique identifier generation for products
- **Utoipa**: OpenAPI documentation generation

### Database
- **SQLite**: Lightweight, embedded database (`products.db`)
- Automatic table creation and schema management

## 📊 โครงสร้างข้อมูลสินค้า

### Product Model
```rust
pub struct Product {
    pub id: Uuid,           // รหัสสินค้าแบบ UUID
    pub name: String,       // ชื่อสินค้า
    pub description: String, // คำอธิบายสินค้า
    pub price: f64,         // ราคา (ตัวเลขทศนิยม)
    pub category: String,   // หมวดหมู่สินค้า
    pub created_at: String, // วันที่สร้าง
    pub updated_at: String, // วันที่แก้ไขล่าสุด
}
```

### Request Models
- **CreateProductRequest**: สำหรับสร้างสินค้าใหม่
- **UpdateProductRequest**: สำหรับแก้ไขสินค้า (ทุกฟิลด์เป็น Option)

## 🚀 API Endpoints

### Product Management
- **GET** `/products` - List all products
- **POST** `/products` - Create a new product
- **GET** `/products/{id}` - Get product by ID
- **PUT** `/products/{id}` - Update product by ID
- **DELETE** `/products/{id}` - Delete product by ID

### Documentation
- **GET** `/swagger-ui` - Interactive Swagger UI documentation
- **GET** `/api-docs/openapi.json` - OpenAPI specification in JSON format

### Request/Response Examples

#### Create Product
```bash
POST /products
Content-Type: application/json

{
    "name": "MacBook Pro",
    "description": "Professional laptop for developers",
    "price": 2499.99,
    "category": "Electronics"
}
```

#### Update Product
```bash
PUT /products/{id}
Content-Type: application/json

{
    "name": "MacBook Pro M3",
    "price": 2299.99
}
```

#### Response Format
```json
{
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "MacBook Pro",
    "description": "Professional laptop for developers",
    "price": 2499.99,
    "category": "Electronics",
    "created_at": "2025-09-02 14:30:00",
    "updated_at": "2025-09-02 14:30:00"
}
```

## 🔧 การทำงานของโค้ด

### 1. การเริ่มต้นระบบ (main function)
```rust
#[tokio::main]
async fn main() {
    // 1. สร้างไฟล์ database หากยังไม่มี
    // 2. เชื่อมต่อกับ SQLite database
    // 3. สร้างตาราง products หากยังไม่มี
    // 4. ตั้งค่า router และ endpoints
    // 5. เริ่มต้น HTTP server ที่ port 3000
}
```

### 2. การจัดการฐานข้อมูล
- ใช้ **SQLx** สำหรับ database operations แบบ async
- ใช้ **Connection Pool** เพื่อการจัดการ connection อย่างมีประสิทธิภาพ
- ทุก query มีการจัดการ error ด้วย `map_err`

### 3. การจัดการ HTTP Requests
- ใช้ **Axum extractors** สำหรับดึงข้อมูลจาก request:
  - `Path(id)`: ดึง UUID จาก URL path
  - `Json(request)`: แปลง JSON body เป็น struct
  - `State(pool)`: ดึง database pool จาก application state

### 4. Error Handling
- ทุก function คืนค่า `Result<T, StatusCode>`
- การจัดการ error แบบ explicit ด้วย HTTP status codes:
  - `500 INTERNAL_SERVER_ERROR`: Database errors
  - `404 NOT_FOUND`: ไม่พบสินค้า
  - `201 CREATED`: สร้างสำเร็จ
  - `204 NO_CONTENT`: ลบสำเร็จ

## 🏗️ Installation & Setup

### Prerequisites
- Rust toolchain (1.70+)
- SQLite3 (automatically handled)

### Quick Start
```bash
# Clone the repository
cd demo-service

# Build the project
cargo build

# Run the service
cargo run
```

The server will start at `http://127.0.0.1:3000`

### Available Endpoints
- API: `http://127.0.0.1:3000/products`
- Swagger UI: `http://127.0.0.1:3000/swagger-ui`
- OpenAPI Spec: `http://127.0.0.1:3000/api-docs/openapi.json`

## 📝 Usage Examples

### Create a Product
```bash
curl -X POST http://127.0.0.1:3000/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "MacBook Pro",
    "description": "Professional laptop for developers",
    "price": 2499.99,
    "category": "Electronics"
  }'
```

### List All Products
```bash
curl http://127.0.0.1:3000/products
```

### Get Product by ID
```bash
curl http://127.0.0.1:3000/products/{product-id}
```

### Update Product
```bash
curl -X PUT http://127.0.0.1:3000/products/{product-id} \
  -H "Content-Type: application/json" \
  -d '{
    "price": 2299.99
  }'
```

### Delete Product
```bash
curl -X DELETE http://127.0.0.1:3000/products/{product-id}
```

## 🎯 Key Features & Architecture

### Code Highlights
1. **Type Safety**: Rust's type system prevents runtime errors and ensures memory safety
2. **Async/Await**: High-performance concurrent request handling with Tokio runtime
3. **Memory Safety**: Zero-cost abstractions with Rust's ownership system - no memory leaks
4. **Error Handling**: Explicit error handling with Result types and HTTP status codes
5. **Database Safety**: Type-safe database queries with SQLx compile-time verification
6. **Auto Documentation**: OpenAPI spec generation with Swagger UI interface

### HTTP Status Codes
- `200 OK`: Successful GET requests
- `201 CREATED`: Successful product creation
- `204 NO_CONTENT`: Successful deletion
- `404 NOT_FOUND`: Product not found
- `500 INTERNAL_SERVER_ERROR`: Database or server errors

## 📂 Project Structure
```
demo-service/
├── Cargo.toml              # Dependencies and project configuration
├── src/
│   └── main.rs            # Main application code with API handlers
├── migrations/             # Database migration files (if using sqlx migrate)
│   └── 001_create_products.sql
├── products.db            # SQLite database file (auto-created)
├── README.md              # This documentation
└── target/                # Rust build artifacts
```

## 🔮 Future Enhancements

Potential features to add:
- **Authentication & Authorization**: JWT tokens, user roles
- **Pagination**: Limit/offset for large product lists  
- **Search & Filtering**: Query products by name, category, price range
- **Input Validation**: Advanced validation rules and custom error messages
- **Caching**: Redis integration for improved performance
- **Logging & Monitoring**: Structured logging and metrics collection
- **Rate Limiting**: Request throttling and API quotas
- **Database Migrations**: Versioned schema changes
- **Testing**: Unit and integration tests
- **Docker**: Containerization for easy deployment

## 🚀 Production Considerations

For production deployment:
- Use environment variables for configuration
- Implement proper logging (tracing, structured logs)
- Add health check endpoints
- Configure CORS for web client access
- Use connection pooling optimization
- Implement graceful shutdown
- Add database connection retries
- Set up monitoring and alerting
