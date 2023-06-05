/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ParentChangedEvent : 親チャンネル変更イベント

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParentChangedEvent {
    /// 変更者UUID
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
    /// 変更前親チャンネルUUID
    #[serde(rename = "before")]
    pub before: uuid::Uuid,
    /// 変更後親チャンネルUUID
    #[serde(rename = "after")]
    pub after: uuid::Uuid,
}

impl ParentChangedEvent {
    /// 親チャンネル変更イベント
    pub fn new(user_id: uuid::Uuid, before: uuid::Uuid, after: uuid::Uuid) -> ParentChangedEvent {
        ParentChangedEvent {
            user_id,
            before,
            after,
        }
    }
}
