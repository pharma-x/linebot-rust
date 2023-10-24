use derive_new::new;
use serde::Serialize;

#[derive(new, Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PrimaryUserId {
    id: String,
}

impl PrimaryUserId {
    pub fn value(&self) -> &String {
        &self.id
    }
}
