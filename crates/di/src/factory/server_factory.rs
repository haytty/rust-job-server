use rust_job_server_infrastructure::job::server::{BasicServer, BasicServerParameters};
use rust_job_server_interface::job::worker::Worker;
use shaku::module;
use std::sync::Arc;

module! {
    pub ServerModule {
        components = [BasicServer],
        providers = []
    }
}

#[derive(Debug)]
pub struct ServerModuleParameters {
    workers: Vec<Arc<dyn Worker>>,
}

impl ServerModuleParameters {
    pub fn new(workers: Vec<Arc<dyn Worker>>) -> Self {
        Self { workers }
    }
}

impl ServerModule {
    pub fn factory(parameters: ServerModuleParameters) -> ServerModule {
        let builder = ServerModule::builder().with_component_parameters::<BasicServer>(
            BasicServerParameters {
                workers: parameters.workers,
            },
        );

        builder.build()
    }
}
