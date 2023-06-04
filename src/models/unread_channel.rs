/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// UnreadChannel : 未読チャンネル情報

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnreadChannel {
    /// チャンネルUUID
    #[serde(rename = "channelId")]
    pub channel_id: uuid::Uuid,
    /// 未読メッセージ数
    #[serde(rename = "count")]
    pub count: i32,
    /// 自分宛てメッセージが含まれているかどうか
    #[serde(rename = "noticeable")]
    pub noticeable: bool,
    /// チャンネルの最古の未読メッセージの日時
    #[serde(rename = "since")]
    pub since: String,
    /// チャンネルの最新の未読メッセージの日時
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// そのチャンネルの未読の中で最も古いメッセージのid
    #[serde(rename = "oldestMessageId")]
    pub oldest_message_id: uuid::Uuid,
}

impl UnreadChannel {
    /// 未読チャンネル情報
    pub fn new(
        channel_id: uuid::Uuid,
        count: i32,
        noticeable: bool,
        since: String,
        updated_at: String,
        oldest_message_id: uuid::Uuid,
    ) -> UnreadChannel {
        UnreadChannel {
            channel_id,
            count,
            noticeable,
            since,
            updated_at,
            oldest_message_id,
        }
    }
}
