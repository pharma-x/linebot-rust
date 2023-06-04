use std::sync::Arc;

use crate::adapter::module::{
    FactoriesModule, FactoriesModuleExt, RepositoriesModule, RepositoriesModuleExt,
};
use crate::adapter::persistance::{firestore::Firestore, mysql::Db};
use crate::application::usecase::linebot_webhook_usecase::LinebotWebhookUseCase;
use reqwest::Client;

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;
    type FactoriesModule: FactoriesModuleExt;

    fn linebot_webhook_usecase(
        &self,
    ) -> &LinebotWebhookUseCase<Self::RepositoriesModule, Self::FactoriesModule>;
}

pub struct Modules {
    linebot_webhook_usecase: LinebotWebhookUseCase<RepositoriesModule, FactoriesModule>,
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;
    type FactoriesModule = FactoriesModule;

    fn linebot_webhook_usecase(
        &self,
    ) -> &LinebotWebhookUseCase<Self::RepositoriesModule, Self::FactoriesModule> {
        &self.linebot_webhook_usecase
    }
}

impl Modules {
    pub async fn new() -> Self {
        let client = Client::new();
        let db = Db::new().await;
        let firestore = Firestore::new().await;
        let repositories_module: Arc<_> = Arc::new(RepositoriesModule::new(client, db, firestore));
        let factories_module: Arc<_> = Arc::new(FactoriesModule::new());

        let linebot_webhook_usecase: LinebotWebhookUseCase<RepositoriesModule, FactoriesModule> =
            LinebotWebhookUseCase::new(repositories_module.clone(), factories_module.clone());

        Self {
            linebot_webhook_usecase,
        }
    }
}
