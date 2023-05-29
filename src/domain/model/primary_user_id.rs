use derive_new::new;

#[derive(new)]
pub struct PrimaryUserId {
    id: String,
}

impl PrimaryUserId {
    pub fn value(&self) -> &String {
        &self.id
    }
}
