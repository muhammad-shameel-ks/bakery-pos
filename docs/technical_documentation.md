# Technical Documentation: Bakery POS & Invoicing System

This documentation describes the codebase architecture, database structures, and backend integration.
It serves as a developer guide for maintaining and extending the application.

---

## 1. Architectural Stack

The application runs as a self-contained, offline-only desktop program.
It consists of three primary layers.

```mermaid
graph LR
    Frontend[React UI Layer] <--> |Tauri IPC invoke| Backend[Rust Command Controller]
    Backend <--> |rusqlite Connection| DB[(SQLite Database)]
```

*   **Frontend Shell**: Built on React 19, TypeScript, and Vite.
*   **Aesthetic System**: Engineered entirely in Vanilla CSS utilizing dynamic CSS variables to handle dark and light theme states.
*   **Desktop Interface Layer**: Powered by Tauri v2.
*   **Database Engine**: Built-in SQLite, accessed via the `rusqlite` crate in Rust.

---

## 2. Project Folder Map

The following lists the locations of the core files in the project workspace:

*   [src-tauri/Cargo.toml](file:///home/mallubeast/Workspace/Speehive/bakery-pos/src-tauri/Cargo.toml): Declares Rust dependency targets such as `rusqlite` and `chrono`.
*   [src-tauri/src/lib.rs](file:///home/mallubeast/Workspace/Speehive/bakery-pos/src-tauri/src/lib.rs): Bootstraps the database setup context and registers Tauri command handlers.
*   [src-tauri/src/db.rs](file:///home/mallubeast/Workspace/Speehive/bakery-pos/src-tauri/src/db.rs): Handles database connection pool management, schema migrations, and mock data seeding.
*   [src-tauri/src/commands.rs](file:///home/mallubeast/Workspace/Speehive/bakery-pos/src-tauri/src/commands.rs): Orchestrates SQL commands for CRUD models, calculations, and reporting.
*   [src/App.tsx](file:///home/mallubeast/Workspace/Speehive/bakery-pos/src/App.tsx): Contains the React single-page frontend views, transaction processing, holds logic, and offline Excel exports.
*   [src/App.css](file:///home/mallubeast/Workspace/Speehive/bakery-pos/src/App.css): Stores layout stylings, pastel color variables, POS view modules, and print stylesheets.

---

## 3. Database Schema Specification

The database runs in Write-Ahead Logging (WAL) mode to support concurrent operations.
Foreign key cascades are enforced on line-level transaction records.

### 3.1 Table: `bakery_profile`
Holds details for the bakery.
There is a single configuration row constrained to `id = 1`.

| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | INTEGER PRIMARY KEY | Must be `1` |
| `name` | TEXT | Bakery store name |
| `gstin` | TEXT | Store tax registry identifier |
| `address` | TEXT | Store postal address |
| `phone` | TEXT | Telephone contact number |
| `email` | TEXT | Store email address |
| `logo_base64` | TEXT | Store logo base64 image data |
| `invoice_note` | TEXT | Footer note for printed receipts |

### 3.2 Table: `items`
Stores the product inventory catalog.

| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | INTEGER PRIMARY KEY AUTOINCREMENT | Unique identifier |
| `item_code` | TEXT UNIQUE | SKU code used for barcode generation |
| `item_name` | TEXT | Name of product |
| `category` | TEXT | Food category group (e.g. Cakes, Cookies) |
| `alias` | TEXT | Short code shortcut |
| `hsnc` | TEXT | HSN code for tax categorization |
| `tax_slab` | TEXT | Tax rate slab (e.g. 5%, 12%, 18%) |
| `mrp` | REAL | Maximum retail price |
| `our_price` | REAL | Standard store sale rate |
| `opening_stock` | REAL | Stock count at configuration setup |
| `reorder_level` | REAL | Low stock warning trigger limit |
| `unit` | TEXT | Measurement unit (e.g. Pcs, Kg) |

### 3.3 Table: `business_parties`
Stores supplier vendors and B2B clients.

| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | INTEGER PRIMARY KEY AUTOINCREMENT | Unique identifier |
| `party_type` | TEXT | Either `'Supplier'` or `'B2B Customer'` |
| `business_name` | TEXT | Business legal name |
| `address` | TEXT | Office address |
| `contact_person` | TEXT | Representative name |
| `phone` | TEXT | Contact mobile number |
| `gstin` | TEXT | Partner tax registry identifier |
| `opening_balance` | REAL | Outstanding offset (Negative for Supplier, Positive for Customer) |

### 3.4 Inward Purchase Tables
Contains purchase header and lines.

#### `purchase_headers`
*   `id` (INTEGER PRIMARY KEY AUTOINCREMENT)
*   `invoice_no` (TEXT): Reference invoice identifier.
*   `date` (TEXT): Format: YYYY-MM-DD.
*   `business_party_id` (INTEGER FK -> `business_parties(id)` ON DELETE CASCADE)
*   `payment_mode` (TEXT)
*   `notes` (TEXT)
*   `subtotal` (REAL)
*   `tax_total` (REAL)
*   `grand_total` (REAL)
*   `paid_amount` (REAL): Partial payments made.

#### `purchase_lines`
*   `id` (INTEGER PRIMARY KEY AUTOINCREMENT)
*   `purchase_header_id` (INTEGER FK -> `purchase_headers(id)` ON DELETE CASCADE)
*   `item_id` (INTEGER FK -> `items(id)` ON DELETE CASCADE)
*   `quantity` (REAL)
*   `rate` (REAL)
*   `tax_rate` (REAL)

### 3.5 Retail Sale POS Tables
Logs fast-paced retail checkout receipts.

#### `retail_sale_headers`
*   `id` (INTEGER PRIMARY KEY AUTOINCREMENT)
*   `invoice_no` (TEXT UNIQUE)
*   `date` (TEXT)
*   `customer_name` (TEXT)
*   `payment_mode` (TEXT)
*   `subtotal` (REAL)
*   `tax_total` (REAL)
*   `grand_total` (REAL)
*   `received_amount` (REAL)

#### `retail_sale_lines`
*   `id` (INTEGER PRIMARY KEY AUTOINCREMENT)
*   `retail_sale_header_id` (INTEGER FK -> `retail_sale_headers(id)` ON DELETE CASCADE)
*   `item_id` (INTEGER FK -> `items(id)` ON DELETE CASCADE)
*   `quantity` (REAL)
*   `rate` (REAL)
*   `tax_rate` (REAL)

### 3.6 B2B Sale GST Tables
Logs B2B commercial sales.

#### `b2b_sale_headers`
*   `id` (INTEGER PRIMARY KEY AUTOINCREMENT)
*   `invoice_no` (TEXT UNIQUE)
*   `date` (TEXT)
*   `business_party_id` (INTEGER FK -> `business_parties(id)` ON DELETE CASCADE)
*   `payment_mode` (TEXT)
*   `notes` (TEXT)
*   `subtotal` (REAL)
*   `tax_total` (REAL)
*   `grand_total` (REAL)
*   `received_amount` (REAL): Amount collected upfront.

#### `b2b_sale_lines`
*   `id` (INTEGER PRIMARY KEY AUTOINCREMENT)
*   `b2b_sale_header_id` (INTEGER FK -> `b2b_sale_headers(id)` ON DELETE CASCADE)
*   `item_id` (INTEGER FK -> `items(id)` ON DELETE CASCADE)
*   `quantity` (REAL)
*   `rate` (REAL)
*   `tax_rate` (REAL)

---

## 4. Backend IPC Interface (Tauri Commands)

Rust controllers in the backend expose core database operations.
They return JSON-serialized objects or text error responses.

*   `get_dashboard_data()` -> `Result<DashboardData, String>`: Aggregates sales statistics and pulls low stock warnings.
*   `get_items(search_query: String)` -> `Result<Vec<Item>, String>`: Standard lookup filter query for products.
*   `save_item(item: Item)` -> `Result<(), String>`: Performs update if `item.id` is present, else inserts new record.
*   `get_businesses(party_type: String, search_query: String)` -> `Result<Vec<BusinessParty>, String>`: Vendor or customer database listing.
*   `get_business_ledger(party_id: i32)` -> `Result<Vec<LedgerRow>, String>`: Computes ledger statements with running balance columns.
*   `get_daybook(from_date: String, to_date: String)` -> `Result<Vec<DayBookRow>, String>`: Combines debits and credits sorted by timestamp.

---

## 5. Algorithmic Calculations

### 5.1 Dynamic Inventory Quantity On Hand
Inventory balances are calculated dynamically from transaction logs:
$$\text{OnHand} = \text{OpeningStock} + \text{InwardPurchases} - (\text{RetailSales} + \text{B2BSales})$$
This calculation is executed directly in the database engine for optimal speed:
```sql
SELECT 
    i.id, i.opening_stock,
    COALESCE((SELECT SUM(quantity) FROM purchase_lines WHERE item_id = i.id), 0) as purchased,
    COALESCE((SELECT SUM(quantity) FROM retail_sale_lines WHERE item_id = i.id), 0) as retail_sold,
    COALESCE((SELECT SUM(quantity) FROM b2b_sale_lines WHERE item_id = i.id), 0) as b2b_sold
FROM items i;
```

### 5.2 Ledger Running Balance
Ledger entries list invoice transactions.
The running balance updates incrementally:
*   **Supplier**: Balance represents the payables due.
    $$\text{Balance}_{n} = \text{Balance}_{n-1} - \text{Debit (Payments)} + \text{Credit (Invoices)}$$
*   **B2B Customer**: Balance represents the receivables due.
    $$\text{Balance}_{n} = \text{Balance}_{n-1} + \text{Credit (Invoices)} - \text{Debit (Payments)}$$
This logic keeps transactions aligned with basic accounting principles.
