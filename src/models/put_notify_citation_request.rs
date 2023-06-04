/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PutNotifyCitationRequest : メッセージ引用通知設定リクエスト

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutNotifyCitationRequest {
    /// メッセージ引用通知の設定情報
    #[serde(rename = "notifyCitation")]
    pub notify_citation: bool,
}

impl PutNotifyCitationRequest {
    /// メッセージ引用通知設定リクエスト
    pub fn new(notify_citation: bool) -> PutNotifyCitationRequest {
        PutNotifyCitationRequest { notify_citation }
    }
}
