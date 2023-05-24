use derive_new::new;
#[derive(new)]
pub struct CreateLineUser {
    pub line_id: String,
    pub display_name: String,
    pub profile_image_url: String,
}
