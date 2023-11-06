use adapter::module::{AdaptersModule, AdaptersModuleExt};
use adapter::persistance::{firestore::Firestore, mysql::Db};
use application::usecase::linebot_webhook_usecase::LinebotWebhookUseCase;
use reqwest::Client;
use std::sync::Arc;

pub trait ModulesExt {
    type AdaptersModule: AdaptersModuleExt;

    fn linebot_webhook_usecase(&self) -> &LinebotWebhookUseCase<Self::AdaptersModule>;
}

pub struct Modules {
    linebot_webhook_usecase: LinebotWebhookUseCase<AdaptersModule>,
}

impl ModulesExt for Modules {
    type AdaptersModule = AdaptersModule;

    fn linebot_webhook_usecase(&self) -> &LinebotWebhookUseCase<Self::AdaptersModule> {
        &self.linebot_webhook_usecase
    }
}

impl Modules {
    pub async fn new() -> Self {
        let client = Client::new();
        let db = Db::new().await;
        let firestore = Firestore::new().await;
        let adapters_module: Arc<_> = Arc::new(AdaptersModule::new(client, db, firestore));

        let linebot_webhook_usecase: LinebotWebhookUseCase<AdaptersModule> =
            LinebotWebhookUseCase::new(adapters_module);

        Self {
            linebot_webhook_usecase,
        }
    }
}

pub mod test {
    use super::ModulesExt;
    use adapter::module::test::TestAdaptersModule;
    use application::usecase::linebot_webhook_usecase::LinebotWebhookUseCase;
    use domain::gateway::{send_message::MockSendMessageGateway, user_auth::MockUserAuthGateway};
    use domain::repository::{talk_room::MockTalkRoomRepository, user::MockUserRepository};
    use std::sync::Arc;

    pub struct TestModules {
        linebot_webhook_usecase: LinebotWebhookUseCase<TestAdaptersModule>,
    }

    impl ModulesExt for TestModules {
        type AdaptersModule = TestAdaptersModule;

        fn linebot_webhook_usecase(&self) -> &LinebotWebhookUseCase<Self::AdaptersModule> {
            &self.linebot_webhook_usecase
        }
    }

    impl TestModules {
        pub async fn new(
            user_auth_gateway: MockUserAuthGateway,
            user_repository: MockUserRepository,
            talk_room_repository: MockTalkRoomRepository,
            send_message_gateway: MockSendMessageGateway,
        ) -> Self {
            let adapters_module = Arc::new(TestAdaptersModule::new(
                user_auth_gateway,
                user_repository,
                talk_room_repository,
                send_message_gateway,
            ));

            let linebot_webhook_usecase: LinebotWebhookUseCase<TestAdaptersModule> =
                LinebotWebhookUseCase::new(adapters_module);

            Self {
                linebot_webhook_usecase,
            }
        }
    }
}
