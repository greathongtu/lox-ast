use std::time::SystemTime;

use crate::callable::*;
use crate::error::*;
use crate::interpreter::*;
use crate::literal::*;

pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(&self, _terp: &Interpreter, _args: Vec<Literal>) -> Result<Literal, LoxResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Literal::Number(n.as_millis() as f64)),
            Err(e) => Err(LoxResult::system_error(&format!(
                "Clock returned invalid duration: {:?}",
                e.duration()
            ))),
        }
    }

    fn arity(&self) -> usize {
        0
    }
}
