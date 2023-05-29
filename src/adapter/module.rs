use super::persistance::{firestore::Firestore, mysql::Db};
use super::repository::{
    DatabaseRepositoryImpl, FirestoreRepositoryImpl, HttpClientRepositoryImpl,
};
use crate::domain::model::talk_room::TalkRoom;
use crate::domain::model::{line_user::LineUser, user_auth::UserAuthData};
use crate::domain::repository::{
    line_user::LineUserRepository, line_user_auth::LineUserAuthRepository,
    talk_room::TalkRoomRepository,
};
use reqwest::Client;

pub trait RepositoriesModuleExt {
    type LineUserAuthRepo: LineUserAuthRepository;
    type LineUserRepo: LineUserRepository;
    type TalkRoomRepo: TalkRoomRepository;
    fn line_user_auth_repository(&self) -> &Self::LineUserAuthRepo;
    fn line_user_repository(&self) -> &Self::LineUserRepo;
    fn talk_room_repository(&self) -> &Self::TalkRoomRepo;
}

pub struct RepositoriesModule {
    line_user_auth_repository: HttpClientRepositoryImpl<UserAuthData<LineUser>>,
    line_user_repository: DatabaseRepositoryImpl<LineUser>,
    talk_room_repository: FirestoreRepositoryImpl<TalkRoom>,
}

impl RepositoriesModuleExt for RepositoriesModule {
    type LineUserAuthRepo = HttpClientRepositoryImpl<UserAuthData<LineUser>>;
    type LineUserRepo = DatabaseRepositoryImpl<LineUser>;
    type TalkRoomRepo = FirestoreRepositoryImpl<TalkRoom>;

    fn line_user_auth_repository(&self) -> &Self::LineUserAuthRepo {
        &self.line_user_auth_repository
    }
    fn line_user_repository(&self) -> &Self::LineUserRepo {
        &self.line_user_repository
    }
    fn talk_room_repository(&self) -> &Self::TalkRoomRepo {
        &self.talk_room_repository
    }
}

impl RepositoriesModule {
    pub fn new(client: Client, db: Db, firestore: Firestore) -> Self {
        // let client = Client::new();
        // let db = Db::new();
        // let firestore = Firestore::new();

        let line_user_auth_repository = HttpClientRepositoryImpl::new(client);
        let line_user_repository = DatabaseRepositoryImpl::new(db.clone());
        let talk_room_repository = FirestoreRepositoryImpl::new(firestore.clone());

        Self {
            line_user_auth_repository,
            line_user_repository,
            talk_room_repository,
        }
    }
}
