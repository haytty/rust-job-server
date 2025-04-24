use rust_job_server_application::usecase::user_export::async_user_export_interactor::AsyncUserExportInteractor;
use rust_job_server_infrastructure::job::queue::sqs::client::SqsClient;
use rust_job_server_infrastructure::job::queue::sqs::sqs_user_export_queue::{
    SqsUserExportQueue, SqsUserExportQueueParameters,
};
use rust_job_server_interface::cli::handler::user_export::user_export_handler::UserExportHandlerImpl;
use shaku::module;
use std::sync::Arc;
use url::Url;

module! {
    pub UserExportHandlerModule {
        components = [
            SqsUserExportQueue,
            UserExportHandlerImpl,
            AsyncUserExportInteractor,
        ],
        providers = []
    }
}

#[derive(Debug)]
pub struct UserExportHandlerModuleParameters {
    queue_url: Url,
    sqs_client: Arc<SqsClient>,
}

impl UserExportHandlerModuleParameters {
    pub fn new(queue_url: Url, sqs_client: Arc<SqsClient>) -> Self {
        Self {
            queue_url,
            sqs_client,
        }
    }
}
impl UserExportHandlerModule {
    pub fn factory(parameters: UserExportHandlerModuleParameters) -> UserExportHandlerModule {
        let builder = UserExportHandlerModule::builder()
            .with_component_parameters::<SqsUserExportQueue>(SqsUserExportQueueParameters {
                url: parameters.queue_url.clone(),
                client: parameters.sqs_client.clone(),
            });
        let module = builder.build();
        module
    }
}
