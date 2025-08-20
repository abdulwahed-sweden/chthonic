//! Module handler core component for Chthonic framework.
//! Defines the base Module trait and management structures.

use async_trait::async_trait;

/// Base trait that all Chthonic modules must implement.
/// This defines the common interface for exploits, payloads, and auxiliary modules.
#[async_trait]
pub trait Module {
    /// Returns the canonical name of the module (e.g., "exploit/windows/smb/eternalblue")
    fn name(&self) -> &'static str;
    
    /// Returns a brief description of the module's functionality
    fn description(&self) -> &'static str;
    
    /// Returns the author(s) of the module
    fn author(&self) -> &'static str;
    
    /// Returns the module version string
    fn version(&self) -> &'static str;
    
    /// Executes the module's main functionality with provided options
    /// # Arguments
    /// * `options` - Key-value pairs of module configuration options
    async fn run(&self, options: &[(String, String)]) -> Result<String, String>;
}

/// Type alias for boxed modules to simplify storage and handling
pub type ModuleBox = Box<dyn Module + Send + Sync>;

/// Manages registration and retrieval of all available modules
pub struct ModuleHandler {
    modules: std::collections::HashMap<&'static str, ModuleBox>,
}

impl ModuleHandler {
    /// Creates a new empty ModuleHandler
    pub fn new() -> Self {
        ModuleHandler {
            modules: std::collections::HashMap::new(),
        }
    }
    
    /// Registers a new module with the handler
    /// # Arguments
    /// * `name` - The canonical name of the module
    /// * `module` - Boxed module instance
    pub fn register_module(&mut self, name: &'static str, module: ModuleBox) {
        self.modules.insert(name, module);
        println!("[+] Module registered: {}", name);
    }
    
    /// Retrieves a module by name
    /// # Arguments
    /// * `name` - The name of the module to retrieve
    pub fn get_module(&self, name: &str) -> Option<&ModuleBox> {
        self.modules.get(name)
    }
    
    /// Lists all registered modules with their details
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