use derive_new::new;

#[derive(new, Debug, Clone)]
pub enum AuthUserId {
    Line(String),
}

impl AuthUserId {
    pub fn value(&self) -> &String {
        match self {
            Self::Line(s) => s,
        }
    }
}

#[derive(new, Debug, Clone)]
pub enum AuthToken {
    Line(String),
}

impl AuthToken {
    pub fn value(&self) -> &String {
        match self {
            Self::Line(s) => s,
        }
    }
}

pub enum UserAuthData {
    Line(LineUserAuthData),
}

#[derive(new, Debug, Clone)]
pub struct LineUserAuthData {
    pub auth_id: AuthUserId,
    pub token: AuthToken,
}
