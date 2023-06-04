/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`create_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateClientError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteClientError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditClientError {
    Status400(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_clients`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_tokens`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyTokensError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_o_auth2_authorize`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOAuth2AuthorizeError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_o_auth2_authorize`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostOAuth2AuthorizeError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_o_auth2_authorize_decide`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostOAuth2AuthorizeDecideError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_o_auth2_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostOAuth2TokenError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`revoke_my_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevokeMyTokenError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`revoke_o_auth2_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevokeOAuth2TokenError {
    UnknownValue(serde_json::Value),
}

/// OAuth2クライアントを作成します。
pub async fn create_client(
    configuration: &configuration::Configuration,
    post_client_request: Option<crate::models::PostClientRequest>,
) -> Result<crate::models::OAuth2ClientDetail, Error<CreateClientError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/clients", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_client_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateClientError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したOAuth2クライアントを削除します。 対象のクライアントの管理権限が必要です。正常に削除された場合、このクライアントに対する認可は全て取り消されます。
pub async fn delete_client(
    configuration: &configuration::Configuration,
    client_id: &str,
) -> Result<(), Error<DeleteClientError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clients/{clientId}",
        local_var_configuration.base_path,
        clientId = crate::apis::urlencode(client_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteClientError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したOAuth2クライアントの情報を変更します。 対象のクライアントの管理権限が必要です。 クライアント開発者UUIDを変更した場合は、変更先ユーザーにクライアント管理権限が移譲され、自分自身は権限を失います。
pub async fn edit_client(
    configuration: &configuration::Configuration,
    client_id: &str,
    patch_client_request: Option<crate::models::PatchClientRequest>,
) -> Result<(), Error<EditClientError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clients/{clientId}",
        local_var_configuration.base_path,
        clientId = crate::apis::urlencode(client_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&patch_client_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<EditClientError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したOAuth2クライアントの情報を取得します。 詳細情報の取得には対象のクライアントの管理権限が必要です。
pub async fn get_client(
    configuration: &configuration::Configuration,
    client_id: &str,
    detail: Option<bool>,
) -> Result<crate::models::GetClient200Response, Error<GetClientError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clients/{clientId}",
        local_var_configuration.base_path,
        clientId = crate::apis::urlencode(client_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = detail {
        local_var_req_builder =
            local_var_req_builder.query(&[("detail", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetClientError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自身が開発者のOAuth2クライアントのリストを取得します。 `all`が`true`の場合、全開発者の全クライアントのリストを返します。
pub async fn get_clients(
    configuration: &configuration::Configuration,
    all: Option<bool>,
) -> Result<Vec<crate::models::OAuth2Client>, Error<GetClientsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/clients", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = all {
        local_var_req_builder = local_var_req_builder.query(&[("all", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetClientsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 有効な自分に発行されたOAuth2トークンのリストを取得します。
pub async fn get_my_tokens(
    configuration: &configuration::Configuration,
) -> Result<Vec<crate::models::ActiveOAuth2Token>, Error<GetMyTokensError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/me/tokens", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMyTokensError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// OAuth2 認可エンドポイント
pub async fn get_o_auth2_authorize(
    configuration: &configuration::Configuration,
    client_id: &str,
    response_type: Option<OAuth2ResponseType>,
    redirect_uri: Option<&str>,
    scope: Option<&str>,
    state: Option<&str>,
    code_challenge: Option<&str>,
    code_challenge_method: Option<&str>,
    nonce: Option<&str>,
    prompt: Option<OAuth2Prompt>,
) -> Result<(), Error<GetOAuth2AuthorizeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth2/authorize", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = response_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("response_type", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("client_id", &client_id.to_string())]);
    if let Some(ref local_var_str) = redirect_uri {
        local_var_req_builder =
            local_var_req_builder.query(&[("redirect_uri", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = scope {
        local_var_req_builder =
            local_var_req_builder.query(&[("scope", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = state {
        local_var_req_builder =
            local_var_req_builder.query(&[("state", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = code_challenge {
        local_var_req_builder =
            local_var_req_builder.query(&[("code_challenge", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = code_challenge_method {
        local_var_req_builder =
            local_var_req_builder.query(&[("code_challenge_method", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = nonce {
        local_var_req_builder =
            local_var_req_builder.query(&[("nonce", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prompt {
        local_var_req_builder =
            local_var_req_builder.query(&[("prompt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetOAuth2AuthorizeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// OAuth2 認可エンドポイント
pub async fn post_o_auth2_authorize(
    configuration: &configuration::Configuration,
    client_id: &str,
    response_type: Option<crate::models::OAuth2ResponseType>,
    redirect_uri: Option<&str>,
    scope: Option<&str>,
    state: Option<&str>,
    code_challenge: Option<&str>,
    code_challenge_method: Option<&str>,
    nonce: Option<&str>,
    prompt: Option<crate::models::OAuth2Prompt>,
) -> Result<(), Error<PostOAuth2AuthorizeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth2/authorize", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = response_type {
        local_var_form_params.insert("response_type", local_var_param_value.to_string());
    }
    local_var_form_params.insert("client_id", client_id.to_string());
    if let Some(local_var_param_value) = redirect_uri {
        local_var_form_params.insert("redirect_uri", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = scope {
        local_var_form_params.insert("scope", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = state {
        local_var_form_params.insert("state", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = code_challenge {
        local_var_form_params.insert("code_challenge", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = code_challenge_method {
        local_var_form_params.insert("code_challenge_method", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = nonce {
        local_var_form_params.insert("nonce", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = prompt {
        local_var_form_params.insert("prompt", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PostOAuth2AuthorizeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// OAuth2 認可承諾
pub async fn post_o_auth2_authorize_decide(
    configuration: &configuration::Configuration,
    submit: &str,
) -> Result<(), Error<PostOAuth2AuthorizeDecideError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/oauth2/authorize/decide",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("submit", submit.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PostOAuth2AuthorizeDecideError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// OAuth2 トークンエンドポイント
pub async fn post_o_auth2_token(
    configuration: &configuration::Configuration,
    grant_type: &str,
    code: Option<&str>,
    redirect_uri: Option<&str>,
    client_id: Option<&str>,
    code_verifier: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
    scope: Option<&str>,
    refresh_token: Option<&str>,
    client_secret: Option<&str>,
) -> Result<crate::models::OAuth2Token, Error<PostOAuth2TokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth2/token", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("grant_type", grant_type.to_string());
    if let Some(local_var_param_value) = code {
        local_var_form_params.insert("code", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = redirect_uri {
        local_var_form_params.insert("redirect_uri", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_id {
        local_var_form_params.insert("client_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = code_verifier {
        local_var_form_params.insert("code_verifier", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = username {
        local_var_form_params.insert("username", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = password {
        local_var_form_params.insert("password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = scope {
        local_var_form_params.insert("scope", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = refresh_token {
        local_var_form_params.insert("refresh_token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_secret {
        local_var_form_params.insert("client_secret", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostOAuth2TokenError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自分の指定したトークンの認可を取り消します。
pub async fn revoke_my_token(
    configuration: &configuration::Configuration,
    token_id: &str,
) -> Result<(), Error<RevokeMyTokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/users/me/tokens/{tokenId}",
        local_var_configuration.base_path,
        tokenId = crate::apis::urlencode(token_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RevokeMyTokenError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// OAuth2 トークン無効化エンドポイント
pub async fn revoke_o_auth2_token(
    configuration: &configuration::Configuration,
    token: &str,
) -> Result<(), Error<RevokeOAuth2TokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth2/revoke", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("token", token.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RevokeOAuth2TokenError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
