// src/core/module_handler.rs
use std::collections::HashMap;
use async_trait::async_trait;

// 1. تعريف "الواجهة" (Trait) لأي وحدة (Module) في نظامنا
#[async_trait]
pub trait Module {
    // اسم الوحدة
    fn name(&self) -> &'static str;
    // وصفها
    fn description(&self) -> &'static str;
    // المؤلف (إنتا 😎)
    fn author(&self) -> &'static str;
    // الإصدار
    fn version(&self) -> &'static str;

    // هذه هي الدالة الأساسية التي ستنفذ الوحدة وظيفتها
    async fn run(&self) -> Result<String, String>; // ترجع Result، إما نجاح (String) أو فشل (String)
}

// 2. نوع لتخزين أي وحدة (ككائن) في HashMap
pub type ModuleBox = Box<dyn Module + Send + Sync>;

// 3. الهيكل الرئيسي الذي يدير جميع الوحدات
pub struct ModuleHandler {
    modules: HashMap<&'static str, ModuleBox>, // الخريطة: [اسم الوحدة] => [الوحدة نفسها]
}

impl ModuleHandler {
    // إنشاء مدير وحدات جديد
    pub fn new() -> Self {
        ModuleHandler {
            modules: HashMap::new(),
        }
    }

    // تسجيل وحدة جديدة (مهم: نستدعي هذه الدالة لكل وحدة نصنعها)
    pub fn register_module(&mut self, name: &'static str, module: ModuleBox) {
        self.modules.insert(name, module);
        println!("[+] Module registered: {}", name);
    }

    // الحصول على وحدة بواسطة اسمها (مهم لأمر `use` لاحقًا)
    pub fn get_module(&self, name: &str) -> Option<&ModuleBox> {
        self.modules.get(name)
    }

    // سرد جميع الوحدات المسجلة (مهم لأمر `show modules` لاحقًا)
    pub fn list_modules(&self) {
        if self.modules.is_empty() {
            println!("[-] No modules registered.");
        } else {
            println!("[+] Available modules:");
            for (name, module) in &self.modules {
                println!("  - {} (v{}) by {}", name, module.version(), module.author());
                println!("    Description: {}", module.description());
            }
        }
    }
}
