use adapter::module::{RepositoriesModule, RepositoriesModuleExt};
use adapter::persistance::{firestore::Firestore, mysql::Db};
use application::usecase::linebot_webhook_usecase::LinebotWebhookUseCase;
use reqwest::Client;
use std::sync::Arc;

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn linebot_webhook_usecase(&self) -> &LinebotWebhookUseCase<Self::RepositoriesModule>;
}

pub struct Modules {
    linebot_webhook_usecase: LinebotWebhookUseCase<RepositoriesModule>,
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn linebot_webhook_usecase(&self) -> &LinebotWebhookUseCase<Self::RepositoriesModule> {
        &self.linebot_webhook_usecase
    }
}

impl Modules {
    pub async fn new() -> Self {
        let client = Client::new();
        let db = Db::new().await;
        let firestore = Firestore::new().await;
        let repositories_module: Arc<_> = Arc::new(RepositoriesModule::new(client, db, firestore));

        let linebot_webhook_usecase: LinebotWebhookUseCase<RepositoriesModule> =
            LinebotWebhookUseCase::new(repositories_module);

        Self {
            linebot_webhook_usecase,
        }
    }
}

pub mod test {
    use super::ModulesExt;
    use adapter::module::test::TestRepositoriesModule;
    use application::usecase::linebot_webhook_usecase::LinebotWebhookUseCase;
    use domain::repository::{
        talk_room::MockTalkRoomRepository, user::MockUserRepository,
        user_auth::MockUserAuthRepository,
    };
    use std::sync::Arc;

    pub struct TestModules {
        linebot_webhook_usecase: LinebotWebhookUseCase<TestRepositoriesModule>,
    }

    impl ModulesExt for TestModules {
        type RepositoriesModule = TestRepositoriesModule;

        fn linebot_webhook_usecase(&self) -> &LinebotWebhookUseCase<Self::RepositoriesModule> {
            &self.linebot_webhook_usecase
        }
    }

    impl TestModules {
        pub async fn new(
            user_auth_repository: MockUserAuthRepository,
            user_repository: MockUserRepository,
            talk_room_repository: MockTalkRoomRepository,
        ) -> Self {
            let repositories_module = Arc::new(TestRepositoriesModule::new(
                user_auth_repository,
                user_repository,
                talk_room_repository,
            ));

            let linebot_webhook_usecase: LinebotWebhookUseCase<TestRepositoriesModule> =
                LinebotWebhookUseCase::new(repositories_module);

            Self {
                linebot_webhook_usecase,
            }
        }
    }
}
