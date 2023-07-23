use std::collections::BTreeSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct User {
    pub name: String,
    pub pass: String,
}

/// users shoould be moved around
#[derive(Default)]
pub struct Users {
    pub free: BTreeSet<User>,
    pub allocated: BTreeSet<User>
}