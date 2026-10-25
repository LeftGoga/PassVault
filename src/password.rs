use crate::errors::PasswordError;
pub struct Password {
    name: String,
    password: String,
}

impl Password {
    pub fn new(name: String, password: String) -> Result<Self, PasswordError> {
        if name.len() <= 0 {
            Err(PasswordError::InvalidName)?
        }
        if password.len() <= 0 {
            Err(PasswordError::InvalidPassword)?
        }
        Ok(Self { name, password })
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.name, self.password)
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
