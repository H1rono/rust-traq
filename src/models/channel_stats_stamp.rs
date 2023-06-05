/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChannelStatsStamp : チャンネル上の特定スタンプ統計情報

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelStatsStamp {
    /// スタンプID
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// スタンプ数(同一メッセージ上のものは複数カウントしない)
    #[serde(rename = "count")]
    pub count: i64,
    /// スタンプ数(同一メッセージ上のものも複数カウントする)
    #[serde(rename = "total")]
    pub total: i64,
}

impl ChannelStatsStamp {
    /// チャンネル上の特定スタンプ統計情報
    pub fn new(id: uuid::Uuid, count: i64, total: i64) -> ChannelStatsStamp {
        ChannelStatsStamp { id, count, total }
    }
}
