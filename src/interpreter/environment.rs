use std::collections::HashMap;

use crate::{object::Object, token::Token};

#[derive(Clone)]
pub struct Environment {
    enclosing: Box<Option<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Box::new(Option::None),
        }
    }

    pub fn new_with_enclosing(environment: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Box::new(Option::Some(environment)),
        }
    }

    pub fn get_enclosing(&mut self) -> Option<Environment> {
        *self.enclosing.clone()
    }

    pub fn _add_enclosing(&mut self, environment: Environment) {
        self.enclosing = Box::new(Option::Some(environment));
    }

    //not checking if variable already exists thus allowing reinitalization of a variable
    pub fn define(&mut self, name: String, value: Object) {
        // println!("defining variable");
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Object, &str> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val.clone()),
            None => match &*self.enclosing {
                Some(env) => env.get(name),
                None => Err("Undefined variable"),
            },
        }
    }

    pub fn assign(&mut self, name: Token, value: &Object) -> Result<(), String> {
        if self.values.contains_key(&name.lexeme) {
            *self.values.entry(name.lexeme).or_insert(Object::Null) = value.clone();
            Ok(())
        } else {
            match &mut *self.enclosing {
                Some(environment) => environment.assign(name, value),
                None => Err("Error : Undefined variable ".to_string() + &name.lexeme),
            }
        }
    }
}
