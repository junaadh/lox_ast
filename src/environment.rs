use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::LoxError, lox_error, token::Token, token_type::Object};

#[derive(Debug)]
pub struct Environment {
    pub enclosing: Option<Rc<Environment>>,
    pub values: RefCell<HashMap<String, Object>>,
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Self {
            enclosing: self.enclosing.clone(),
            values: self.values.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.enclosing = source.enclosing.clone();
        self.values = source.values.clone();
    }
}

impl Environment {
    // gloabl_scope
    pub fn initialize() -> Self {
        Self {
            enclosing: None,
            values: RefCell::new(HashMap::<String, Object>::new()),
        }
    }

    // create local_scope
    pub fn from(enclosing: Environment) -> Self {
        Self {
            enclosing: Some(Rc::new(enclosing)),
            values: RefCell::new(HashMap::<String, Object>::new()),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        // self.values.insert(name, value);
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        // if self.values.contains_key(&name.lexeme) {
        //     Ok(self.values.get(&name.lexeme).unwrap().clone())
        // } else if self.enclosing.is_some() {
        //     self.enclosing.clone().unwrap().get(name)
        // } else {
        //     Err(lox_error!(name.clone(), "Undefined variable."))
        // }
        match self.values.borrow_mut().get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => match &self.enclosing {
                Some(env) => env.get(name),
                None => Err(lox_error!(name.clone(), "Undefined variable.")),
            },
        }
    }

    pub fn assign(&self, name: &Token, value: Object) -> Result<(), LoxError> {
        // if self.values.contains_key(&name.lexeme) {
        //     self.values.insert(name.lexeme.clone(), value);
        //     Ok(())
        // } else if self.enclosing.is_some() {
        //     // self.enclosing.clone().unwrap().assign(name, value)
        //     self.enclosing
        //         .borrow_mut()
        //         .clone()
        //         .unwrap()
        //         .assign(name, value)
        // } else {
        //     Err(lox_error!(name.clone(), "Undefined variable."))
        // }
        if self.values.borrow_mut().contains_key(&name.lexeme) {
            self.values.borrow_mut().insert(name.lexeme.clone(), value);
            return Ok(());
        }
        match &self.enclosing {
            Some(env) => env.assign(name, value),
            None => Err(lox_error!(name.clone(), "Undefined variable.")),
        }
    }
}
