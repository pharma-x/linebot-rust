use crate::domain::model::user::User;
use derive_new::new;
use std::marker::PhantomData;

// pub trait AuthUserId {
//     fn value(&self) -> &String;
// }

#[derive(new, Debug, Clone)]
pub struct AuthUserId<T: User> {
    value: String,
    _marker: PhantomData<T>,
}

impl<T: User> AuthUserId<T> {
    pub fn value(&self) -> &String {
        &self.value
    }
}

// pub trait AuthToken {
//     fn value(&self) -> &String;
// }

#[derive(new, Debug, Clone)]
pub struct AuthToken<T: User> {
    pub value: String,
    _marker: PhantomData<T>,
}

impl<T: User> AuthToken<T> {
    pub fn value(&self) -> &String {
        &self.value
    }
}

// pub trait UserAuthData {
//     type UserId: AuthUserId;
//     type Token: AuthToken;
//     fn user_id(&self) -> &Self::UserId;
//     fn token(&self) -> &Self::Token;
// }

#[derive(new, Debug, Clone)]
pub struct UserAuthData<T: User> {
    user_id: AuthUserId<T>,
    token: AuthToken<T>,
}

impl<T: User> UserAuthData<T> {
    pub fn user_id(&self) -> &AuthUserId<T> {
        &self.user_id
    }
    pub fn token(&self) -> &AuthToken<T> {
        &self.token
    }
}
