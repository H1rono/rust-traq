/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VersionFlags {
    /// 有効な外部ログインプロバイダ
    #[serde(rename = "externalLogin")]
    pub external_login: Vec<String>,
    /// ユーザーが自身で新規登録(POST /api/v3/users)可能か
    #[serde(rename = "signUpAllowed")]
    pub sign_up_allowed: bool,
}

impl VersionFlags {
    pub fn new(external_login: Vec<String>, sign_up_allowed: bool) -> VersionFlags {
        VersionFlags {
            external_login,
            sign_up_allowed,
        }
    }
}