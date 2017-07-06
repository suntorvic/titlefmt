use super::*;

pub enum Error {
    ArgumentError,
    TypeError,
}

pub type FunctionClosure<T> = Fn(&T, &[Box<expression::Expression<T>>]) -> Result<value::Value, Error>;

pub struct Function<T: metadata::Provider> {
    closure: Box<FunctionClosure<T>>,
    name: String,
}

#[macro_export]
macro_rules! function_object_maker {
    ($func_name: ident) => {
        pub fn make_function_object<T: metadata::Provider>() -> super::Function<T> {
            Function::new(
                stringify!($func_name),
                Box::new(|provider: &T, expressions: &[Box<expression::Expression<T>>]| -> Result<Value, Error> { $func_name(provider, expressions) })
            )
        }
    }
}

mod add;
mod if2;

pub fn standard_functions<T: metadata::Provider>() -> Vec<Box<Function<T>>> {
    let mut s = Vec::new();
    macro_rules! add_function {
        ($func_name: ident) => {
            s.push(Box::new($func_name::make_function_object::<T>()));
        }
    }
    add_function!(add);
    add_function!(if2);
    s
}

impl<T: metadata::Provider> Function<T> {
    pub fn new(name_param: &str, closure: Box<FunctionClosure<T>>) -> Function<T> {
        Function {
            closure,
            name: name_param.to_lowercase(),
        }
    }
    pub fn apply(&self, provider: &T, arguments: &[Box<expression::Expression<T>>]) -> Result<value::Value, Error> {
        (self.closure)(&provider, &arguments)
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
