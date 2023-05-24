use super::repository::HttpClientRepositoryImpl;
use crate::domain::model::{line_user::LineUser, user_auth::UserAuthData};
use crate::domain::repository::line_user_auth::LineUserAuthRepository;
use reqwest::Client;

pub trait RepositoriesModuleExt {
    type LineUserAuthRepo: LineUserAuthRepository;
    fn line_user_auth_repository(&self) -> &Self::LineUserAuthRepo;
}

pub struct RepositoriesModule {
    line_user_auth_repository: HttpClientRepositoryImpl<UserAuthData<LineUser>>,
}

impl RepositoriesModuleExt for RepositoriesModule {
    type LineUserAuthRepo = HttpClientRepositoryImpl<UserAuthData<LineUser>>;

    fn line_user_auth_repository(&self) -> &Self::LineUserAuthRepo {
        &self.line_user_auth_repository
    }
}

impl RepositoriesModule {
    pub fn new() -> Self {
        let client = Client::new();
        let line_user_auth_repository =
            HttpClientRepositoryImpl::<UserAuthData<LineUser>>::new(client);

        Self {
            line_user_auth_repository,
        }
    }
}
