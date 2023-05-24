use std::sync::Arc;

use crate::adapter::module::{RepositoriesModule, RepositoriesModuleExt};
use crate::application::usecase::linebot_webhook_usecase::LinebotWebhookUseCase;

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
    pub async fn new() -> Modules {
        let repositories_module: Arc<_> = Arc::new(RepositoriesModule::new());

        let linebot_webhook_usecase: LinebotWebhookUseCase<RepositoriesModule> = LinebotWebhookUseCase::new(repositories_module.clone());

        Self {
            linebot_webhook_usecase,
        }
    }
}
