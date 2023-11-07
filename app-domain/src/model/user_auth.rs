use derive_new::new;

#[derive(new, Debug, Clone, PartialEq, Eq)]
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

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct LineId {
    pub value: String,
}

impl LineId {
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[derive(new, Debug, Clone, PartialEq, Eq)]
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

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct LineAuthToken {
    pub value: String,
}

impl LineAuthToken {
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserAuthData {
    Line(LineUserAuthData),
}

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct LineUserAuthData {
    pub auth_id: AuthUserId,
    pub auth_token: AuthToken,
}
