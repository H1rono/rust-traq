/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// OgpMedia : OGPに含まれる画像の情報

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OgpMedia {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "secureUrl", deserialize_with = "Option::deserialize")]
    pub secure_url: Option<String>,
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<String>,
    #[serde(rename = "width", deserialize_with = "Option::deserialize")]
    pub width: Option<i32>,
    #[serde(rename = "height", deserialize_with = "Option::deserialize")]
    pub height: Option<i32>,
}

impl OgpMedia {
    /// OGPに含まれる画像の情報
    pub fn new(
        url: String,
        secure_url: Option<String>,
        r#type: Option<String>,
        width: Option<i32>,
        height: Option<i32>,
    ) -> OgpMedia {
        OgpMedia {
            url,
            secure_url,
            r#type,
            width,
            height,
        }
    }
}
