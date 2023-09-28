use derive_new::new;
use serde::Serialize;

#[derive(new, Clone, Serialize)]
pub struct PrimaryUserId {
    id: String,
}

impl PrimaryUserId {
    pub fn value(&self) -> &String {
        &self.id
    }
}
