use crate::factory::aggregation_handler_factory::{
    AggregationHandlerModule, AggregationHandlerModuleParameters,
};
use crate::factory::user_export_handler_factory::{
    UserExportHandlerModule, UserExportHandlerModuleParameters,
};
use rust_job_server_application::queue::queue::QueueType;
use rust_job_server_application::usecase::user_export::async_user_export_interactor::AsyncUserExportInteractor;
use rust_job_server_config::Config;
use rust_job_server_infrastructure::job::queue::factory::sqs_client_factory::{
    SqsClientFactory, SqsClientFactoryError,
};
use rust_job_server_infrastructure::job::queue::sqs::sqs_user_export_queue::SqsUserExportQueue;
use rust_job_server_interface::cli::handler::aggregation::aggregation_handler::AggregationHandler;
use rust_job_server_interface::cli::handler::user_export::user_export_handler::{
    UserExportHandler, UserExportHandlerImpl,
};
use shaku::{module, HasComponent};
use std::sync::Arc;
use thiserror::Error;

pub struct CliContainer {}

module! {
    pub UserExportModule {
        components = [
            SqsUserExportQueue,
            UserExportHandlerImpl,
            AsyncUserExportInteractor,
        ],
        providers = []
    }
}

impl CliContainer {
    pub async fn build_aggregation_handler(
        config: Config,
    ) -> Result<Arc<dyn AggregationHandler>, CliContainerError> {
        let factory = SqsClientFactory::new(
            config.queue().base_url().to_string(),
            config.aws().region().to_string(),
            *config.queue().wait_time_seconds(),
        );

        let sqs_client = factory
            .build()
            .await
            .map_err(CliContainerError::BuildQueueClientError)?;

        let sqs_client = Arc::new(sqs_client);

        let url = sqs_client
            .get_queue_url(&QueueType::Aggregation.queue_name())
            .await
            .map_err(|_| CliContainerError::FetchQueueUrlError)?;

        let parameters = AggregationHandlerModuleParameters::new(url, sqs_client);

        // モジュールを構築
        let module = AggregationHandlerModule::factory(parameters);

        Ok(module.resolve())
    }

    pub async fn build_user_export_handler(
        config: Config,
    ) -> Result<Arc<dyn UserExportHandler>, CliContainerError> {
        let factory = SqsClientFactory::new(
            config.queue().base_url().to_string(),
            config.aws().region().to_string(),
            *config.queue().wait_time_seconds(),
        );

        let sqs_client = factory
            .build()
            .await
            .map_err(CliContainerError::BuildQueueClientError)?;

        let sqs_client = Arc::new(sqs_client);

        let url = sqs_client
            .get_queue_url(&QueueType::UserExport.queue_name())
            .await
            .map_err(|_| CliContainerError::FetchQueueUrlError)?;

        let parameters = UserExportHandlerModuleParameters::new(url, sqs_client);

        // モジュールを構築
        let module = UserExportHandlerModule::factory(parameters);

        Ok(module.resolve())
    }
}

#[derive(Debug, Error)]
pub enum CliContainerError {
    #[error("BuildQueueClientError {0}")]
    BuildQueueClientError(SqsClientFactoryError),
    #[error("FetchQueueUrlError")]
    FetchQueueUrlError,
    #[error("SendMessageError")]
    SendMessageError,
}
