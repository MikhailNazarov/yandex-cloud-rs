//! This example uses the Yandex Cloud Logging API to write a log entry.

use prost_types::Timestamp;
use tonic::transport::channel::Endpoint;
use yandex_cloud::yandex::cloud::logging::v1::destination::Destination;
use yandex_cloud::yandex::cloud::logging::v1::log_ingestion_service_client::LogIngestionServiceClient;
use yandex_cloud::yandex::cloud::logging::v1::Destination as OuterDestination;
use yandex_cloud::yandex::cloud::logging::v1::IncomingLogEntry;
use yandex_cloud::yandex::cloud::logging::v1::WriteRequest;
use yandex_cloud::AuthInterceptor;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("https://ingester.logging.yandexcloud.net")
        .connect()
        .await?;

    let mut client = LogIngestionServiceClient::with_interceptor(
        channel,
        AuthInterceptor::new("YOUR_TOKEN_HERE"),
    );

    let request = WriteRequest {
        destination: Some(OuterDestination {
            destination: Some(Destination::LogGroupId("YOUR_LOG_GROUP_ID".into())),
        }),
        entries: vec![IncomingLogEntry {
            timestamp: Some(Timestamp::date_time(2023, 04, 24, 23, 44, 30).unwrap()),
            message: "test log message".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    client.write(request).await.unwrap();
    Ok(())
}
