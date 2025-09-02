# Demo Service - Rust REST API สำหรับจัดการสินค้า

โปรเจกต์นี้เป็น REST API ที่พัฒนาด้วย Rust สำหรับจัดการข้อมูลสินค้า (Product Management System) โดยใช้ฐานข้อมูล SQLite

## ⚙️ สถาปัตยกรรมและเทคโนโลยี

### ไลบรารีหลักที่ใช้
- **Axum**: Web framework สำหรับสร้าง HTTP server และ API endpoints
- **SQLx**: Database driver แบบ async สำหรับ SQLite พร้อม type-safe queries
- **Tokio**: Async runtime สำหรับการทำงานแบบ asynchronous
- **Serde**: สำหรับ serialization/deserialization ของ JSON
- **UUID**: สำหรับสร้าง unique identifier ของสินค้า

### โครงสร้างข้อมูล
โปรเจกต์ใช้ SQLite database (`products.db`) เพื่อเก็บข้อมูลสินค้า

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

### 1. สร้างสินค้าใหม่
- **POST** `/products`
- **Body**: 
```json
{
    "name": "ชื่อสินค้า",
    "description": "คำอธิบาย",
    "price": 1299.99,
    "category": "หมวดหมู่"
}
```
- **Response**: สินค้าที่สร้างขึ้นพร้อม ID และ timestamp

### 2. ดูสินค้าทั้งหมด
- **GET** `/products`
- **Response**: Array ของสินค้าทั้งหมดในระบบ

### 3. ดูสินค้าตาม ID
- **GET** `/products/{id}`
- **Response**: ข้อมูลสินค้าที่ระบุ หรือ 404 Not Found

### 4. แก้ไขสินค้า
- **PUT** `/products/{id}`
- **Body**: ฟิลด์ที่ต้องการแก้ไข (ส่งเฉพาะที่จำเป็น)
```json
{
    "name": "ชื่อใหม่",
    "price": 999.99
}
```

### 5. ลบสินค้า
- **DELETE** `/products/{id}`
- **Response**: 204 No Content หรือ 404 Not Found

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

## 🏗️ การติดตั้งและรันโปรเจกต์

### ข้อกำหนดเบื้องต้น
- Rust toolchain (1.70+)
- SQLite3

### วิธีการรัน
```bash
# Clone และเข้าไปในโฟลเดอร์
cd demo-service

# ติดตั้ง dependencies
cargo build

# รันโปรเจกต์
cargo run
```

เซิร์ฟเวอร์จะรันที่ `http://127.0.0.1:3000`

## 📝 ตัวอย่างการใช้งาน

### สร้างสินค้าใหม่
```bash
curl -X POST http://127.0.0.1:3000/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "MacBook Pro",
    "description": "แล็ปท็อปสำหรับมืออาชีพ",
    "price": 2499.00,
    "category": "คอมพิวเตอร์"
  }'
```

### ดูสินค้าทั้งหมด
```bash
curl http://127.0.0.1:3000/products
```

### แก้ไขสินค้า
```bash
curl -X PUT http://127.0.0.1:3000/products/{product-id} \
  -H "Content-Type: application/json" \
  -d '{
    "price": 2299.00
  }'
```

## 🎯 จุดเด่นของโค้ด

1. **Type Safety**: ใช้ Rust type system เพื่อป้องกัน runtime errors
2. **Async/Await**: รองรับการทำงานแบบ concurrent ด้วย Tokio
3. **Memory Safety**: ไม่มี memory leaks ด้วย Rust ownership system
4. **Error Handling**: จัดการ error อย่างชัดเจนและปลอดภัย
5. **Database Safety**: Type-safe database queries ด้วย SQLx
6. **JSON Serialization**: อัตโนมัติด้วย Serde

## 📂 โครงสร้างไฟล์
```
demo-service/
├── Cargo.toml              # ไฟล์กำหนด dependencies
├── src/
│   └── main.rs            # โค้ดหลักของแอปพลิเคชัน
├── migrations/
│   └── 001_create_products.sql  # SQL สำหรับสร้างตาราง
├── products.db            # ไฟล์ฐานข้อมูล SQLite
└── target/                # ไฟล์ build output
```

## 🔮 การพัฒนาต่อ

สามารถเพิ่มฟีเจอร์เพิ่มเติมได้เช่น:
- Authentication และ Authorization
- Pagination สำหรับรายการสินค้า
- Search และ Filter
- Validation ที่ซับซ้อนขึ้น
- Caching
- Logging และ Monitoring
