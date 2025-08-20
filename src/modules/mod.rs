// src/modules/mod.rs
pub mod exploits;
pub mod auxiliary; // تأكد من فك التعليق أو وجود هذا السطر
// pub mod payloads;
// pub mod encoders;

use crate::core::module_handler::{ModuleHandler, ModuleBox};

pub fn register_all_modules(handler: &mut ModuleHandler) {
    // سجل المستغلات
    handler.register_module(
        "exploit/test_exploit",
        Box::new(exploits::TestExploit),
    );

    // سجل الأدوات المساعدة (Auxiliary)
    handler.register_module(
        "auxiliary/port_scanner",
        Box::new(auxiliary::PortScanner), // أضف هذا السطر
    );

    // هنا لاحقًا: payloads و encoders
}