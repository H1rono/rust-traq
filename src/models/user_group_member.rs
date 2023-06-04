/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// UserGroupMember : ユーザーグループメンバー

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserGroupMember {
    /// ユーザーUUID
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// ユーザーの役割
    #[serde(rename = "role")]
    pub role: String,
}

impl UserGroupMember {
    /// ユーザーグループメンバー
    pub fn new(id: uuid::Uuid, role: String) -> UserGroupMember {
        UserGroupMember { id, role }
    }
}
