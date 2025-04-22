pub mod job_container;
pub mod cli_container;


// pub struct DI {}
// 
// module! {
//     RegisterModule {
//         components = [RegisterService<SqsExampleQueue>],
//         providers = []
//     }
// }
// 
// module! {
//     ServerModule {
//         components = [ServerService<SqsExampleQueue>],
//         providers = []
//     }
// }
// 
// impl DI {
//     pub async fn register_service_build(config: Config) -> Arc<dyn Service> {
//         let client = Self::build_sqs_client(config).await;
//         let example_queue = SqsExampleQueue::new(client);
// 
//         let module = RegisterModule::builder()
//             .with_component_parameters::<RegisterService<SqsExampleQueue>>(
//                 RegisterServiceParameters {
//                     example_queue: example_queue,
//                 },
//             )
//             .build();
// 
//         module.resolve()
//     }
// 
//     pub async fn server_service_build(config: Config) -> Arc<dyn Service> {
//         let client = Self::build_sqs_client(config).await;
// 
//         let server: Server = ServerBuilder::new().add_worker(Arc::new(SqsWorker::new(
//             SqsQueue::new(
//                 url::Url::from_str(config.queue().base_url()).expect("invalid url"),
//                 Self::build_sqs_client(config).await,
//             ),
//             Box::new(ExampleHandler::new()),
//         ).build();
// 
// 
//         // let client = Self::build_sqs_client(config).await;
//         // let example_queue = SqsExampleQueue::new(client);
//         //
//         // let module = ServerModule::builder()
//         //     .with_component_parameters::<ServerService<SqsExampleQueue>>(ServerServiceParameters {
//         //         example_queue: example_queue,
//         //     })
//         //     .build();
//         //
//         // module.resolve()
//     }
// 
//     async fn build_sqs_client(config: Config) -> Arc<AwsSqsClient> {
//         let url = Url::from_str(config.queue().base_url()).expect("invalid url");
//         AwsSqsClient::new()
// 
//         let sqs_client = build_client(url).await.expect("sqs client build error");
//         let client = Arc::new(sqs_client);
//         client
//     }
// }
