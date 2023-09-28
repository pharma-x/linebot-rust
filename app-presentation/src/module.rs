use std::sync::Arc;
use adapter::module::{
    RepositoriesModule, RepositoriesModuleExt,
};
use adapter::persistance::{firestore::Firestore, mysql::Db};
use application::usecase::linebot_webhook_usecase::LinebotWebhookUseCase;
use reqwest::Client;

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn linebot_webhook_usecase(
        &self,
    ) -> &LinebotWebhookUseCase<Self::RepositoriesModule>;
}

pub struct Modules {
    linebot_webhook_usecase: LinebotWebhookUseCase<RepositoriesModule>,
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn linebot_webhook_usecase(
        &self,
    ) -> &LinebotWebhookUseCase<Self::RepositoriesModule> {
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
