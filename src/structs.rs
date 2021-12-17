pub enum Type<'a> {
  I32,
  I64,
  FuncRef(Box<dyn AsRef<Function<'a>>>),
}

pub struct Export<'a> {
  exported_name: &'a str,
  export: Box<dyn AsRef<ModuleItems<'a>>>,
}

pub struct Function<'a> {
  name: Option<&'a str>,
  span: (usize, usize),
  ptr: usize,
  parameters: Option<Vec<Type<'a>>>,
  locals: Option<Vec<Type<'a>>>,
  result: Option<Vec<Type<'a>>>,
  export: Option<Box<Export<'a>>>,
}

pub struct Memory<'a> {
  name: Option<&'a str>,
  span: (usize, usize),
  size: usize,
  ptr: usize,
}

pub enum ModuleItems<'a> {
  Function(Function<'a>),
  Memory(Memory<'a>),
  Export(Export<'a>),
}
