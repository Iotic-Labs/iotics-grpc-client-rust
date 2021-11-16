use anyhow::Context;
use tonic::transport::Channel;
use tonic::{Request, Response};

use crate::client::iotics::api::twin_api_client::TwinApiClient;
use crate::client::iotics::api::upsert_twin_request::Payload as UpsertTwinRequestPayload;
use crate::client::iotics::api::UpsertTwinRequest;
use crate::client::iotics::api::{GeoLocation, Headers, LangLiteral, Property};

use crate::helpers::generate_client_app_id;

pub use crate::client::iotics::api::UpsertFeedWithMeta;
pub use crate::client::iotics::api::UpsertTwinResponse;

#[allow(clippy::too_many_arguments)]
pub async fn upsert_twin(
    client: &mut TwinApiClient<Channel>,
    token: &str,
    did: &str,
    labels: Vec<LangLiteral>,
    properties: Vec<Property>,
    feeds: Vec<UpsertFeedWithMeta>,
    location: Option<GeoLocation>,
    visibility: i32,
) -> Result<Response<UpsertTwinResponse>, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let payload = UpsertTwinRequestPayload {
        twin_id: did.to_string(),
        labels,
        properties,
        feeds,
        location,
        visibility,
        ..Default::default()
    };

    let mut request = Request::new(UpsertTwinRequest {
        headers: Some(headers.clone()),
        payload: Some(payload),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let response = client.upsert_twin(request).await?;

    Ok(response)
}
