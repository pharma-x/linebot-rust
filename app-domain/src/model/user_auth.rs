use derive_new::new;

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub enum AuthUserId {
    Line(LineId),
}

impl AuthUserId {
    pub fn value(&self) -> &String {
        match self {
            Self::Line(s) => &s.0,
        }
    }
}

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct LineId(pub String);

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub enum AuthToken {
    Line(LineAuthToken),
}

impl AuthToken {
    pub fn value(&self) -> &String {
        match self {
            Self::Line(s) => &s.0,
        }
    }
}

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct LineAuthToken(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserAuthData {
    Line(LineUserAuthData),
}

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct LineUserAuthData {
    pub auth_id: LineId,
    pub auth_token: LineAuthToken,
}
