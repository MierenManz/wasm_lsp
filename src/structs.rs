pub enum Type {
    I32,
    I64,
    FuncRef(Box<dyn AsRef<Function>>),
}

pub struct Export {
    pub exported_name: String,
    pub exported: Box<dyn AsRef<ModuleItems>>,
}

pub struct Function {
    pub name: Option<String>,
    // pub ptr: usize,
    pub parameters: Option<Vec<Type>>,
    pub locals: Option<Vec<Type>>,
    pub result: Option<Vec<Type>>,
    pub export: Option<Box<Export>>,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            name: None,
            // ptr: usize::MAX,
            parameters: None,
            locals: None,
            result: None,
            export: None,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Memory {
    pub name: Option<String>,
    pub size: usize,
    // pub ptr: usize,
}

pub struct Global {
    pub name: Option<String>,
    pub mutatable: bool,
    pub value: String,
}

pub enum ModuleItems {
    Global(Global),
    Function(Function),
    Memory(Memory),
    Export(Export),
}

pub struct Module {
    pub items: Vec<ModuleItems>,
}

impl Module {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: ModuleItems) {
        self.items.push(item);
    }
}
