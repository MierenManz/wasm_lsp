pub enum Type<'a> {
    I32,
    I64,
    FuncRef(Box<dyn AsRef<Function<'a>>>),
}

pub struct Export<'a> {
    pub exported_name: &'a str,
    pub export: Box<dyn AsRef<ModuleItems<'a>>>,
}

pub struct Function<'a> {
    pub name: Option<&'a str>,
    pub span: (usize, usize),
    pub ptr: usize,
    pub parameters: Option<Vec<Type<'a>>>,
    pub locals: Option<Vec<Type<'a>>>,
    pub result: Option<Vec<Type<'a>>>,
    pub export: Option<Box<Export<'a>>>,
}

pub struct Memory<'a> {
    pub name: Option<&'a str>,
    pub span: (usize, usize),
    pub size: usize,
    pub ptr: usize,
}

pub enum ModuleItems<'a> {
    Function(Function<'a>),
    Memory(Memory<'a>),
    Export(Export<'a>),
}

pub struct Module<'a> {
    pub items: Vec<ModuleItems<'a>>,
    pub span: (usize, usize),
}
