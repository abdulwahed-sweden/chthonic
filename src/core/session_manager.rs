#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
// TcpStream مش مستخدمة لسه، لكن بنحطها للاستخدام المستقبلي
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub enum SessionType {
    Shell(TcpStream),
}

pub struct Session {
    pub id: u32,
    pub target: String,
    pub session_type: SessionType,
    pub info: String,
}

pub struct SessionManager {
    sessions: HashMap<u32, Session>,
    next_id: Mutex<u32>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: HashMap::new(),
            next_id: Mutex::new(1),
        }
    }

    pub async fn add_session(&mut self, target: String, session_type: SessionType, info: String) -> u32 {
        let mut id_lock = self.next_id.lock().await;
        let id = *id_lock;
        *id_lock += 1;

        let session = Session {
            id,
            target: target.clone(), // استخدم clone هنا
            session_type,
            info: info.clone(),     // واستخدم clone هنا
        };

        self.sessions.insert(id, session);
        println!("[+] New session {} opened to {}", id, target); // target لسه موجودة بسبب ال clone
        id
    }

    pub fn list_sessions(&self) {
        if self.sessions.is_empty() {
            println!("[-] No active sessions.");
        } else {
            for (id, session) in &self.sessions {
                println!("{} - {} ({})", id, session.target, session.info);
            }
        }
    }
}