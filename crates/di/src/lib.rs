use aws_sdk_sqs::Client as AwsSqsClient;
use rust_job_server_config::Config;
use rust_job_server_infrastructure::queue::aws_sqs_modules::build_client;
use rust_job_server_infrastructure::queue::sqs_example_queue::SqsExampleQueue;
use rust_job_server_service::service::register_service::{
    RegisterService, RegisterServiceParameters,
};
use rust_job_server_service::service::server_service::{ServerService, ServerServiceParameters};
use rust_job_server_service::service::Service;
use shaku::{module, HasComponent};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use url::Url;

pub struct DI {}

module! {
    RegisterModule {
        components = [RegisterService<SqsExampleQueue>],
        providers = []
    }
}

module! {
    ServerModule {
        components = [ServerService<SqsExampleQueue>],
        providers = []
    }
}

impl DI {
    pub async fn register_service_build(config: Config) -> Arc<dyn Service> {
        let client = Self::build_sqs_client(config).await;
        let example_queue = SqsExampleQueue::new(client);

        let module = RegisterModule::builder()
            .with_component_parameters::<RegisterService<SqsExampleQueue>>(
                RegisterServiceParameters {
                    example_queue: example_queue,
                },
            )
            .build();

        module.resolve()
    }

    pub async fn server_service_build(config: Config) -> Arc<dyn Service> {
        let client = Self::build_sqs_client(config).await;
        let example_queue = SqsExampleQueue::new(client);

        let module = ServerModule::builder()
            .with_component_parameters::<ServerService<SqsExampleQueue>>(ServerServiceParameters {
                example_queue: example_queue,
            })
            .build();

        module.resolve()
    }

    async fn build_sqs_client(config: Config) -> Arc<AwsSqsClient> {
        let url = Url::from_str(config.queue().base_url()).expect("invalid url");
        let sqs_client = build_client(url).await.expect("sqs client build error");
        let client = Arc::new(sqs_client);
        client
    }
}
