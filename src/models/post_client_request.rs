/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostClientRequest : OAuth2クライアント作成リクエスト

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostClientRequest {
    /// クライアント名
    #[serde(rename = "name")]
    pub name: String,
    /// コールバックURL
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// 要求スコープの配列
    #[serde(rename = "scopes")]
    pub scopes: Vec<crate::models::OAuth2Scope>,
    /// 説明
    #[serde(rename = "description")]
    pub description: String,
}

impl PostClientRequest {
    /// OAuth2クライアント作成リクエスト
    pub fn new(
        name: String,
        callback_url: String,
        scopes: Vec<crate::models::OAuth2Scope>,
        description: String,
    ) -> PostClientRequest {
        PostClientRequest {
            name,
            callback_url,
            scopes,
            description,
        }
    }
}
