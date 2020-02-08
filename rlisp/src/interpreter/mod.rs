use std::fmt;

pub struct EvalOutput {
  pub output: String,
}

pub struct EvalError {

}

impl fmt::Display for EvalError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "error")
  }
}

pub trait Interpreter {
  fn eval(&mut self, code: &str) -> Result<EvalOutput, EvalError>;
}

pub struct BasicInterpreter {
  
}

impl BasicInterpreter {
  pub fn new() -> BasicInterpreter {
    return BasicInterpreter{};
  }
}

impl Interpreter for BasicInterpreter {
  fn eval(&mut self, code: &str) -> Result<EvalOutput, EvalError> {
    Ok(EvalOutput{
      output: String::from("ok"),
    })
  }
}
