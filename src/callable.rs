use core::fmt::Debug;

use std::fmt;
use std::rc::Rc;

use crate::error::*;
use crate::interpreter::*;
use crate::literal::*;

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<callable>")
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxResult>;
    fn arity(&self) -> usize;
}

impl LoxCallable for Callable {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxResult> {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.func.arity()
    }
}
