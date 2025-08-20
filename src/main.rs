// src/main.rs
mod cli;
mod core;
// ... other mods

#[tokio::main]
async fn main() {
    println!("[+] Chthonic Rising from the Underworld... 🦀☠️");

    // اختبر مدير الجلسات
    let mut manager = core::session_manager::SessionManager::new();
    manager.list_sessions(); // يجب تطبع: No active sessions.

    // (لاحقًا) هنا رح نمرر المدير للـ CLI
    // cli::run(manager).await;
}