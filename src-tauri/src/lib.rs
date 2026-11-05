pub mod crawler;
pub mod embed;
pub mod indexer;
pub mod ingest;
pub mod ipc;
pub mod search;
pub mod store;

use indexer::SearchIndex;
use ipc::AppState;
use std::sync::Arc;
use store::DbStore;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Determine application data directory for SQLite & Tantivy storage
            let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| {
                dirs::data_local_dir()
                    .unwrap_or_else(std::env::temp_dir)
                    .join("private-search")
            });

            std::fs::create_dir_all(&app_data_dir).ok();

            let db = Arc::new(
                DbStore::new(&app_data_dir).expect("Failed to initialize SQLite database"),
            );
            let index = Arc::new(
                SearchIndex::new(&app_data_dir).expect("Failed to initialize Tantivy index"),
            );

            // Seed initial source if none exist
            if let Ok(sources) = db.get_sources() {
                if sources.is_empty() {
                    let _ = db.insert_source(&store::NewSourceRecord {
                        kind: "web".to_string(),
                        uri: "https://artcraft.com".to_string(),
                        label: "ArtCraft UI & Architecture".to_string(),
                        crawl_depth: 2,
                        refresh_hours: 168,
                        enabled: true,
                    });
                }
            }

            app.manage(AppState { db, index });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::search,
            ipc::suggest,
            ipc::get_sources,
            ipc::add_source,
            ipc::toggle_source,
            ipc::delete_source,
            ipc::crawl_now,
            ipc::open_document,
            ipc::pin_document,
            ipc::tag_document,
            ipc::get_stats,
            ipc::add_manual_document,
            ipc::export_library,
            ipc::import_library
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
