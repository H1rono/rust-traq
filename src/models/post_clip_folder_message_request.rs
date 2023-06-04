/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostClipFolderMessageRequest : クリップ追加リクエスト

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostClipFolderMessageRequest {
    /// メッセージUUID
    #[serde(rename = "messageId")]
    pub message_id: uuid::Uuid,
}

impl PostClipFolderMessageRequest {
    /// クリップ追加リクエスト
    pub fn new(message_id: uuid::Uuid) -> PostClipFolderMessageRequest {
        PostClipFolderMessageRequest { message_id }
    }
}
