// src/core/session_manager.rs
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

// نوع الجلسة - ممكن نوسعها بعدين
pub enum SessionType {
    Shell(TcpStream),
    // Meterpreter, VNC, etc. future types
}

// معلومات الجلسة
pub struct Session {
    pub id: u32,
    pub target: String,
    pub session_type: SessionType,
    pub info: String, // معلومات إضافية
}

// الهيكل الرئيسي لمدير الجلسات
pub struct SessionManager {
    sessions: HashMap<u32, Session>,
    next_id: Mutex<u32>, // Mutex علشان async
}

impl SessionManager {
    // إنشاء مدير جلسات جديد
    pub fn new() -> Self {
        SessionManager {
            sessions: HashMap::new(),
            next_id: Mutex::new(1), // يبدأ ID من 1
        }
    }

    // إضافة جلسة جديدة
    pub async fn add_session(&mut self, target: String, session_type: SessionType, info: String) -> u32 {
        let mut id_lock = self.next_id.lock().await;
        let id = *id_lock;
        *id_lock += 1;

        let session = Session {
            id,
            target,
            session_type,
            info,
        };

        self.sessions.insert(id, session);
        println!("[+] New session {} opened to {}", id, target);
        id
    }

    // إدراة الجلسات النشطة
    pub fn list_sessions(&self) {
        if self.sessions.is_empty() {
            println!("[-] No active sessions.");
        } else {
            for (id, session) in &self.sessions {
                println!("{} - {} ({})", id, session.target, session.info);
            }
        }
    }

    // (لاحقًا) دوال لإيجاد جلسة، إغلاق جلسة، إلخ...
    // pub async fn get_session(&self, id: u32) -> Option<&Session> { ... }
    // pub fn kill_session(&mut self, id: u32) { ... }
}