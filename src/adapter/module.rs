use crate::domain::{
    model::line_user_auth::LineUserAuthData, repository::line_user_auth::LineUserAuthRepository,
};

use super::repository::user_auth::UserAuthRepository;
use reqwest::Client;

pub trait RepositoriesModuleExt {
    type LineUserAuthRepo: LineUserAuthRepository;
    fn line_user_auth_repository(&self) -> &Self::LineUserAuthRepo;
}

pub struct RepositoriesModule {
    line_user_auth_repository: UserAuthRepository<LineUserAuthData>,
}

impl RepositoriesModuleExt for RepositoriesModule {
    type LineUserAuthRepo = UserAuthRepository<LineUserAuthData>;

    fn line_user_auth_repository(&self) -> &Self::LineUserAuthRepo {
        &self.line_user_auth_repository
    }
}

impl RepositoriesModule {
    pub fn new() -> Self {
        let client = Client::new();
        let line_user_auth_repository = UserAuthRepository::<LineUserAuthData>::new(client);

        Self {
            line_user_auth_repository,
        }
    }
}
