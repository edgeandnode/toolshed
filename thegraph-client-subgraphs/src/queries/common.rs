use thegraph_core::BlockPointer;
use thegraph_graphql_http::{
    http::request::IntoRequestParameters,
    http_client::{ReqwestExt, ResponseResult},
};
use url::Url;

#[derive(Debug, serde::Deserialize)]
pub struct Meta {
    pub block: BlockPointer,
}

/// Send an authenticated GraphQL query to a subgraph.
pub async fn send_query<T>(
    client: &reqwest::Client,
    url: Url,
    auth: Option<&str>,
    query: impl IntoRequestParameters + Send,
) -> Result<ResponseResult<T>, String>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let mut builder = client.post(url);

    if let Some(auth) = auth {
        builder = builder.bearer_auth(auth)
    }

    let res = builder
        .send_graphql(query)
        .await
        .map_err(|err| err.to_string())?;

    Ok(res)
}

pub async fn send_subgraph_query<T>(
    client: &reqwest::Client,
    subgraph_url: Url,
    auth: Option<&str>,
    query: impl IntoRequestParameters + Send,
) -> Result<T, String>
where
    T: for<'de> serde::Deserialize<'de>,
{
    send_query(client, subgraph_url, auth, query)
        .await
        .map_err(|err| format!("Error sending subgraph graphql query: {}", err))?
        .map_err(|err| err.to_string())
}
