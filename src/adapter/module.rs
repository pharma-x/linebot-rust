use super::persistance::mysql::Db;
use super::repository::{DatabaseRepositoryImpl, HttpClientRepositoryImpl};
use crate::domain::model::{line_user::LineUser, user_auth::UserAuthData};
use crate::domain::repository::line_user::LineUserRepository;
use crate::domain::repository::line_user_auth::LineUserAuthRepository;
use reqwest::Client;

pub trait RepositoriesModuleExt {
    type LineUserAuthRepo: LineUserAuthRepository;
    type LineUserRepo: LineUserRepository;
    fn line_user_auth_repository(&self) -> &Self::LineUserAuthRepo;
    fn line_user_repository(&self) -> &Self::LineUserRepo;
}

pub struct RepositoriesModule {
    line_user_auth_repository: HttpClientRepositoryImpl<UserAuthData<LineUser>>,
    line_user_repository: DatabaseRepositoryImpl<LineUser>,
}

impl RepositoriesModuleExt for RepositoriesModule {
    type LineUserAuthRepo = HttpClientRepositoryImpl<UserAuthData<LineUser>>;
    type LineUserRepo = DatabaseRepositoryImpl<LineUser>;

    fn line_user_auth_repository(&self) -> &Self::LineUserAuthRepo {
        &self.line_user_auth_repository
    }
    fn line_user_repository(&self) -> &Self::LineUserRepo {
        &self.line_user_repository
    }
}

impl RepositoriesModule {
    pub fn new() -> Self {
        let client = Client::new();
        let db = Db::new();

        let line_user_auth_repository =
            HttpClientRepositoryImpl::<UserAuthData<LineUser>>::new(client);
        let line_user_repository = DatabaseRepositoryImpl::<LineUser>::new(db.clone());

        Self {
            line_user_auth_repository,
            line_user_repository,
        }
    }
}
