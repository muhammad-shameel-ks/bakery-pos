use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Database State wrapper
pub struct DbState {
    pub conn: std::sync::Mutex<Connection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BakeryProfile {
    pub id: Option<i32>,
    pub name: String,
    pub gstin: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub logo_base64: String,
    pub invoice_note: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub id: Option<i32>,
    pub item_code: String,
    pub item_name: String,
    pub category: String,
    pub alias: String,
    pub hsnc: String,
    pub tax_slab: String, // e.g. "5%", "12%"
    pub mrp: f64,
    pub our_price: f64,
    pub opening_stock: f64,
    pub reorder_level: f64,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BusinessParty {
    pub id: Option<i32>,
    pub party_type: String, // "Supplier" or "B2B Customer"
    pub business_name: String,
    pub address: String,
    pub contact_person: String,
    pub phone: String,
    pub gstin: String,
    pub opening_balance: f64,
}

// Line Item DTOs
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionLineDto {
    pub item_id: i32,
    pub quantity: f64,
    pub rate: f64,
    pub tax_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionDto {
    pub invoice_no: String,
    pub date: String,
    pub business_party_id: i32,
    pub payment_mode: String,
    pub notes: String,
    pub paid_or_received: f64,
    pub lines: Vec<TransactionLineDto>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RetailSaleDto {
    pub invoice_no: String,
    pub date: String,
    pub customer_name: String,
    pub payment_mode: String,
    pub received_amount: f64,
    pub lines: Vec<TransactionLineDto>,
}

// Structs returned to frontend
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PurchaseLine {
    pub id: i32,
    pub purchase_header_id: i32,
    pub item_id: i32,
    pub item_name: String,
    pub item_code: String,
    pub quantity: f64,
    pub rate: f64,
    pub tax_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PurchaseHeader {
    pub id: i32,
    pub invoice_no: String,
    pub date: String,
    pub business_party_id: i32,
    pub business_name: String,
    pub payment_mode: String,
    pub notes: String,
    pub subtotal: f64,
    pub tax_total: f64,
    pub grand_total: f64,
    pub paid_amount: f64,
    pub lines: Vec<PurchaseLine>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RetailSaleLine {
    pub id: i32,
    pub retail_sale_header_id: i32,
    pub item_id: i32,
    pub item_name: String,
    pub item_code: String,
    pub quantity: f64,
    pub rate: f64,
    pub tax_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RetailSaleHeader {
    pub id: i32,
    pub invoice_no: String,
    pub date: String,
    pub customer_name: String,
    pub payment_mode: String,
    pub subtotal: f64,
    pub tax_total: f64,
    pub grand_total: f64,
    pub received_amount: f64,
    pub lines: Vec<RetailSaleLine>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct B2BSaleLine {
    pub id: i32,
    pub b2b_sale_header_id: i32,
    pub item_id: i32,
    pub item_name: String,
    pub item_code: String,
    pub quantity: f64,
    pub rate: f64,
    pub tax_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct B2BSaleHeader {
    pub id: i32,
    pub invoice_no: String,
    pub date: String,
    pub business_party_id: i32,
    pub business_name: String,
    pub business_gstin: String,
    pub business_address: String,
    pub payment_mode: String,
    pub notes: String,
    pub subtotal: f64,
    pub tax_total: f64,
    pub grand_total: f64,
    pub received_amount: f64,
    pub lines: Vec<B2BSaleLine>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DashboardData {
    pub today_retail: f64,
    pub today_b2b: f64,
    pub today_purchase: f64,
    pub stock_value: f64,
    pub low_stock_items: Vec<StockStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StockStatus {
    pub id: i32,
    pub item_code: String,
    pub item_name: String,
    pub category: String,
    pub unit: String,
    pub opening_stock: f64,
    pub purchases: f64,
    pub retail_sales: f64,
    pub b2b_sales: f64,
    pub on_hand: f64,
    pub reorder_level: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LedgerRow {
    pub date: String,
    pub transaction_type: String, // "Opening Balance", "Purchase", "B2B Sale", etc.
    pub ref_no: String,
    pub debit: f64,
    pub credit: f64,
    pub balance: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DayBookRow {
    pub date: String,
    pub module: String, // "Purchase", "Retail Sale", "B2B Sale"
    pub ref_no: String,
    pub party_name: String,
    pub payment_mode: String,
    pub debit: f64,
    pub credit: f64,
}

// Initializes the SQLite connection and schema
pub fn init_db(db_path: PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    // Enable WAL mode
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    // Create Tables
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS bakery_profile (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            name TEXT NOT NULL,
            gstin TEXT,
            address TEXT,
            phone TEXT,
            email TEXT,
            logo_base64 TEXT,
            invoice_note TEXT
        );

        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_code TEXT NOT NULL UNIQUE,
            item_name TEXT NOT NULL,
            category TEXT,
            alias TEXT,
            hsnc TEXT,
            tax_slab TEXT NOT NULL,
            mrp REAL DEFAULT 0,
            our_price REAL DEFAULT 0,
            opening_stock REAL DEFAULT 0,
            reorder_level REAL DEFAULT 0,
            unit TEXT
        );

        CREATE TABLE IF NOT EXISTS business_parties (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            party_type TEXT NOT NULL, -- 'Supplier' or 'B2B Customer'
            business_name TEXT NOT NULL,
            address TEXT,
            contact_person TEXT,
            phone TEXT,
            gstin TEXT,
            opening_balance REAL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS purchase_headers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            invoice_no TEXT NOT NULL,
            date TEXT NOT NULL,
            business_party_id INTEGER NOT NULL,
            payment_mode TEXT NOT NULL,
            notes TEXT,
            subtotal REAL DEFAULT 0,
            tax_total REAL DEFAULT 0,
            grand_total REAL DEFAULT 0,
            paid_amount REAL DEFAULT 0,
            FOREIGN KEY (business_party_id) REFERENCES business_parties (id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS purchase_lines (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            purchase_header_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            rate REAL NOT NULL,
            tax_rate REAL NOT NULL,
            FOREIGN KEY (purchase_header_id) REFERENCES purchase_headers (id) ON DELETE CASCADE,
            FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS retail_sale_headers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            invoice_no TEXT NOT NULL UNIQUE,
            date TEXT NOT NULL,
            customer_name TEXT NOT NULL,
            payment_mode TEXT NOT NULL,
            subtotal REAL DEFAULT 0,
            tax_total REAL DEFAULT 0,
            grand_total REAL DEFAULT 0,
            received_amount REAL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS retail_sale_lines (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            retail_sale_header_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            rate REAL NOT NULL,
            tax_rate REAL NOT NULL,
            FOREIGN KEY (retail_sale_header_id) REFERENCES retail_sale_headers (id) ON DELETE CASCADE,
            FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS b2b_sale_headers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            invoice_no TEXT NOT NULL UNIQUE,
            date TEXT NOT NULL,
            business_party_id INTEGER NOT NULL,
            payment_mode TEXT NOT NULL,
            notes TEXT,
            subtotal REAL DEFAULT 0,
            tax_total REAL DEFAULT 0,
            grand_total REAL DEFAULT 0,
            received_amount REAL DEFAULT 0,
            FOREIGN KEY (business_party_id) REFERENCES business_parties (id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS b2b_sale_lines (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            b2b_sale_header_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            rate REAL NOT NULL,
            tax_rate REAL NOT NULL,
            FOREIGN KEY (b2b_sale_header_id) REFERENCES b2b_sale_headers (id) ON DELETE CASCADE,
            FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
        );"
    )?;

    // Seed data if database is empty
    let count: i32 = conn.query_row("SELECT count(*) FROM items", [], |r| r.get(0))?;
    if count == 0 {
        seed_data(&conn)?;
    }

    Ok(conn)
}

fn seed_data(conn: &Connection) -> Result<()> {
    // Seed Bakery Profile
    conn.execute(
        "INSERT OR IGNORE INTO bakery_profile (id, name, gstin, address, phone, email, logo_base64, invoice_note) 
         VALUES (1, 'A Plus Bakery & Confectionery', '29ABCDE1234F1Z5', '123 Sweet Tooth Lane, Baker Valley', '+91 98765 43210', 'hello@aplusbakery.com', '', 'Thank you for choosing A Plus Bakery! Have a sweet day.');",
        [],
    )?;

    // Seed Items
    conn.execute(
        "INSERT INTO items (item_code, item_name, category, alias, hsnc, tax_slab, mrp, our_price, opening_stock, reorder_level, unit) 
         VALUES 
         ('IT001', 'Milk Bread 400g', 'Bread', 'MB400', '19059090', '5%', 45.00, 40.00, 100.0, 20.0, 'Pcs'),
         ('IT002', 'Chocolate Fudge Cake 1Kg', 'Cakes', 'CFC1K', '19053100', '18%', 850.00, 800.00, 10.0, 3.0, 'Pcs'),
         ('IT003', 'Butter Cookies 250g', 'Cookies', 'BC250', '19053100', '18%', 120.00, 110.00, 50.0, 10.0, 'Pcs');",
        [],
    )?;

    // Seed Business Parties
    conn.execute(
        "INSERT INTO business_parties (party_type, business_name, address, contact_person, phone, gstin, opening_balance) 
         VALUES 
         ('Supplier', 'Golden Flour Mills', 'Industrial Area, City Center', 'John Miller', '98765 00001', '29SUPPL1234F1Z1', -5000.00),
         ('B2B Customer', 'Elite Café & Lounge', 'High Street Promenade', 'Alice Cooper', '98765 00002', '29CUSTO1234F1Z2', 1500.00);",
        [],
    )?;

    // Seed a Purchase Inward
    conn.execute(
        "INSERT INTO purchase_headers (invoice_no, date, business_party_id, payment_mode, notes, subtotal, tax_total, grand_total, paid_amount)
         VALUES ('PUR-001', '2026-07-26', 1, 'UPI', 'Opening stock raw material', 2000.0, 100.0, 2100.0, 2100.0);",
        [],
    )?;
    conn.execute(
        "INSERT INTO purchase_lines (purchase_header_id, item_id, quantity, rate, tax_rate)
         VALUES (1, 1, 50.0, 40.0, 5.0);",
        [],
    )?;

    // Seed a Retail Sale
    conn.execute(
        "INSERT INTO retail_sale_headers (invoice_no, date, customer_name, payment_mode, subtotal, tax_total, grand_total, received_amount)
         VALUES ('RET-001', '2026-07-26', 'Walk-in Customer', 'Cash', 137.28, 22.72, 160.0, 160.0);",
        [],
    )?;
    conn.execute(
        "INSERT INTO retail_sale_lines (retail_sale_header_id, item_id, quantity, rate, tax_rate)
         VALUES (1, 1, 2.0, 40.0, 5.0), (1, 3, 1.0, 110.0, 18.0);",
        [],
    )?;

    // Seed a B2B Sale
    conn.execute(
        "INSERT INTO b2b_sale_headers (invoice_no, date, business_party_id, payment_mode, notes, subtotal, tax_total, grand_total, received_amount)
         VALUES ('INV-001', '2026-07-26', 2, 'Card', 'Event delivery', 2110.16, 389.84, 2500.0, 2000.0);",
        [],
    )?;
    conn.execute(
        "INSERT INTO b2b_sale_lines (b2b_sale_header_id, item_id, quantity, rate, tax_rate)
         VALUES (1, 2, 2.0, 800.0, 18.0), (1, 3, 5.0, 110.0, 18.0);",
        [],
    )?;

    Ok(())
}
