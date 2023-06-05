use derive_new::new;

#[derive(new, Debug, Clone)]
pub enum AuthUserId {
    Line(LineId),
}

impl AuthUserId {
    pub fn value(&self) -> &String {
        match self {
            Self::Line(s) => s.value(),
        }
    }
}

#[derive(new, Debug, Clone)]
pub struct LineId {
    pub value: String,
}

impl LineId {
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[derive(new, Debug, Clone)]
pub enum AuthToken {
    Line(LineAuthToken),
}

impl AuthToken {
    pub fn value(&self) -> &String {
        match self {
            Self::Line(s) => s.value(),
        }
    }
}

#[derive(new, Debug, Clone)]
pub struct LineAuthToken {
    pub value: String,
}

impl LineAuthToken {
    pub fn value(&self) -> &String {
        &self.value
    }
}

pub enum UserAuthData {
    Line(LineUserAuthData),
}

#[derive(new, Debug, Clone)]
pub struct LineUserAuthData {
    pub auth_id: AuthUserId,
    pub auth_token: AuthToken,
}

impl LineUserAuthData {
    pub fn auth_id(&self) -> &String {
        &self.auth_id.value()
    }
    pub fn auth_token(&self) -> &String {
        &self.auth_token.value()
    }
}
