use uuid::Uuid;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub roles: HashSet<String>,
}

impl AuthUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}