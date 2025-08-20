// src/modules/mod.rs
pub mod exploits;
pub mod auxiliary;
// pub mod payloads;
// pub mod encoders;

// إما: مسح ModuleBox غير المستخدم
use crate::core::module_handler::ModuleHandler;
// أو: استخدام allow لتجاهل التحذير
// #[allow(unused_imports)]
// use crate::core::module_handler::{ModuleHandler, ModuleBox};

pub fn register_all_modules(handler: &mut ModuleHandler) {
    // سجل المستغلات
    handler.register_module(
        "exploit/test_exploit",
        Box::new(exploits::TestExploit),
    );

    // سجل الأدوات المساعدة (Auxiliary)
    handler.register_module(
        "auxiliary/port_scanner",  // تم تصحيح المسافة البادئة
        Box::new(auxiliary::PortScanner::default()),
    );

    // هنا لاحقًا: payloads و encoders
}