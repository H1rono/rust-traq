/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChannelEvent : チャンネルイベント

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelEvent {
    /// イベントタイプ
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// イベント日時
    #[serde(rename = "datetime")]
    pub datetime: String,
    #[serde(rename = "detail")]
    pub detail: Box<crate::models::ChannelEventDetail>,
}

impl ChannelEvent {
    /// チャンネルイベント
    pub fn new(
        r#type: RHashType,
        datetime: String,
        detail: crate::models::ChannelEventDetail,
    ) -> ChannelEvent {
        ChannelEvent {
            r#type,
            datetime,
            detail: Box::new(detail),
        }
    }
}

/// イベントタイプ
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "TopicChanged")]
    TopicChanged,
    #[serde(rename = "SubscribersChanged")]
    SubscribersChanged,
    #[serde(rename = "PinAdded")]
    PinAdded,
    #[serde(rename = "PinRemoved")]
    PinRemoved,
    #[serde(rename = "NameChanged")]
    NameChanged,
    #[serde(rename = "ParentChanged")]
    ParentChanged,
    #[serde(rename = "VisibilityChanged")]
    VisibilityChanged,
    #[serde(rename = "ForcedNotificationChanged")]
    ForcedNotificationChanged,
    #[serde(rename = "ChildCreated")]
    ChildCreated,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::TopicChanged
    }
}
