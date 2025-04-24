use rust_job_server_application::usecase::user_export::user_export_from_file_interactor::UserExportFromFileInteractor;
use rust_job_server_infrastructure::job::queue::sqs::client::SqsClient;
use rust_job_server_infrastructure::job::queue::sqs::sqs_user_export_queue::{
    SqsUserExportQueue, SqsUserExportQueueParameters,
};
use rust_job_server_infrastructure::job::worker::user_export_worker::UserExportWorker;
use rust_job_server_infrastructure::repository::file_user_repository::{
    FileUserRepository, FileUserRepositoryParameters,
};
use rust_job_server_interface::job::handler::user_export::user_export_handler::UserExportHandlerImpl;
use shaku::module;
use std::sync::Arc;
use url::Url;

// モジュール構築用の `shaku` 定義
module! {
    pub UserExportWorkerModule {
        components = [
            UserExportWorker,
            SqsUserExportQueue,
            UserExportHandlerImpl,
            UserExportFromFileInteractor,
            FileUserRepository
        ],
        providers = []
    }
}

// モジュール構築用のパラメータ
#[derive(Debug)]
pub struct UserExportWorkerModuleParameters {
    pub queue_url: Url,
    pub sqs_client: Arc<SqsClient>,
}

impl UserExportWorkerModuleParameters {
    pub fn new(queue_url: Url, sqs_client: Arc<SqsClient>) -> Self {
        Self {
            queue_url,
            sqs_client,
        }
    }
}

impl UserExportWorkerModule {
    pub fn factory(parameters: UserExportWorkerModuleParameters) -> UserExportWorkerModule {
        let builder = UserExportWorkerModule::builder()
            .with_component_parameters::<SqsUserExportQueue>(SqsUserExportQueueParameters {
                url: parameters.queue_url.clone(),
                client: parameters.sqs_client.clone(),
            })
            .with_component_parameters::<FileUserRepository>(FileUserRepositoryParameters {});

        builder.build()
    }
}
