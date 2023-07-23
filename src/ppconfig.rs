use std::collections::HashMap;

use serde::Serialize;

use crate::User;

#[derive(Serialize, Default)]
pub struct PPConfig {
    pub users: HashMap<String, PPUser>
}

#[derive(Serialize)]
pub struct PPUser {
    acct: String,
    pass: String,
    otp: String
}

impl PPUser {
    pub fn from_user(user: &User) -> Self {
        Self {
            acct: user.name.to_string(),
            pass: user.pass.to_string(),
            otp: String::default()
        }
    }
}