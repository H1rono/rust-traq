/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ExternalProviderUser : 外部認証アカウントユーザー

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExternalProviderUser {
    /// 外部サービス名
    #[serde(rename = "providerName")]
    pub provider_name: String,
    /// 紐付けた日時
    #[serde(rename = "linkedAt")]
    pub linked_at: String,
    /// 外部アカウント名
    #[serde(rename = "externalName")]
    pub external_name: String,
}

impl ExternalProviderUser {
    /// 外部認証アカウントユーザー
    pub fn new(
        provider_name: String,
        linked_at: String,
        external_name: String,
    ) -> ExternalProviderUser {
        ExternalProviderUser {
            provider_name,
            linked_at,
            external_name,
        }
    }
}
