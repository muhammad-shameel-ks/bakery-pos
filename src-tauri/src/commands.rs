use crate::db::{
    BakeryProfile, BusinessParty, DashboardData, DayBookRow, DbState, Item, LedgerRow,
    PurchaseHeader, PurchaseLine, RetailSaleHeader, RetailSaleLine, B2BSaleHeader, B2BSaleLine,
    RetailSaleDto, StockStatus, TransactionDto,
};
use rusqlite::{params, Result};
use tauri::State;
use chrono::Local;

// Helper to format SQLite errors for frontend consumption
fn map_err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

// ------------------ DASHBOARD ------------------
#[tauri::command]
pub fn get_dashboard_data(state: State<'_, DbState>) -> Result<DashboardData, String> {
    let conn = state.conn.lock().map_err(map_err)?;
    let today = Local::now().format("%Y-%m-%d").to_string();

    // 1. Today's Retail Sales
    let today_retail: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(grand_total), 0) FROM retail_sale_headers WHERE date = ?",
            [&today],
            |r| r.get(0),
        )
        .map_err(map_err)?;

    // 2. Today's B2B Sales
    let today_b2b: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(grand_total), 0) FROM b2b_sale_headers WHERE date = ?",
            [&today],
            |r| r.get(0),
        )
        .map_err(map_err)?;

    // 3. Today's Purchases
    let today_purchase: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(grand_total), 0) FROM purchase_headers WHERE date = ?",
            [&today],
            |r| r.get(0),
        )
        .map_err(map_err)?;

    // 4. Stock calculations for all items to compute total stock value
    let mut stmt = conn
        .prepare(
            "SELECT 
                i.id, i.item_code, i.item_name, i.category, i.unit, i.opening_stock, i.reorder_level, i.our_price,
                COALESCE((SELECT SUM(quantity) FROM purchase_lines WHERE item_id = i.id), 0) as purchased,
                COALESCE((SELECT SUM(quantity) FROM retail_sale_lines WHERE item_id = i.id), 0) as retail_sold,
                COALESCE((SELECT SUM(quantity) FROM b2b_sale_lines WHERE item_id = i.id), 0) as b2b_sold
             FROM items i",
        )
        .map_err(map_err)?;

    let mut stock_value = 0.0;
    let mut low_stock_items = Vec::new();

    let rows = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            let item_code: String = row.get(1)?;
            let item_name: String = row.get(2)?;
            let category: String = row.get(3)?;
            let unit: String = row.get(4)?;
            let opening_stock: f64 = row.get(5)?;
            let reorder_level: f64 = row.get(6)?;
            let our_price: f64 = row.get(7)?;
            let purchased: f64 = row.get(8)?;
            let retail_sold: f64 = row.get(9)?;
            let b2b_sold: f64 = row.get(10)?;

            let on_hand = opening_stock + purchased - retail_sold - b2b_sold;
            
            Ok((
                our_price * on_hand,
                StockStatus {
                    id,
                    item_code,
                    item_name,
                    category,
                    unit,
                    opening_stock,
                    purchases: purchased,
                    retail_sales: retail_sold,
                    b2b_sales: b2b_sold,
                    on_hand,
                    reorder_level,
                },
            ))
        })
        .map_err(map_err)?;

    for r in rows {
        let (val, status) = r.map_err(map_err)?;
        stock_value += val;
        if status.on_hand <= status.reorder_level {
            low_stock_items.push(status);
        }
    }

    // 5. Weekly Sales (last 7 days)
    let mut weekly_sales = Vec::new();
    for i in (0..7).rev() {
        let day = Local::now() - chrono::Duration::days(i);
        let day_str = day.format("%Y-%m-%d").to_string();

        let retail: f64 = conn
            .query_row(
                "SELECT COALESCE(SUM(grand_total), 0) FROM retail_sale_headers WHERE date = ?",
                [&day_str],
                |r| r.get(0),
            )
            .unwrap_or(0.0);

        let b2b: f64 = conn
            .query_row(
                "SELECT COALESCE(SUM(grand_total), 0) FROM b2b_sale_headers WHERE date = ?",
                [&day_str],
                |r| r.get(0),
            )
            .unwrap_or(0.0);

        weekly_sales.push(crate::db::WeeklySales {
            date: day_str,
            retail,
            b2b,
        });
    }

    Ok(DashboardData {
        today_retail,
        today_b2b,
        today_purchase,
        stock_value,
        low_stock_items,
        weekly_sales,
    })
}

// ------------------ BAKERY PROFILE ------------------
#[tauri::command]
pub fn get_bakery_profile(state: State<'_, DbState>) -> Result<BakeryProfile, String> {
    let conn = state.conn.lock().map_err(map_err)?;
    let profile = conn.query_row(
        "SELECT id, name, gstin, address, phone, email, logo_base64, invoice_note FROM bakery_profile WHERE id = 1",
        [],
        |row| {
            Ok(BakeryProfile {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                gstin: row.get(2)?,
                address: row.get(3)?,
                phone: row.get(4)?,
                email: row.get(5)?,
                logo_base64: row.get(6)?,
                invoice_note: row.get(7)?,
            })
        },
    );

    match profile {
        Ok(p) => Ok(p),
        Err(_) => Ok(BakeryProfile {
            id: Some(1),
            name: "A Plus Bakery".to_string(),
            gstin: "".to_string(),
            address: "".to_string(),
            phone: "".to_string(),
            email: "".to_string(),
            logo_base64: "".to_string(),
            invoice_note: "".to_string(),
        }),
    }
}

#[tauri::command]
pub fn save_bakery_profile(state: State<'_, DbState>, profile: BakeryProfile) -> Result<(), String> {
    let conn = state.conn.lock().map_err(map_err)?;
    conn.execute(
        "INSERT OR REPLACE INTO bakery_profile (id, name, gstin, address, phone, email, logo_base64, invoice_note)
         VALUES (1, ?, ?, ?, ?, ?, ?, ?)",
        params![
            profile.name,
            profile.gstin,
            profile.address,
            profile.phone,
            profile.email,
            profile.logo_base64,
            profile.invoice_note
        ],
    )
    .map_err(map_err)?;
    Ok(())
}

// ------------------ ITEMS MASTER ------------------
#[tauri::command]
pub fn get_items(state: State<'_, DbState>, search_query: String) -> Result<Vec<Item>, String> {
    let conn = state.conn.lock().map_err(map_err)?;
    let mut result = Vec::new();

    if search_query.trim().is_empty() {
        let mut stmt = conn
            .prepare("SELECT id, item_code, item_name, category, alias, hsnc, tax_slab, mrp, our_price, opening_stock, reorder_level, unit FROM items ORDER BY item_name ASC")
            .map_err(map_err)?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Item {
                    id: Some(row.get(0)?),
                    item_code: row.get(1)?,
                    item_name: row.get(2)?,
                    category: row.get(3)?,
                    alias: row.get(4)?,
                    hsnc: row.get(5)?,
                    tax_slab: row.get(6)?,
                    mrp: row.get(7)?,
                    our_price: row.get(8)?,
                    opening_stock: row.get(9)?,
                    reorder_level: row.get(10)?,
                    unit: row.get(11)?,
                })
            })
            .map_err(map_err)?;

        for r in rows {
            result.push(r.map_err(map_err)?);
        }
    } else {
        let mut stmt = conn
            .prepare(
                "SELECT id, item_code, item_name, category, alias, hsnc, tax_slab, mrp, our_price, opening_stock, reorder_level, unit 
                 FROM items 
                 WHERE item_name LIKE ?1 OR item_code LIKE ?1 OR alias LIKE ?1
                 ORDER BY item_name ASC",
            )
            .map_err(map_err)?;
        let param = format!("%{}%", search_query);
        let rows = stmt
            .query_map([param], |row| {
                Ok(Item {
                    id: Some(row.get(0)?),
                    item_code: row.get(1)?,
                    item_name: row.get(2)?,
                    category: row.get(3)?,
                    alias: row.get(4)?,
                    hsnc: row.get(5)?,
                    tax_slab: row.get(6)?,
                    mrp: row.get(7)?,
                    our_price: row.get(8)?,
                    opening_stock: row.get(9)?,
                    reorder_level: row.get(10)?,
                    unit: row.get(11)?,
                })
            })
            .map_err(map_err)?;

        for r in rows {
            result.push(r.map_err(map_err)?);
        }
    }

    Ok(result)
}

#[tauri::command]
pub fn save_item(state: State<'_, DbState>, item: Item) -> Result<(), String> {
    let conn = state.conn.lock().map_err(map_err)?;
    if let Some(id) = item.id {
        if id > 0 {
            conn.execute(
                "UPDATE items SET item_code=?, item_name=?, category=?, alias=?, hsnc=?, tax_slab=?, mrp=?, our_price=?, opening_stock=?, reorder_level=?, unit=? WHERE id=?",
                params![
                    item.item_code,
                    item.item_name,
                    item.category,
                    item.alias,
                    item.hsnc,
                    item.tax_slab,
                    item.mrp,
                    item.our_price,
                    item.opening_stock,
                    item.reorder_level,
                    item.unit,
                    id
                ],
            )
            .map_err(map_err)?;
            return Ok(());
        }
    }

    conn.execute(
        "INSERT INTO items (item_code, item_name, category, alias, hsnc, tax_slab, mrp, our_price, opening_stock, reorder_level, unit)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            item.item_code,
            item.item_name,
            item.category,
            item.alias,
            item.hsnc,
            item.tax_slab,
            item.mrp,
            item.our_price,
            item.opening_stock,
            item.reorder_level,
            item.unit
        ],
    )
    .map_err(map_err)?;

    Ok(())
}

#[tauri::command]
pub fn delete_item(state: State<'_, DbState>, id: i32) -> Result<(), String> {
    let conn = state.conn.lock().map_err(map_err)?;
    conn.execute("DELETE FROM items WHERE id = ?", [id])
        .map_err(map_err)?;
    Ok(())
}

// ------------------ BUSINESS PARTIES ------------------
#[tauri::command]
pub fn get_businesses(
    state: State<'_, DbState>,
    party_type: String,
    search_query: String,
) -> Result<Vec<BusinessParty>, String> {
    let conn = state.conn.lock().map_err(map_err)?;
    let mut result = Vec::new();

    if search_query.trim().is_empty() {
        let mut stmt = conn
            .prepare("SELECT id, party_type, business_name, address, contact_person, phone, gstin, opening_balance FROM business_parties WHERE party_type = ? ORDER BY business_name ASC")
            .map_err(map_err)?;
        let rows = stmt
            .query_map([party_type], |row| {
                Ok(BusinessParty {
                    id: Some(row.get(0)?),
                    party_type: row.get(1)?,
                    business_name: row.get(2)?,
                    address: row.get(3)?,
                    contact_person: row.get(4)?,
                    phone: row.get(5)?,
                    gstin: row.get(6)?,
                    opening_balance: row.get(7)?,
                })
            })
            .map_err(map_err)?;

        for r in rows {
            result.push(r.map_err(map_err)?);
        }
    } else {
        let mut stmt = conn
            .prepare(
                "SELECT id, party_type, business_name, address, contact_person, phone, gstin, opening_balance 
                 FROM business_parties 
                 WHERE party_type = ?1 AND (business_name LIKE ?2 OR contact_person LIKE ?2 OR phone LIKE ?2)
                 ORDER BY business_name ASC",
            )
            .map_err(map_err)?;
        let param = format!("%{}%", search_query);
        let rows = stmt
            .query_map([party_type, param], |row| {
                Ok(BusinessParty {
                    id: Some(row.get(0)?),
                    party_type: row.get(1)?,
                    business_name: row.get(2)?,
                    address: row.get(3)?,
                    contact_person: row.get(4)?,
                    phone: row.get(5)?,
                    gstin: row.get(6)?,
                    opening_balance: row.get(7)?,
                })
            })
            .map_err(map_err)?;

        for r in rows {
            result.push(r.map_err(map_err)?);
        }
    }

    Ok(result)
}


#[tauri::command]
pub fn save_business(state: State<'_, DbState>, business: BusinessParty) -> Result<(), String> {
    let conn = state.conn.lock().map_err(map_err)?;
    if let Some(id) = business.id {
        if id > 0 {
            conn.execute(
                "UPDATE business_parties SET party_type=?, business_name=?, address=?, contact_person=?, phone=?, gstin=?, opening_balance=? WHERE id=?",
                params![
                    business.party_type,
                    business.business_name,
                    business.address,
                    business.contact_person,
                    business.phone,
                    business.gstin,
                    business.opening_balance,
                    id
                ],
            )
            .map_err(map_err)?;
            return Ok(());
        }
    }

    conn.execute(
        "INSERT INTO business_parties (party_type, business_name, address, contact_person, phone, gstin, opening_balance)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            business.party_type,
            business.business_name,
            business.address,
            business.contact_person,
            business.phone,
            business.gstin,
            business.opening_balance
        ],
    )
    .map_err(map_err)?;

    Ok(())
}

#[tauri::command]
pub fn delete_business(state: State<'_, DbState>, id: i32) -> Result<(), String> {
    let conn = state.conn.lock().map_err(map_err)?;
    conn.execute("DELETE FROM business_parties WHERE id = ?", [id])
        .map_err(map_err)?;
    Ok(())
}

// ------------------ PURCHASES ------------------
#[tauri::command]
pub fn get_purchases(state: State<'_, DbState>) -> Result<Vec<PurchaseHeader>, String> {
    let mut conn = state.conn.lock().map_err(map_err)?;
    let tx = conn.transaction().map_err(map_err)?;

    let mut stmt = tx
        .prepare(
            "SELECT h.id, h.invoice_no, h.date, h.business_party_id, p.business_name, h.payment_mode, h.notes, h.subtotal, h.tax_total, h.grand_total, h.paid_amount
             FROM purchase_headers h
             JOIN business_parties p ON h.business_party_id = p.id
             ORDER BY h.date DESC, h.id DESC",
        )
        .map_err(map_err)?;

    let headers = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            Ok(PurchaseHeader {
                id,
                invoice_no: row.get(1)?,
                date: row.get(2)?,
                business_party_id: row.get(3)?,
                business_name: row.get(4)?,
                payment_mode: row.get(5)?,
                notes: row.get(6).unwrap_or_default(),
                subtotal: row.get(7)?,
                tax_total: row.get(8)?,
                grand_total: row.get(9)?,
                paid_amount: row.get(10)?,
                lines: Vec::new(),
            })
        })
        .map_err(map_err)?;

    let mut result = Vec::new();
    for h in headers {
        let mut header = h.map_err(map_err)?;
        
        let mut line_stmt = tx
            .prepare(
                "SELECT l.id, l.purchase_header_id, l.item_id, i.item_name, i.item_code, l.quantity, l.rate, l.tax_rate
                 FROM purchase_lines l
                 JOIN items i ON l.item_id = i.id
                 WHERE l.purchase_header_id = ?",
            )
            .map_err(map_err)?;
        
        let lines = line_stmt
            .query_map([header.id], |line_row| {
                Ok(PurchaseLine {
                    id: line_row.get(0)?,
                    purchase_header_id: line_row.get(1)?,
                    item_id: line_row.get(2)?,
                    item_name: line_row.get(3)?,
                    item_code: line_row.get(4)?,
                    quantity: line_row.get(5)?,
                    rate: line_row.get(6)?,
                    tax_rate: line_row.get(7)?,
                })
            })
            .map_err(map_err)?;
        
        for l in lines {
            header.lines.push(l.map_err(map_err)?);
        }
        result.push(header);
    }

    Ok(result)
}

#[tauri::command]
pub fn save_purchase(state: State<'_, DbState>, purchase: TransactionDto) -> Result<(), String> {
    let mut conn = state.conn.lock().map_err(map_err)?;
    let tx = conn.transaction().map_err(map_err)?;

    // Calculate totals
    let mut subtotal = 0.0;
    let mut tax_total = 0.0;
    for line in &purchase.lines {
        let amount = line.quantity * line.rate;
        let tax = amount * (line.tax_rate / 100.0);
        subtotal += amount;
        tax_total += tax;
    }
    let grand_total = subtotal + tax_total;

    // Insert Header
    tx.execute(
        "INSERT INTO purchase_headers (invoice_no, date, business_party_id, payment_mode, notes, subtotal, tax_total, grand_total, paid_amount)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            purchase.invoice_no,
            purchase.date,
            purchase.business_party_id,
            purchase.payment_mode,
            purchase.notes,
            subtotal,
            tax_total,
            grand_total,
            purchase.paid_or_received
        ],
    )
    .map_err(map_err)?;

    let header_id = tx.last_insert_rowid();

    // Insert Lines
    for line in &purchase.lines {
        tx.execute(
            "INSERT INTO purchase_lines (purchase_header_id, item_id, quantity, rate, tax_rate)
             VALUES (?, ?, ?, ?, ?)",
            params![
                header_id,
                line.item_id,
                line.quantity,
                line.rate,
                line.tax_rate
            ],
        )
        .map_err(map_err)?;
    }

    tx.commit().map_err(map_err)?;
    Ok(())
}

// ------------------ RETAIL SALES ------------------
#[tauri::command]
pub fn get_retail_sales(state: State<'_, DbState>) -> Result<Vec<RetailSaleHeader>, String> {
    let mut conn = state.conn.lock().map_err(map_err)?;
    let tx = conn.transaction().map_err(map_err)?;

    let mut stmt = tx
        .prepare(
            "SELECT id, invoice_no, date, customer_name, payment_mode, subtotal, tax_total, grand_total, received_amount
             FROM retail_sale_headers
             ORDER BY date DESC, id DESC",
        )
        .map_err(map_err)?;

    let headers = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            Ok(RetailSaleHeader {
                id,
                invoice_no: row.get(1)?,
                date: row.get(2)?,
                customer_name: row.get(3)?,
                payment_mode: row.get(4)?,
                subtotal: row.get(5)?,
                tax_total: row.get(6)?,
                grand_total: row.get(7)?,
                received_amount: row.get(8)?,
                lines: Vec::new(),
            })
        })
        .map_err(map_err)?;

    let mut result = Vec::new();
    for h in headers {
        let mut header = h.map_err(map_err)?;
        
        let mut line_stmt = tx
            .prepare(
                "SELECT l.id, l.retail_sale_header_id, l.item_id, i.item_name, i.item_code, l.quantity, l.rate, l.tax_rate
                 FROM retail_sale_lines l
                 JOIN items i ON l.item_id = i.id
                 WHERE l.retail_sale_header_id = ?",
            )
            .map_err(map_err)?;
        
        let lines = line_stmt
            .query_map([header.id], |line_row| {
                Ok(RetailSaleLine {
                    id: line_row.get(0)?,
                    retail_sale_header_id: line_row.get(1)?,
                    item_id: line_row.get(2)?,
                    item_name: line_row.get(3)?,
                    item_code: line_row.get(4)?,
                    quantity: line_row.get(5)?,
                    rate: line_row.get(6)?,
                    tax_rate: line_row.get(7)?,
                })
            })
            .map_err(map_err)?;
        
        for l in lines {
            header.lines.push(l.map_err(map_err)?);
        }
        result.push(header);
    }

    Ok(result)
}

#[tauri::command]
pub fn save_retail_sale(state: State<'_, DbState>, sale: RetailSaleDto) -> Result<(), String> {
    let mut conn = state.conn.lock().map_err(map_err)?;
    let tx = conn.transaction().map_err(map_err)?;

    // Calculate totals
    let mut subtotal = 0.0;
    let mut tax_total = 0.0;
    for line in &sale.lines {
        let amount = line.quantity * line.rate;
        let tax = amount * (line.tax_rate / 100.0);
        subtotal += amount;
        tax_total += tax;
    }
    let grand_total = subtotal + tax_total;

    // Insert Header
    tx.execute(
        "INSERT INTO retail_sale_headers (invoice_no, date, customer_name, payment_mode, subtotal, tax_total, grand_total, received_amount)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            sale.invoice_no,
            sale.date,
            sale.customer_name,
            sale.payment_mode,
            subtotal,
            tax_total,
            grand_total,
            sale.received_amount
        ],
    )
    .map_err(map_err)?;

    let header_id = tx.last_insert_rowid();

    // Insert Lines
    for line in &sale.lines {
        tx.execute(
            "INSERT INTO retail_sale_lines (retail_sale_header_id, item_id, quantity, rate, tax_rate)
             VALUES (?, ?, ?, ?, ?)",
            params![
                header_id,
                line.item_id,
                line.quantity,
                line.rate,
                line.tax_rate
            ],
        )
        .map_err(map_err)?;
    }

    tx.commit().map_err(map_err)?;
    Ok(())
}

// ------------------ B2B SALES ------------------
#[tauri::command]
pub fn get_b2b_sales(state: State<'_, DbState>) -> Result<Vec<B2BSaleHeader>, String> {
    let mut conn = state.conn.lock().map_err(map_err)?;
    let tx = conn.transaction().map_err(map_err)?;

    let mut stmt = tx
        .prepare(
            "SELECT h.id, h.invoice_no, h.date, h.business_party_id, p.business_name, p.gstin, p.address, h.payment_mode, h.notes, h.subtotal, h.tax_total, h.grand_total, h.received_amount
             FROM b2b_sale_headers h
             JOIN business_parties p ON h.business_party_id = p.id
             ORDER BY h.date DESC, h.id DESC",
        )
        .map_err(map_err)?;

    let headers = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            Ok(B2BSaleHeader {
                id,
                invoice_no: row.get(1)?,
                date: row.get(2)?,
                business_party_id: row.get(3)?,
                business_name: row.get(4)?,
                business_gstin: row.get(5).unwrap_or_default(),
                business_address: row.get(6).unwrap_or_default(),
                payment_mode: row.get(7)?,
                notes: row.get(8).unwrap_or_default(),
                subtotal: row.get(9)?,
                tax_total: row.get(10)?,
                grand_total: row.get(11)?,
                received_amount: row.get(12)?,
                lines: Vec::new(),
            })
        })
        .map_err(map_err)?;

    let mut result = Vec::new();
    for h in headers {
        let mut header = h.map_err(map_err)?;
        
        let mut line_stmt = tx
            .prepare(
                "SELECT l.id, l.b2b_sale_header_id, l.item_id, i.item_name, i.item_code, l.quantity, l.rate, l.tax_rate
                 FROM b2b_sale_lines l
                 JOIN items i ON l.item_id = i.id
                 WHERE l.b2b_sale_header_id = ?",
            )
            .map_err(map_err)?;
        
        let lines = line_stmt
            .query_map([header.id], |line_row| {
                Ok(B2BSaleLine {
                    id: line_row.get(0)?,
                    b2b_sale_header_id: line_row.get(1)?,
                    item_id: line_row.get(2)?,
                    item_name: line_row.get(3)?,
                    item_code: line_row.get(4)?,
                    quantity: line_row.get(5)?,
                    rate: line_row.get(6)?,
                    tax_rate: line_row.get(7)?,
                })
            })
            .map_err(map_err)?;
        
        for l in lines {
            header.lines.push(l.map_err(map_err)?);
        }
        result.push(header);
    }

    Ok(result)
}

#[tauri::command]
pub fn save_b2b_sale(state: State<'_, DbState>, sale: TransactionDto) -> Result<(), String> {
    let mut conn = state.conn.lock().map_err(map_err)?;
    let tx = conn.transaction().map_err(map_err)?;

    // Calculate totals
    let mut subtotal = 0.0;
    let mut tax_total = 0.0;
    for line in &sale.lines {
        let amount = line.quantity * line.rate;
        let tax = amount * (line.tax_rate / 100.0);
        subtotal += amount;
        tax_total += tax;
    }
    let grand_total = subtotal + tax_total;

    // Insert Header
    tx.execute(
        "INSERT INTO b2b_sale_headers (invoice_no, date, business_party_id, payment_mode, notes, subtotal, tax_total, grand_total, received_amount)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            sale.invoice_no,
            sale.date,
            sale.business_party_id,
            sale.payment_mode,
            sale.notes,
            subtotal,
            tax_total,
            grand_total,
            sale.paid_or_received
        ],
    )
    .map_err(map_err)?;

    let header_id = tx.last_insert_rowid();

    // Insert Lines
    for line in &sale.lines {
        tx.execute(
            "INSERT INTO b2b_sale_lines (b2b_sale_header_id, item_id, quantity, rate, tax_rate)
             VALUES (?, ?, ?, ?, ?)",
            params![
                header_id,
                line.item_id,
                line.quantity,
                line.rate,
                line.tax_rate
            ],
        )
        .map_err(map_err)?;
    }

    tx.commit().map_err(map_err)?;
    Ok(())
}

// ------------------ DAY BOOK ------------------
#[tauri::command]
pub fn get_daybook(
    state: State<'_, DbState>,
    from_date: String,
    to_date: String,
) -> Result<Vec<DayBookRow>, String> {
    let conn = state.conn.lock().map_err(map_err)?;

    let mut result = Vec::new();

    // 1. Purchase payments: Debit = paid_amount
    let mut p_stmt = conn
        .prepare(
            "SELECT h.date, h.invoice_no, p.business_name, h.payment_mode, h.paid_amount
             FROM purchase_headers h
             JOIN business_parties p ON h.business_party_id = p.id
             WHERE h.date >= ?1 AND h.date <= ?2 AND h.paid_amount > 0",
        )
        .map_err(map_err)?;

    let p_rows = p_stmt
        .query_map([&from_date, &to_date], |row| {
            Ok(DayBookRow {
                date: row.get(0)?,
                module: "Purchase Inward".to_string(),
                ref_no: row.get(1)?,
                party_name: row.get(2)?,
                payment_mode: row.get(3)?,
                debit: row.get(4)?,
                credit: 0.0,
            })
        })
        .map_err(map_err)?;

    for r in p_rows {
        result.push(r.map_err(map_err)?);
    }

    // 2. Retail sales receipts: Credit = received_amount
    let mut r_stmt = conn
        .prepare(
            "SELECT date, invoice_no, customer_name, payment_mode, received_amount
             FROM retail_sale_headers
             WHERE date >= ?1 AND date <= ?2 AND received_amount > 0",
        )
        .map_err(map_err)?;

    let r_rows = r_stmt
        .query_map([&from_date, &to_date], |row| {
            Ok(DayBookRow {
                date: row.get(0)?,
                module: "Retail Sale".to_string(),
                ref_no: row.get(1)?,
                party_name: row.get(2)?,
                payment_mode: row.get(3)?,
                debit: 0.0,
                credit: row.get(4)?,
            })
        })
        .map_err(map_err)?;

    for r in r_rows {
        result.push(r.map_err(map_err)?);
    }

    // 3. B2B sales receipts: Credit = received_amount
    let mut b_stmt = conn
        .prepare(
            "SELECT h.date, h.invoice_no, p.business_name, h.payment_mode, h.received_amount
             FROM b2b_sale_headers h
             JOIN business_parties p ON h.business_party_id = p.id
             WHERE h.date >= ?1 AND h.date <= ?2 AND h.received_amount > 0",
        )
        .map_err(map_err)?;

    let b_rows = b_stmt
        .query_map([&from_date, &to_date], |row| {
            Ok(DayBookRow {
                date: row.get(0)?,
                module: "B2B Sale".to_string(),
                ref_no: row.get(1)?,
                party_name: row.get(2)?,
                payment_mode: row.get(3)?,
                debit: 0.0,
                credit: row.get(4)?,
            })
        })
        .map_err(map_err)?;

    for r in b_rows {
        result.push(r.map_err(map_err)?);
    }

    // Sort by date ascending, then module, then invoice_no
    result.sort_by(|a, b| {
        a.date
            .cmp(&b.date)
            .then_with(|| a.module.cmp(&b.module))
            .then_with(|| a.ref_no.cmp(&b.ref_no))
    });

    Ok(result)
}

// ------------------ STOCK VIEW ------------------
#[tauri::command]
pub fn get_stock_status(state: State<'_, DbState>) -> Result<Vec<StockStatus>, String> {
    let conn = state.conn.lock().map_err(map_err)?;
    let mut stmt = conn
        .prepare(
            "SELECT 
                i.id, i.item_code, i.item_name, i.category, i.unit, i.opening_stock, i.reorder_level,
                COALESCE((SELECT SUM(quantity) FROM purchase_lines WHERE item_id = i.id), 0) as purchased,
                COALESCE((SELECT SUM(quantity) FROM retail_sale_lines WHERE item_id = i.id), 0) as retail_sold,
                COALESCE((SELECT SUM(quantity) FROM b2b_sale_lines WHERE item_id = i.id), 0) as b2b_sold
             FROM items i
             ORDER BY i.item_name ASC",
        )
        .map_err(map_err)?;

    let rows = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            let item_code: String = row.get(1)?;
            let item_name: String = row.get(2)?;
            let category: String = row.get(3)?;
            let unit: String = row.get(4)?;
            let opening_stock: f64 = row.get(5)?;
            let reorder_level: f64 = row.get(6)?;
            let purchased: f64 = row.get(7)?;
            let retail_sold: f64 = row.get(8)?;
            let b2b_sold: f64 = row.get(9)?;

            let on_hand = opening_stock + purchased - retail_sold - b2b_sold;

            Ok(StockStatus {
                id,
                item_code,
                item_name,
                category,
                unit,
                opening_stock,
                purchases: purchased,
                retail_sales: retail_sold,
                b2b_sales: b2b_sold,
                on_hand,
                reorder_level,
            })
        })
        .map_err(map_err)?;

    let mut result = Vec::new();
    for r in rows {
        result.push(r.map_err(map_err)?);
    }
    Ok(result)
}

// ------------------ BUSINESS LEDGER ------------------
#[tauri::command]
pub fn get_business_ledger(state: State<'_, DbState>, party_id: i32) -> Result<Vec<LedgerRow>, String> {
    let conn = state.conn.lock().map_err(map_err)?;

    // Get party opening balance and type
    let (opening_balance, party_type): (f64, String) = conn
        .query_row(
            "SELECT opening_balance, party_type FROM business_parties WHERE id = ?",
            [party_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(map_err)?;

    let mut rows = Vec::new();

    // 1. Opening Balance Row
    // If negative (supplier), we owe them. If positive (customer), they owe us.
    let (op_debit, op_credit) = if opening_balance > 0.0 {
        (opening_balance, 0.0)
    } else {
        (0.0, -opening_balance)
    };

    rows.push(LedgerRow {
        date: "".to_string(),
        transaction_type: "Opening Balance".to_string(),
        ref_no: "".to_string(),
        debit: op_debit,
        credit: op_credit,
        balance: opening_balance,
    });

    // 2. Fetch all Purchases from this party
    let mut p_stmt = conn
        .prepare(
            "SELECT date, invoice_no, grand_total, paid_amount
             FROM purchase_headers
             WHERE business_party_id = ?
             ORDER BY date ASC, id ASC",
        )
        .map_err(map_err)?;

    let p_rows = p_stmt
        .query_map([party_id], |row| {
            let grand_total: f64 = row.get(2)?;
            let paid_amount: f64 = row.get(3)?;
            // Net balance increase of payables = grand_total - paid_amount (we owe them more)
            // Let's record Debit = grand_total - paid_amount, Credit = 0
            Ok(LedgerRow {
                date: row.get(0)?,
                transaction_type: "Purchase".to_string(),
                ref_no: row.get(1)?,
                debit: grand_total - paid_amount,
                credit: 0.0,
                balance: 0.0,
            })
        })
        .map_err(map_err)?;

    for r in p_rows {
        rows.push(r.map_err(map_err)?);
    }

    // 3. Fetch all B2B Sales to this party
    let mut s_stmt = conn
        .prepare(
            "SELECT date, invoice_no, grand_total, received_amount
             FROM b2b_sale_headers
             WHERE business_party_id = ?
             ORDER BY date ASC, id ASC",
        )
        .map_err(map_err)?;

    let s_rows = s_stmt
        .query_map([party_id], |row| {
            let grand_total: f64 = row.get(2)?;
            let received_amount: f64 = row.get(3)?;
            // Net balance increase of receivables = grand_total - received_amount (they owe us more)
            // Let's record Credit = grand_total - received_amount (receivables decrease credit outstanding, or we can represent B2B sale as Credit and payments as Debit depending on perspective.
            // Let's follow: Debit = grand_total - received_amount if we represent the amount they owe us, or follow the KT:
            // "Each B2B sale: Credit = GrandTotal - ReceivedAmount (receivables)"
            // Let's output Credit = grand_total - received_amount exactly as in KT!
            Ok(LedgerRow {
                date: row.get(0)?,
                transaction_type: "B2B Sale".to_string(),
                ref_no: row.get(1)?,
                debit: 0.0,
                credit: grand_total - received_amount,
                balance: 0.0,
            })
        })
        .map_err(map_err)?;

    for r in s_rows {
        rows.push(r.map_err(map_err)?);
    }

    // Sort transactions by date (excluding the first row which is opening balance)
    let mut txs = rows.split_off(1);
    txs.sort_by(|a, b| a.date.cmp(&b.date));

    // Re-assemble and compute running balance
    let mut current_balance = opening_balance;
    let mut final_rows = vec![rows.remove(0)];

    for mut r in txs {
        if party_type == "Supplier" {
            // For suppliers, purchases increase what we owe them (debit increases balance, credit reduces it)
            // We represent balance as a negative number when we owe them (Golden Flour Mills starts at -5000)
            // Wait: purchase outstanding is grand_total - paid_amount. It is added as Debit.
            // If we owe them more, does the negative balance become more negative?
            // E.g. -5000 + (Debit - Credit). If Debit = 1000, then -5000 + 1000 = -4000? No, if we owe them 5000 and purchase 1000 more, we owe them 6000.
            // So for Supplier: running_balance = current_balance - Debit + Credit (if we treat credit as payment/reduction and debit as invoice amount? Wait, the KT says:
            // "Each purchase: Debit = GrandTotal - PaidAmount (payables)".
            // So if we have Debit = GrandTotal - PaidAmount, it increases payables.
            // Let's make the running balance equal:
            // Supplier: running_balance = current_balance - r.debit + r.credit (so -5000 - 1000 = -6000).
            // Customer: running_balance = current_balance + r.credit - r.debit?
            // Wait, let's look at B2B sale outstanding: B2B sale adds receivables. Customer owes us more.
            // B2B Customer opening outstanding is positive: 1500 (they owe us).
            // KT says: "Each B2B sale: Credit = GrandTotal - ReceivedAmount (receivables)".
            // So if they buy 1000 worth B2B, they owe us 1000 more. If Credit = 1000, then running_balance = current_balance + Credit - Debit (so 1500 + 1000 = 2500).
            // In both cases:
            // Supplier: running_balance = current_balance - r.debit + r.credit (decreases with debit, increases with credit, keeping it negative as payable)
            // Customer: running_balance = current_balance + r.credit - r.debit (increases with credit, decreases with debit, keeping it positive as receivable)
            // Wait, this means:
            // For Supplier: outstanding (payables) = -running_balance.
            // For Customer: outstanding (receivables) = running_balance.
            // Let's use this exact math!
            current_balance = current_balance - r.debit + r.credit;
        } else {
            // Customer
            current_balance = current_balance + r.credit - r.debit;
        }
        r.balance = current_balance;
        final_rows.push(r);
    }

    Ok(final_rows)
}

#[tauri::command]
pub fn select_export_path(default_name: String) -> Option<String> {
    let result = rfd::FileDialog::new()
        .set_file_name(&default_name)
        .add_filter("Excel Worksheet", &["xlsx"])
        .save_file();
    result.map(|path| path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn write_binary_file(path: String, data: Vec<u8>) -> Result<(), String> {
    std::fs::write(&path, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_all_data(state: State<'_, DbState>) -> Result<(), String> {
    let mut conn = state.conn.lock().map_err(map_err)?;
    let tx = conn.transaction().map_err(map_err)?;
    
    tx.execute("DELETE FROM purchase_lines", []).map_err(map_err)?;
    tx.execute("DELETE FROM purchase_headers", []).map_err(map_err)?;
    tx.execute("DELETE FROM retail_sale_lines", []).map_err(map_err)?;
    tx.execute("DELETE FROM retail_sale_headers", []).map_err(map_err)?;
    tx.execute("DELETE FROM b2b_sale_lines", []).map_err(map_err)?;
    tx.execute("DELETE FROM b2b_sale_headers", []).map_err(map_err)?;
    tx.execute("DELETE FROM items", []).map_err(map_err)?;
    tx.execute("DELETE FROM business_parties", []).map_err(map_err)?;
    
    tx.execute("DELETE FROM sqlite_sequence WHERE name IN ('purchase_headers', 'purchase_lines', 'retail_sale_headers', 'retail_sale_lines', 'b2b_sale_headers', 'b2b_sale_lines', 'items', 'business_parties')", []).map_err(map_err)?;
    
    tx.commit().map_err(map_err)?;
    Ok(())
}

#[tauri::command]
pub fn reset_to_factory(state: State<'_, DbState>) -> Result<(), String> {
    let mut conn = state.conn.lock().map_err(map_err)?;
    let tx = conn.transaction().map_err(map_err)?;
    
    tx.execute("DELETE FROM purchase_lines", []).map_err(map_err)?;
    tx.execute("DELETE FROM purchase_headers", []).map_err(map_err)?;
    tx.execute("DELETE FROM retail_sale_lines", []).map_err(map_err)?;
    tx.execute("DELETE FROM retail_sale_headers", []).map_err(map_err)?;
    tx.execute("DELETE FROM b2b_sale_lines", []).map_err(map_err)?;
    tx.execute("DELETE FROM b2b_sale_headers", []).map_err(map_err)?;
    tx.execute("DELETE FROM items", []).map_err(map_err)?;
    tx.execute("DELETE FROM business_parties", []).map_err(map_err)?;
    tx.execute("DELETE FROM bakery_profile", []).map_err(map_err)?;
    
    tx.execute("DELETE FROM sqlite_sequence WHERE name IN ('purchase_headers', 'purchase_lines', 'retail_sale_headers', 'retail_sale_lines', 'b2b_sale_headers', 'b2b_sale_lines', 'items', 'business_parties', 'bakery_profile')", []).map_err(map_err)?;
    
    crate::db::seed_data(&tx).map_err(map_err)?;
    
    tx.commit().map_err(map_err)?;
    Ok(())
}
