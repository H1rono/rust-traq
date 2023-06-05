/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostLinkExternalAccount : POST /users/me/ex-accounts/link 用リクエストボディ

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostLinkExternalAccount {
    /// 外部サービス名
    #[serde(rename = "providerName")]
    pub provider_name: String,
}

impl PostLinkExternalAccount {
    /// POST /users/me/ex-accounts/link 用リクエストボディ
    pub fn new(provider_name: String) -> PostLinkExternalAccount {
        PostLinkExternalAccount { provider_name }
    }
}
