use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };
use std::path::{ Path };
use std::ops::Deref;

fn directory_path<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() < 1 {
        return Err(Error::ArgumentError);
    }
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let file_path = Path::new(original_string.as_str());
    let result_path = {
        if file_path.is_dir() {
            file_path
        }
        else {
            match file_path.parent() {
                Some(dir) => dir,
                None => Path::new("/"),
            }
        }
    };
    let result_text: String = result_path.to_string_lossy().deref().to_owned();
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

function_object_maker!(directory_path);

#[cfg(test)]
mod test;