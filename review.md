# Code Review: demo-service (Rust / Axum / SQLx)

## ภาพรวม
- โปรเจกต์เป็น REST API สำหรับจัดการสินค้า (CRUD) ด้วย Axum + SQLx + SQLite พร้อม OpenAPI/Swagger UI
- โครงสร้างโดยรวมสะอาด เข้าใจง่าย และแยก concerns พอสมควรสำหรับบริการขนาดเล็ก
- มีเอกสาร README ดี ครอบคลุมการใช้งานพื้นฐาน แต่บางจุดยังไม่สอดคล้องกับโค้ดจริง

## จุดเด่น
- โค้ดกระชับ อ่านง่าย: เส้นทาง (routes) และ handler แยกชัดเจนใน `src/main.rs`
- ใช้ binding ใน SQLx ปลอดภัยจาก SQL injection
- ใช้ `utoipa` เพื่อสร้าง OpenAPI และมี Swagger UI ให้ทดสอบ API ได้สะดวก
- ครอบคลุม CRUD ครบถ้วน พร้อมสถานะ HTTP ที่เหมาะสม (201/204/404/500)

## ประเด็นที่ควรปรับปรุง (สำคัญ)
- Error handling
  - มีการใช้ `unwrap()` หลายจุด (เช่น สร้างไฟล์ DB, เชื่อมต่อ DB, `Uuid::parse_str`) เสี่ยง panic ควร map เป็น `StatusCode` ที่เหมาะสมหรือ log แล้วคืน 500
- การจัดการสคีมาฐานข้อมูล / migrations
  - มีโฟลเดอร์ `migrations/001_create_products.sql` แต่โค้ดไม่ได้เรียกใช้ migration ใด ๆ (ใช้ `CREATE TABLE IF NOT EXISTS` ใน runtime แทน)
  - สคีมาจาก runtime (TEXT timestamps) ไม่ตรงกับไฟล์ migration (DATETIME) ควรทำให้สอดคล้องและใช้ migration เดียวเป็น single source of truth
- ประเภทข้อมูลวันที่/เวลา
  - ฟิลด์ `created_at`/`updated_at` ถูกจัดเก็บและ return เป็น `String` ทั้งที่เปิดใช้ `chrono` แล้ว แนะนำให้ใช้ `chrono::DateTime` ที่ map กับ SQLx เพื่อความถูกต้องเชิงชนิดข้อมูล
- ประเภทข้อมูลราคา
  - ใช้ `f64` และ `REAL` ใน SQLite ซึ่งไม่เหมาะกับจำนวนเงิน (ปัญหาความแม่นยำ) แนะนำเก็บเป็นจำนวนเต็ม (เช่น หน่วยย่อย เช่น “สตางค์”) หรือใช้ Decimal (เช่น `rust_decimal`/`sqlx` รองรับ)
- การตั้งค่า runtime
  - ที่อยู่/พอร์ตถูก hardcode เป็น `127.0.0.1:3000`; แนะนำอ่านจาก ENV (เช่น `HOST`/`PORT`) พร้อม default
  - ใช้ `SqlitePool::connect` โดยไม่มี options; แนะนำ `SqlitePoolOptions` (เช่น max connections, timeouts) และ `SqliteConnectOptions::create_if_missing(true)` แทนการ `File::create`
- Swagger UI แบบโหลด CDN
  - ดึงไฟล์จาก `unpkg.com` ภายนอก อาจล้มเหลวในสภาพแวดล้อมปิดเครือข่าย แนะนำพิจารณาใช้ `utoipa-swagger-ui` crate หรือเสิร์ฟ asset ภายใน
- ความสอดคล้องของเอกสาร
  - README ระบุ “SQLx compile-time verification” แต่โค้ดใช้ `query()` แบบ dynamic string ไม่มี `query!`/`query_as!` ตรวจสอบตอนคอมไพล์ ควรปรับคำอธิบายหรือเปลี่ยนมาใช้ macro ของ SQLx
- Repository hygiene
  - มีไฟล์ฐานข้อมูล `products.db` ใน repo ซึ่งไม่ควร commit (ควรใส่ใน `.gitignore`)
  - ไฟล์ `.gitignore` มีรายการของโปรเจกต์ Node จำนวนมากที่ไม่เกี่ยวข้อง

## ข้อเสนอแนะเชิงปฏิบัติ (priority → ต่ำ)
1) ปรับ Error handling ให้ไม่ panic
- แทน `unwrap()` ด้วยการ map error เป็น `StatusCode::INTERNAL_SERVER_ERROR` (และ log)
- ตัวอย่าง:
```rust
let pool = SqlitePool::connect(database_url)
    .await
    .map_err(|e| {
        eprintln!("db connect error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
```
- สำหรับ `Uuid::parse_str` ให้ handle error เช่น:
```rust
let id = Uuid::parse_str(&row.get::<String, _>("id"))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
```

2) ใช้ migrations อย่างเป็นทางการ
- เปิดใช้ `sqlx::migrate!()` และรันตอน start-up แทน `CREATE TABLE IF NOT EXISTS`
- ทำให้สคีมาที่ migration กับ model ตรงกัน (DATETIME/TEXT ให้เลือกอันเดียวและสอดคล้อง)
- ลบการ `File::create("./products.db")` แล้วใช้ `SqliteConnectOptions::new().filename("products.db").create_if_missing(true)`

3) ปรับชนิดข้อมูลเวลาและราคา
- ใช้ `chrono::{DateTime, Utc}` ใน struct และ derive Serde ด้วย `#[serde(with = "chrono::serde::ts_seconds")]` หรือฟอร์แมตที่ต้องการ
- ราคา: เปลี่ยนเป็น `i64` (หน่วยย่อย เช่น สตางค์) หรือ `Decimal` + `NUMERIC` ใน SQLite เพื่อลดปัญหา precision

4) เพิ่ม config จาก ENV และปรับการตั้งค่าของ Pool
- อ่าน `HOST`/`PORT`/`DATABASE_URL` จาก ENV (พร้อม default)
- ใช้ `SqlitePoolOptions::new().max_connections(5)` เป็นต้น

5) เสริม API usability และความปลอดภัย
- เพิ่ม CORS layer (`tower_http::cors`) สำหรับ client ที่ต่าง origin
- เพิ่ม health check (เช่น `GET /healthz`)
- เพิ่ม validation เบื้องต้น (ชื่อว่าง, ราคา < 0, category ว่าง)

6) ปรับปรุง Swagger UI
- พิจารณาใช้ `utoipa-swagger-ui` เพื่อตัดการพึ่งพา CDN
- ให้ path ในเอกสารสอดคล้องกับ router (docs ใช้ `/products/{id}` ส่วน router ใช้ `/products/:id` ซึ่งใช้งานจริงได้ แต่ควรรักษาความสม่ำเสมอ)

7) ความสอดคล้องของการอ่านค่าหลัง INSERT/UPDATE
- หลัง `INSERT` ควรอ่านค่าจากแถวที่ select กลับแล้ว map ทุกฟิลด์จาก DB (ไม่ผสม request + DB) เพื่อความแน่นอน
- หลัง `UPDATE` สามารถ `SELECT` ทั้งแถวเดียวเพื่อคืนค่าล่าสุด (ปัจจุบัน select เฉพาะ `updated_at`)

8) โครงสร้างโปรเจกต์และ docs
- พิจารณาแยกไฟล์: model, db, routes แทนรวมใน `main.rs` เมื่อโปรเจกต์โตขึ้น
- อัปเดต README ให้สอดคล้องกับโค้ดจริง (compile-time SQLx) และเพิ่มส่วน ENV variables

## หมายเหตุเชิงรายละเอียดในไฟล์
- `src/main.rs`
  - Binding ของเส้นทาง: ใช้ `"/products/:id"` กับ Axum 0.7 ใช้ได้ แต่ในเอกสาร Utoipa เป็น `{id}` สื่อสารให้ตรงกันจะดี
  - `swagger_ui()` อ้างอิง CDN อาจล้มเหลวถ้า network จำกัด
  - ใช้ `println!` แทน `tracing`/`tower_http::trace` — ควรพิจารณาเพิ่ม tracing เพื่อ production
- `migrations/001_create_products.sql`
  - ชนิดคอลัมน์เวลาคือ `DATETIME` แต่ runtime table สร้างเป็น `TEXT` — เลือกทางเดียวและปรับให้สอดคล้องกับ model/serde
- `.gitignore`
  - แนะนำเพิ่ม `products.db` และลบ entries ของ Node ที่ไม่เกี่ยวข้อง เพื่อลดสัญญาณรบกวน

## การทดสอบที่แนะนำ
- Integration tests สำหรับเส้นทางหลัก (POST/GET/PUT/DELETE) ผ่าน `axum::Router` ใน memory (ใช้ `tower::ServiceExt`)
- Property tests สำหรับ validation (ชื่อ/ราคา)
- Test concurrency (สร้าง/อัปเดตพร้อมกัน) เพื่อตรวจสอบ race conditions เบื้องต้น

## สรุป
- โครงสร้างและฟังก์ชันหลักทำได้ครบและอ่านง่าย เหมาะสำหรับ demo/service ขนาดเล็ก
- เพื่อความพร้อมใช้งานจริง แนะนำจัดการ error ให้ไม่ panic, ใช้ migrations อย่างเป็นระบบ, ปรับชนิดข้อมูลเวลา/ราคา, เพิ่ม config จาก ENV, เพิ่ม CORS/healthz และ tracing/logging
- ปรับปรุงเอกสารและ `.gitignore` เพื่อความสะอาดและสอดคล้องของโปรเจกต์
