/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FileInfo : ファイル情報

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FileInfo {
    /// ファイルUUID
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// ファイル名
    #[serde(rename = "name")]
    pub name: String,
    /// MIMEタイプ
    #[serde(rename = "mime")]
    pub mime: String,
    /// ファイルサイズ
    #[serde(rename = "size")]
    pub size: i64,
    /// MD5ハッシュ
    #[serde(rename = "md5")]
    pub md5: String,
    /// アニメーション画像かどうか
    #[serde(rename = "isAnimatedImage")]
    pub is_animated_image: bool,
    /// アップロード日時
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<crate::models::ThumbnailInfo>,
    #[serde(rename = "thumbnail", deserialize_with = "Option::deserialize")]
    pub thumbnail: Option<Box<crate::models::FileInfoThumbnail>>,
    /// 属しているチャンネルUUID
    #[serde(rename = "channelId", deserialize_with = "Option::deserialize")]
    pub channel_id: Option<uuid::Uuid>,
    /// アップロード者UUID
    #[serde(rename = "uploaderId", deserialize_with = "Option::deserialize")]
    pub uploader_id: Option<uuid::Uuid>,
}

impl FileInfo {
    /// ファイル情報
    pub fn new(
        id: uuid::Uuid,
        name: String,
        mime: String,
        size: i64,
        md5: String,
        is_animated_image: bool,
        created_at: String,
        thumbnails: Vec<crate::models::ThumbnailInfo>,
        thumbnail: Option<crate::models::FileInfoThumbnail>,
        channel_id: Option<uuid::Uuid>,
        uploader_id: Option<uuid::Uuid>,
    ) -> FileInfo {
        FileInfo {
            id,
            name,
            mime,
            size,
            md5,
            is_animated_image,
            created_at,
            thumbnails,
            thumbnail: thumbnail.map(Box::new),
            channel_id,
            uploader_id,
        }
    }
}
