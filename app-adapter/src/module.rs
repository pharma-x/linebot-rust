use crate::persistance::{firestore::Firestore, mysql::Db};
use crate::repository::{
    DatabaseRepositoryImpl, DbFirestoreRepositoryImpl, HttpClientRepositoryImpl,
};
use domain::model::{talk_room::TalkRoom, user::User, user_auth::UserAuthData};
use domain::repository::{
    talk_room::TalkRoomRepository, user::UserRepository, user_auth::UserAuthRepository,
};
use reqwest::Client;

pub trait RepositoriesModuleExt {
    type UserAuthRepo: UserAuthRepository;
    type UserRepo: UserRepository;
    type TalkRoomRepo: TalkRoomRepository;
    fn user_auth_repository(&self) -> &Self::UserAuthRepo;
    fn user_repository(&self) -> &Self::UserRepo;
    fn talk_room_repository(&self) -> &Self::TalkRoomRepo;
}

pub struct RepositoriesModule {
    user_auth_repository: HttpClientRepositoryImpl<UserAuthData>,
    user_repository: DatabaseRepositoryImpl<User>,
    talk_room_repository: DbFirestoreRepositoryImpl<TalkRoom>,
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserAuthRepo = HttpClientRepositoryImpl<UserAuthData>;
    type UserRepo = DatabaseRepositoryImpl<User>;
    type TalkRoomRepo = DbFirestoreRepositoryImpl<TalkRoom>;

    fn user_auth_repository(&self) -> &Self::UserAuthRepo {
        &self.user_auth_repository
    }
    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
    fn talk_room_repository(&self) -> &Self::TalkRoomRepo {
        &self.talk_room_repository
    }
}

impl RepositoriesModule {
    pub fn new(client: Client, db: Db, firestore: Firestore) -> Self {
        let user_auth_repository = HttpClientRepositoryImpl::new(client);
        let user_repository = DatabaseRepositoryImpl::new(db.clone());
        let talk_room_repository = DbFirestoreRepositoryImpl::new(db, firestore.clone());

        Self {
            user_auth_repository,
            user_repository,
            talk_room_repository,
        }
    }
}
