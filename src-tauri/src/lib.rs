mod db;
mod commands;

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Find application data directory
            let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
            std::fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
            let db_path = app_data_dir.join("aplus.db");
            
            // Log path to console for debug purposes
            println!("SQLite DB Path: {:?}", db_path);

            // Initialize database
            let conn = db::init_db(db_path).map_err(|e| e.to_string())?;
            app.manage(db::DbState {
                conn: std::sync::Mutex::new(conn),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::get_dashboard_data,
            commands::get_bakery_profile,
            commands::save_bakery_profile,
            commands::get_items,
            commands::save_item,
            commands::delete_item,
            commands::get_businesses,
            commands::save_business,
            commands::delete_business,
            commands::get_purchases,
            commands::save_purchase,
            commands::get_retail_sales,
            commands::save_retail_sale,
            commands::get_b2b_sales,
            commands::save_b2b_sale,
            commands::get_daybook,
            commands::get_stock_status,
            commands::get_business_ledger
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

