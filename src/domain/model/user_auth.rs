pub trait AuthUserId {
    fn value(&self) -> &String;
}

pub trait AuthToken {
    fn value(&self) -> &String;
}

pub trait UserAuthData {
    type UserId: AuthUserId;
    type Token: AuthToken;
    fn user_id(&self) -> &Self::UserId;
    fn token(&self) -> &Self::Token;
}
