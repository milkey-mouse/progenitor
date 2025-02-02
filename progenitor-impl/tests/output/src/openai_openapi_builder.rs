#[allow(unused_imports)]
use progenitor_client::{encode_path, to_form_string, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub use reqwest::multipart::Part;
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///CreateImageEditBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "image",
    ///    "prompt"
    ///  ],
    ///  "properties": {
    ///    "n": {
    ///      "description": "The number of images to generate. Must be between 1
    /// and 10.",
    ///      "default": 1,
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "maximum": 10.0,
    ///      "minimum": 1.0
    ///    },
    ///    "prompt": {
    ///      "description": "A text description of the desired image(s). The
    /// maximum length is 1000 characters.",
    ///      "examples": [
    ///        "A cute baby sea otter wearing a beret"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "response_format": {
    ///      "description": "The format in which the generated images are
    /// returned. Must be one of `url` or `b64_json`.",
    ///      "default": "url",
    ///      "examples": [
    ///        "url"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "enum": [
    ///        "url",
    ///        "b64_json"
    ///      ]
    ///    },
    ///    "size": {
    ///      "description": "The size of the generated images. Must be one of
    /// `256x256`, `512x512`, or `1024x1024`.",
    ///      "default": "1024x1024",
    ///      "examples": [
    ///        "1024x1024"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "enum": [
    ///        "256x256",
    ///        "512x512",
    ///        "1024x1024"
    ///      ]
    ///    },
    ///    "user": {
    ///      "description": "A unique identifier representing your end-user,
    /// which can help OpenAI to monitor and detect abuse. [Learn
    /// more](/docs/guides/safety-best-practices/end-user-ids).\n",
    ///      "examples": [
    ///        "user-1234"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct CreateImageEditBody {
        ///The number of images to generate. Must be between 1 and 10.
        #[serde(default = "defaults::create_image_edit_body_n")]
        pub n: Option<i64>,
        ///A text description of the desired image(s). The maximum length is
        /// 1000 characters.
        pub prompt: String,
        ///The format in which the generated images are returned. Must be one
        /// of `url` or `b64_json`.
        #[serde(default = "defaults::create_image_edit_body_response_format")]
        pub response_format: Option<CreateImageEditBodyResponseFormat>,
        ///The size of the generated images. Must be one of `256x256`,
        /// `512x512`, or `1024x1024`.
        #[serde(default = "defaults::create_image_edit_body_size")]
        pub size: Option<CreateImageEditBodySize>,
        ///A unique identifier representing your end-user, which can help
        /// OpenAI to monitor and detect abuse. [Learn
        /// more](/docs/guides/safety-best-practices/end-user-ids).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
    }

    impl From<&CreateImageEditBody> for CreateImageEditBody {
        fn from(value: &CreateImageEditBody) -> Self {
            value.clone()
        }
    }

    impl CreateImageEditBody {
        pub fn builder() -> builder::CreateImageEditBody {
            Default::default()
        }
    }

    ///The format in which the generated images are returned. Must be one of
    /// `url` or `b64_json`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The format in which the generated images are returned.
    /// Must be one of `url` or `b64_json`.",
    ///  "default": "url",
    ///  "examples": [
    ///    "url"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "url",
    ///    "b64_json"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum CreateImageEditBodyResponseFormat {
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "b64_json")]
        B64Json,
    }

    impl From<&CreateImageEditBodyResponseFormat> for CreateImageEditBodyResponseFormat {
        fn from(value: &CreateImageEditBodyResponseFormat) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageEditBodyResponseFormat {
        fn to_string(&self) -> String {
            match *self {
                Self::Url => "url".to_string(),
                Self::B64Json => "b64_json".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageEditBodyResponseFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "url" => Ok(Self::Url),
                "b64_json" => Ok(Self::B64Json),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CreateImageEditBodyResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageEditBodyResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageEditBodyResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///The size of the generated images. Must be one of `256x256`, `512x512`,
    /// or `1024x1024`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The size of the generated images. Must be one of
    /// `256x256`, `512x512`, or `1024x1024`.",
    ///  "default": "1024x1024",
    ///  "examples": [
    ///    "1024x1024"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "256x256",
    ///    "512x512",
    ///    "1024x1024"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum CreateImageEditBodySize {
        #[serde(rename = "256x256")]
        _256x256,
        #[serde(rename = "512x512")]
        _512x512,
        #[serde(rename = "1024x1024")]
        _1024x1024,
    }

    impl From<&CreateImageEditBodySize> for CreateImageEditBodySize {
        fn from(value: &CreateImageEditBodySize) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageEditBodySize {
        fn to_string(&self) -> String {
            match *self {
                Self::_256x256 => "256x256".to_string(),
                Self::_512x512 => "512x512".to_string(),
                Self::_1024x1024 => "1024x1024".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageEditBodySize {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "256x256" => Ok(Self::_256x256),
                "512x512" => Ok(Self::_512x512),
                "1024x1024" => Ok(Self::_1024x1024),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CreateImageEditBodySize {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageEditBodySize {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageEditBodySize {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///CreateImageRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "prompt"
    ///  ],
    ///  "properties": {
    ///    "n": {
    ///      "description": "The number of images to generate. Must be between 1
    /// and 10.",
    ///      "default": 1,
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "maximum": 10.0,
    ///      "minimum": 1.0
    ///    },
    ///    "prompt": {
    ///      "description": "A text description of the desired image(s). The
    /// maximum length is 1000 characters.",
    ///      "examples": [
    ///        "A cute baby sea otter"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "response_format": {
    ///      "description": "The format in which the generated images are
    /// returned. Must be one of `url` or `b64_json`.",
    ///      "default": "url",
    ///      "examples": [
    ///        "url"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "enum": [
    ///        "url",
    ///        "b64_json"
    ///      ]
    ///    },
    ///    "size": {
    ///      "description": "The size of the generated images. Must be one of
    /// `256x256`, `512x512`, or `1024x1024`.",
    ///      "default": "1024x1024",
    ///      "examples": [
    ///        "1024x1024"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "enum": [
    ///        "256x256",
    ///        "512x512",
    ///        "1024x1024"
    ///      ]
    ///    },
    ///    "user": {
    ///      "description": "A unique identifier representing your end-user,
    /// which can help OpenAI to monitor and detect abuse. [Learn
    /// more](/docs/guides/safety-best-practices/end-user-ids).\n",
    ///      "examples": [
    ///        "user-1234"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct CreateImageRequest {
        ///The number of images to generate. Must be between 1 and 10.
        #[serde(default = "defaults::create_image_request_n")]
        pub n: Option<i64>,
        ///A text description of the desired image(s). The maximum length is
        /// 1000 characters.
        pub prompt: String,
        ///The format in which the generated images are returned. Must be one
        /// of `url` or `b64_json`.
        #[serde(default = "defaults::create_image_request_response_format")]
        pub response_format: Option<CreateImageRequestResponseFormat>,
        ///The size of the generated images. Must be one of `256x256`,
        /// `512x512`, or `1024x1024`.
        #[serde(default = "defaults::create_image_request_size")]
        pub size: Option<CreateImageRequestSize>,
        ///A unique identifier representing your end-user, which can help
        /// OpenAI to monitor and detect abuse. [Learn
        /// more](/docs/guides/safety-best-practices/end-user-ids).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
    }

    impl From<&CreateImageRequest> for CreateImageRequest {
        fn from(value: &CreateImageRequest) -> Self {
            value.clone()
        }
    }

    impl CreateImageRequest {
        pub fn builder() -> builder::CreateImageRequest {
            Default::default()
        }
    }

    ///The format in which the generated images are returned. Must be one of
    /// `url` or `b64_json`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The format in which the generated images are returned.
    /// Must be one of `url` or `b64_json`.",
    ///  "default": "url",
    ///  "examples": [
    ///    "url"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "url",
    ///    "b64_json"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum CreateImageRequestResponseFormat {
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "b64_json")]
        B64Json,
    }

    impl From<&CreateImageRequestResponseFormat> for CreateImageRequestResponseFormat {
        fn from(value: &CreateImageRequestResponseFormat) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageRequestResponseFormat {
        fn to_string(&self) -> String {
            match *self {
                Self::Url => "url".to_string(),
                Self::B64Json => "b64_json".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageRequestResponseFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "url" => Ok(Self::Url),
                "b64_json" => Ok(Self::B64Json),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CreateImageRequestResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageRequestResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageRequestResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///The size of the generated images. Must be one of `256x256`, `512x512`,
    /// or `1024x1024`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The size of the generated images. Must be one of
    /// `256x256`, `512x512`, or `1024x1024`.",
    ///  "default": "1024x1024",
    ///  "examples": [
    ///    "1024x1024"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "256x256",
    ///    "512x512",
    ///    "1024x1024"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum CreateImageRequestSize {
        #[serde(rename = "256x256")]
        _256x256,
        #[serde(rename = "512x512")]
        _512x512,
        #[serde(rename = "1024x1024")]
        _1024x1024,
    }

    impl From<&CreateImageRequestSize> for CreateImageRequestSize {
        fn from(value: &CreateImageRequestSize) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageRequestSize {
        fn to_string(&self) -> String {
            match *self {
                Self::_256x256 => "256x256".to_string(),
                Self::_512x512 => "512x512".to_string(),
                Self::_1024x1024 => "1024x1024".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageRequestSize {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "256x256" => Ok(Self::_256x256),
                "512x512" => Ok(Self::_512x512),
                "1024x1024" => Ok(Self::_1024x1024),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CreateImageRequestSize {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageRequestSize {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageRequestSize {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///CreateImageVariationBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "image"
    ///  ],
    ///  "properties": {
    ///    "n": {
    ///      "description": "The number of images to generate. Must be between 1
    /// and 10.",
    ///      "default": 1,
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "maximum": 10.0,
    ///      "minimum": 1.0
    ///    },
    ///    "response_format": {
    ///      "description": "The format in which the generated images are
    /// returned. Must be one of `url` or `b64_json`.",
    ///      "default": "url",
    ///      "examples": [
    ///        "url"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "enum": [
    ///        "url",
    ///        "b64_json"
    ///      ]
    ///    },
    ///    "size": {
    ///      "description": "The size of the generated images. Must be one of
    /// `256x256`, `512x512`, or `1024x1024`.",
    ///      "default": "1024x1024",
    ///      "examples": [
    ///        "1024x1024"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "enum": [
    ///        "256x256",
    ///        "512x512",
    ///        "1024x1024"
    ///      ]
    ///    },
    ///    "user": {
    ///      "description": "A unique identifier representing your end-user,
    /// which can help OpenAI to monitor and detect abuse. [Learn
    /// more](/docs/guides/safety-best-practices/end-user-ids).\n",
    ///      "examples": [
    ///        "user-1234"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct CreateImageVariationBody {
        ///The number of images to generate. Must be between 1 and 10.
        #[serde(default = "defaults::create_image_variation_body_n")]
        pub n: Option<i64>,
        ///The format in which the generated images are returned. Must be one
        /// of `url` or `b64_json`.
        #[serde(default = "defaults::create_image_variation_body_response_format")]
        pub response_format: Option<CreateImageVariationBodyResponseFormat>,
        ///The size of the generated images. Must be one of `256x256`,
        /// `512x512`, or `1024x1024`.
        #[serde(default = "defaults::create_image_variation_body_size")]
        pub size: Option<CreateImageVariationBodySize>,
        ///A unique identifier representing your end-user, which can help
        /// OpenAI to monitor and detect abuse. [Learn
        /// more](/docs/guides/safety-best-practices/end-user-ids).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
    }

    impl From<&CreateImageVariationBody> for CreateImageVariationBody {
        fn from(value: &CreateImageVariationBody) -> Self {
            value.clone()
        }
    }

    impl CreateImageVariationBody {
        pub fn builder() -> builder::CreateImageVariationBody {
            Default::default()
        }
    }

    ///The format in which the generated images are returned. Must be one of
    /// `url` or `b64_json`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The format in which the generated images are returned.
    /// Must be one of `url` or `b64_json`.",
    ///  "default": "url",
    ///  "examples": [
    ///    "url"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "url",
    ///    "b64_json"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum CreateImageVariationBodyResponseFormat {
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "b64_json")]
        B64Json,
    }

    impl From<&CreateImageVariationBodyResponseFormat> for CreateImageVariationBodyResponseFormat {
        fn from(value: &CreateImageVariationBodyResponseFormat) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageVariationBodyResponseFormat {
        fn to_string(&self) -> String {
            match *self {
                Self::Url => "url".to_string(),
                Self::B64Json => "b64_json".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageVariationBodyResponseFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "url" => Ok(Self::Url),
                "b64_json" => Ok(Self::B64Json),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CreateImageVariationBodyResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageVariationBodyResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageVariationBodyResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///The size of the generated images. Must be one of `256x256`, `512x512`,
    /// or `1024x1024`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The size of the generated images. Must be one of
    /// `256x256`, `512x512`, or `1024x1024`.",
    ///  "default": "1024x1024",
    ///  "examples": [
    ///    "1024x1024"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "256x256",
    ///    "512x512",
    ///    "1024x1024"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum CreateImageVariationBodySize {
        #[serde(rename = "256x256")]
        _256x256,
        #[serde(rename = "512x512")]
        _512x512,
        #[serde(rename = "1024x1024")]
        _1024x1024,
    }

    impl From<&CreateImageVariationBodySize> for CreateImageVariationBodySize {
        fn from(value: &CreateImageVariationBodySize) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageVariationBodySize {
        fn to_string(&self) -> String {
            match *self {
                Self::_256x256 => "256x256".to_string(),
                Self::_512x512 => "512x512".to_string(),
                Self::_1024x1024 => "1024x1024".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageVariationBodySize {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "256x256" => Ok(Self::_256x256),
                "512x512" => Ok(Self::_512x512),
                "1024x1024" => Ok(Self::_1024x1024),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CreateImageVariationBodySize {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageVariationBodySize {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageVariationBodySize {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Error
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message",
    ///    "param",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "param": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct Error {
        pub code: Option<String>,
        pub message: String,
        pub param: Option<String>,
        #[serde(rename = "type")]
        pub type_: String,
    }

    impl From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }

    impl Error {
        pub fn builder() -> builder::Error {
            Default::default()
        }
    }

    ///ErrorResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "$ref": "#/components/schemas/Error"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ErrorResponse {
        pub error: Error,
    }

    impl From<&ErrorResponse> for ErrorResponse {
        fn from(value: &ErrorResponse) -> Self {
            value.clone()
        }
    }

    impl ErrorResponse {
        pub fn builder() -> builder::ErrorResponse {
            Default::default()
        }
    }

    ///Represents the url or the content of an image generated by the OpenAI
    /// API.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Represents the url or the content of an image generated
    /// by the OpenAI API.",
    ///  "type": "object",
    ///  "properties": {
    ///    "b64_json": {
    ///      "description": "The base64-encoded JSON of the generated image, if
    /// `response_format` is `b64_json`.",
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "description": "The URL of the generated image, if
    /// `response_format` is `url` (default).",
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-oaiMeta": {
    ///    "example": "{\n  \"url\": \"...\"\n}\n",
    ///    "name": "The image object"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct Image {
        ///The base64-encoded JSON of the generated image, if `response_format`
        /// is `b64_json`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub b64_json: Option<String>,
        ///The URL of the generated image, if `response_format` is `url`
        /// (default).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }

    impl From<&Image> for Image {
        fn from(value: &Image) -> Self {
            value.clone()
        }
    }

    impl Image {
        pub fn builder() -> builder::Image {
            Default::default()
        }
    }

    ///ImagesResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "required": [
    ///    "created",
    ///    "data"
    ///  ],
    ///  "properties": {
    ///    "created": {
    ///      "type": "integer"
    ///    },
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Image"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ImagesResponse {
        pub created: i64,
        pub data: Vec<Image>,
    }

    impl From<&ImagesResponse> for ImagesResponse {
        fn from(value: &ImagesResponse) -> Self {
            value.clone()
        }
    }

    impl ImagesResponse {
        pub fn builder() -> builder::ImagesResponse {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct CreateImageEditBody {
            n: Result<Option<i64>, String>,
            prompt: Result<String, String>,
            response_format: Result<Option<super::CreateImageEditBodyResponseFormat>, String>,
            size: Result<Option<super::CreateImageEditBodySize>, String>,
            user: Result<Option<String>, String>,
        }

        impl Default for CreateImageEditBody {
            fn default() -> Self {
                Self {
                    n: Ok(super::defaults::create_image_edit_body_n()),
                    prompt: Err("no value supplied for prompt".to_string()),
                    response_format: Ok(super::defaults::create_image_edit_body_response_format()),
                    size: Ok(super::defaults::create_image_edit_body_size()),
                    user: Ok(Default::default()),
                }
            }
        }

        impl CreateImageEditBody {
            pub fn n<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.n = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for n: {}", e));
                self
            }
            pub fn prompt<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.prompt = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for prompt: {}", e));
                self
            }
            pub fn response_format<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::CreateImageEditBodyResponseFormat>>,
                T::Error: std::fmt::Display,
            {
                self.response_format = value.try_into().map_err(|e| {
                    format!("error converting supplied value for response_format: {}", e)
                });
                self
            }
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::CreateImageEditBodySize>>,
                T::Error: std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
            pub fn user<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.user = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateImageEditBody> for super::CreateImageEditBody {
            type Error = super::error::ConversionError;
            fn try_from(value: CreateImageEditBody) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    n: value.n?,
                    prompt: value.prompt?,
                    response_format: value.response_format?,
                    size: value.size?,
                    user: value.user?,
                })
            }
        }

        impl From<super::CreateImageEditBody> for CreateImageEditBody {
            fn from(value: super::CreateImageEditBody) -> Self {
                Self {
                    n: Ok(value.n),
                    prompt: Ok(value.prompt),
                    response_format: Ok(value.response_format),
                    size: Ok(value.size),
                    user: Ok(value.user),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateImageRequest {
            n: Result<Option<i64>, String>,
            prompt: Result<String, String>,
            response_format: Result<Option<super::CreateImageRequestResponseFormat>, String>,
            size: Result<Option<super::CreateImageRequestSize>, String>,
            user: Result<Option<String>, String>,
        }

        impl Default for CreateImageRequest {
            fn default() -> Self {
                Self {
                    n: Ok(super::defaults::create_image_request_n()),
                    prompt: Err("no value supplied for prompt".to_string()),
                    response_format: Ok(super::defaults::create_image_request_response_format()),
                    size: Ok(super::defaults::create_image_request_size()),
                    user: Ok(Default::default()),
                }
            }
        }

        impl CreateImageRequest {
            pub fn n<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.n = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for n: {}", e));
                self
            }
            pub fn prompt<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.prompt = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for prompt: {}", e));
                self
            }
            pub fn response_format<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::CreateImageRequestResponseFormat>>,
                T::Error: std::fmt::Display,
            {
                self.response_format = value.try_into().map_err(|e| {
                    format!("error converting supplied value for response_format: {}", e)
                });
                self
            }
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::CreateImageRequestSize>>,
                T::Error: std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
            pub fn user<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.user = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateImageRequest> for super::CreateImageRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: CreateImageRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    n: value.n?,
                    prompt: value.prompt?,
                    response_format: value.response_format?,
                    size: value.size?,
                    user: value.user?,
                })
            }
        }

        impl From<super::CreateImageRequest> for CreateImageRequest {
            fn from(value: super::CreateImageRequest) -> Self {
                Self {
                    n: Ok(value.n),
                    prompt: Ok(value.prompt),
                    response_format: Ok(value.response_format),
                    size: Ok(value.size),
                    user: Ok(value.user),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateImageVariationBody {
            n: Result<Option<i64>, String>,
            response_format: Result<Option<super::CreateImageVariationBodyResponseFormat>, String>,
            size: Result<Option<super::CreateImageVariationBodySize>, String>,
            user: Result<Option<String>, String>,
        }

        impl Default for CreateImageVariationBody {
            fn default() -> Self {
                Self {
                    n: Ok(super::defaults::create_image_variation_body_n()),
                    response_format: Ok(
                        super::defaults::create_image_variation_body_response_format(),
                    ),
                    size: Ok(super::defaults::create_image_variation_body_size()),
                    user: Ok(Default::default()),
                }
            }
        }

        impl CreateImageVariationBody {
            pub fn n<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.n = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for n: {}", e));
                self
            }
            pub fn response_format<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::CreateImageVariationBodyResponseFormat>>,
                T::Error: std::fmt::Display,
            {
                self.response_format = value.try_into().map_err(|e| {
                    format!("error converting supplied value for response_format: {}", e)
                });
                self
            }
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::CreateImageVariationBodySize>>,
                T::Error: std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
            pub fn user<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.user = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateImageVariationBody> for super::CreateImageVariationBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreateImageVariationBody,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    n: value.n?,
                    response_format: value.response_format?,
                    size: value.size?,
                    user: value.user?,
                })
            }
        }

        impl From<super::CreateImageVariationBody> for CreateImageVariationBody {
            fn from(value: super::CreateImageVariationBody) -> Self {
                Self {
                    n: Ok(value.n),
                    response_format: Ok(value.response_format),
                    size: Ok(value.size),
                    user: Ok(value.user),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Error {
            code: Result<Option<String>, String>,
            message: Result<String, String>,
            param: Result<Option<String>, String>,
            type_: Result<String, String>,
        }

        impl Default for Error {
            fn default() -> Self {
                Self {
                    code: Err("no value supplied for code".to_string()),
                    message: Err("no value supplied for message".to_string()),
                    param: Err("no value supplied for param".to_string()),
                    type_: Err("no value supplied for type_".to_string()),
                }
            }
        }

        impl Error {
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn param<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.param = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for param: {}", e));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Error> for super::Error {
            type Error = super::error::ConversionError;
            fn try_from(value: Error) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    code: value.code?,
                    message: value.message?,
                    param: value.param?,
                    type_: value.type_?,
                })
            }
        }

        impl From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    code: Ok(value.code),
                    message: Ok(value.message),
                    param: Ok(value.param),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ErrorResponse {
            error: Result<super::Error, String>,
        }

        impl Default for ErrorResponse {
            fn default() -> Self {
                Self {
                    error: Err("no value supplied for error".to_string()),
                }
            }
        }

        impl ErrorResponse {
            pub fn error<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Error>,
                T::Error: std::fmt::Display,
            {
                self.error = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ErrorResponse> for super::ErrorResponse {
            type Error = super::error::ConversionError;
            fn try_from(value: ErrorResponse) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    error: value.error?,
                })
            }
        }

        impl From<super::ErrorResponse> for ErrorResponse {
            fn from(value: super::ErrorResponse) -> Self {
                Self {
                    error: Ok(value.error),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Image {
            b64_json: Result<Option<String>, String>,
            url: Result<Option<String>, String>,
        }

        impl Default for Image {
            fn default() -> Self {
                Self {
                    b64_json: Ok(Default::default()),
                    url: Ok(Default::default()),
                }
            }
        }

        impl Image {
            pub fn b64_json<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.b64_json = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for b64_json: {}", e));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Image> for super::Image {
            type Error = super::error::ConversionError;
            fn try_from(value: Image) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    b64_json: value.b64_json?,
                    url: value.url?,
                })
            }
        }

        impl From<super::Image> for Image {
            fn from(value: super::Image) -> Self {
                Self {
                    b64_json: Ok(value.b64_json),
                    url: Ok(value.url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ImagesResponse {
            created: Result<i64, String>,
            data: Result<Vec<super::Image>, String>,
        }

        impl Default for ImagesResponse {
            fn default() -> Self {
                Self {
                    created: Err("no value supplied for created".to_string()),
                    data: Err("no value supplied for data".to_string()),
                }
            }
        }

        impl ImagesResponse {
            pub fn created<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i64>,
                T::Error: std::fmt::Display,
            {
                self.created = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created: {}", e));
                self
            }
            pub fn data<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::Image>>,
                T::Error: std::fmt::Display,
            {
                self.data = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for data: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ImagesResponse> for super::ImagesResponse {
            type Error = super::error::ConversionError;
            fn try_from(value: ImagesResponse) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created: value.created?,
                    data: value.data?,
                })
            }
        }

        impl From<super::ImagesResponse> for ImagesResponse {
            fn from(value: super::ImagesResponse) -> Self {
                Self {
                    created: Ok(value.created),
                    data: Ok(value.data),
                }
            }
        }
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn create_image_edit_body_n() -> Option<i64> {
            Some(1_i64)
        }

        pub(super) fn create_image_edit_body_response_format(
        ) -> Option<super::CreateImageEditBodyResponseFormat> {
            Some(super::CreateImageEditBodyResponseFormat::Url)
        }

        pub(super) fn create_image_edit_body_size() -> Option<super::CreateImageEditBodySize> {
            Some(super::CreateImageEditBodySize::_1024x1024)
        }

        pub(super) fn create_image_request_n() -> Option<i64> {
            Some(1_i64)
        }

        pub(super) fn create_image_request_response_format(
        ) -> Option<super::CreateImageRequestResponseFormat> {
            Some(super::CreateImageRequestResponseFormat::Url)
        }

        pub(super) fn create_image_request_size() -> Option<super::CreateImageRequestSize> {
            Some(super::CreateImageRequestSize::_1024x1024)
        }

        pub(super) fn create_image_variation_body_n() -> Option<i64> {
            Some(1_i64)
        }

        pub(super) fn create_image_variation_body_response_format(
        ) -> Option<super::CreateImageVariationBodyResponseFormat> {
            Some(super::CreateImageVariationBodyResponseFormat::Url)
        }

        pub(super) fn create_image_variation_body_size(
        ) -> Option<super::CreateImageVariationBodySize> {
            Some(super::CreateImageVariationBodySize::_1024x1024)
        }
    }
}

#[derive(Clone, Debug)]
///Client for OpenAI API
///
///The OpenAI REST API. Please see https://platform.openai.com/docs/api-reference for more details.
///
///https://openai.com/policies/terms-of-use
///
///Version: 2.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "2.0.0"
    }
}

impl Client {
    ///Creates an image given a prompt
    ///
    ///Sends a `POST` request to `/images/generations`
    ///
    ///```ignore
    /// let response = client.create_image()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_image(&self) -> builder::CreateImage {
        builder::CreateImage::new(self)
    }

    ///Creates an edited or extended image given an original image and a prompt
    ///
    ///Sends a `POST` request to `/images/edits`
    ///
    ///Arguments:
    /// - `body`
    /// - `mask`: An additional image whose fully transparent areas (e.g. where
    ///   alpha is zero) indicate where `image` should be edited. Must be a
    ///   valid PNG file, less than 4MB, and have the same dimensions as
    ///   `image`.
    /// - `image`: The image to edit. Must be a valid PNG file, less than 4MB,
    ///   and square. If mask is not provided, image must have transparency,
    ///   which will be used as the mask.
    ///```ignore
    /// let response = client.create_image_edit()
    ///    .body(body)
    ///    .mask(mask)
    ///    .image(image)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_image_edit(&self) -> builder::CreateImageEdit {
        builder::CreateImageEdit::new(self)
    }

    ///Creates a variation of a given image
    ///
    ///Sends a `POST` request to `/images/variations`
    ///
    ///Arguments:
    /// - `body`
    /// - `image`: The image to use as the basis for the variation(s). Must be a
    ///   valid PNG file, less than 4MB, and square.
    ///```ignore
    /// let response = client.create_image_variation()
    ///    .body(body)
    ///    .image(image)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_image_variation(&self) -> builder::CreateImageVariation {
        builder::CreateImageVariation::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, to_form_string, ByteStream, Error, HeaderMap, HeaderValue, Part,
        RequestBuilderExt, ResponseValue,
    };
    ///Builder for [`Client::create_image`]
    ///
    ///[`Client::create_image`]: super::Client::create_image
    #[derive(Debug, Clone)]
    pub struct CreateImage<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateImageRequest, String>,
    }

    impl<'a> CreateImage<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::CreateImageRequest::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateImageRequest>,
            <V as std::convert::TryInto<types::CreateImageRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `CreateImageRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateImageRequest,
            ) -> types::builder::CreateImageRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/images/generations`
        pub async fn send(self) -> Result<ResponseValue<types::ImagesResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::CreateImageRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/images/generations", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::create_image_edit`]
    ///
    ///[`Client::create_image_edit`]: super::Client::create_image_edit
    #[derive(Debug)]
    pub struct CreateImageEdit<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateImageEditBody, String>,
        mask: Result<Option<Part>, String>,
        image: Result<Part, String>,
    }

    impl<'a> CreateImageEdit<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::CreateImageEditBody::default()),
                mask: Ok(None),
                image: Err("image was not initialized".to_string()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateImageEditBody>,
            <V as std::convert::TryInto<types::CreateImageEditBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `CreateImageEditBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateImageEditBody,
            ) -> types::builder::CreateImageEditBody,
        {
            self.body = self.body.map(f);
            self
        }

        pub fn mask(mut self, value: Option<Part>) -> Self {
            self.mask = Ok(value);
            self
        }

        pub fn image(mut self, value: Part) -> Self {
            self.image = Ok(value);
            self
        }

        ///Sends a `POST` request to `/images/edits`
        pub async fn send(self) -> Result<ResponseValue<types::ImagesResponse>, Error<()>> {
            let Self {
                client,
                body,
                mask,
                image,
            } = self;
            let body = body
                .and_then(|v| types::CreateImageEditBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let mask = mask.map_err(Error::InvalidRequest)?;
            let image = image.map_err(Error::InvalidRequest)?;
            let url = format!("{}/images/edits", client.baseurl,);
            let body = serde_json::to_value(body)
                .map_err(|e| Error::InvalidRequest(e.to_string()))?
                .as_object()
                .cloned()
                .ok_or_else(|| Error::InvalidRequest("Serializing body failed.".to_string()))?;
            let mut form = reqwest::multipart::Form::new().part("image", image);
            if let Some(mask) = mask {
                form = form.part("mask", mask);
            };
            if let Some(v) = body.get("n") {
                let v = to_form_string(v)?;
                form = form.text("n", v);
            };
            if let Some(v) = body.get("prompt") {
                let v = to_form_string(v)?;
                form = form.text("prompt", v);
            };
            if let Some(v) = body.get("response_format") {
                let v = to_form_string(v)?;
                form = form.text("response_format", v);
            };
            if let Some(v) = body.get("size") {
                let v = to_form_string(v)?;
                form = form.text("size", v);
            };
            if let Some(v) = body.get("user") {
                let v = to_form_string(v)?;
                form = form.text("user", v);
            };
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .multipart(form)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::create_image_variation`]
    ///
    ///[`Client::create_image_variation`]: super::Client::create_image_variation
    #[derive(Debug)]
    pub struct CreateImageVariation<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateImageVariationBody, String>,
        image: Result<Part, String>,
    }

    impl<'a> CreateImageVariation<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::CreateImageVariationBody::default()),
                image: Err("image was not initialized".to_string()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateImageVariationBody>,
            <V as std::convert::TryInto<types::CreateImageVariationBody>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `CreateImageVariationBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateImageVariationBody,
            ) -> types::builder::CreateImageVariationBody,
        {
            self.body = self.body.map(f);
            self
        }

        pub fn image(mut self, value: Part) -> Self {
            self.image = Ok(value);
            self
        }

        ///Sends a `POST` request to `/images/variations`
        pub async fn send(self) -> Result<ResponseValue<types::ImagesResponse>, Error<()>> {
            let Self {
                client,
                body,
                image,
            } = self;
            let body = body
                .and_then(|v| {
                    types::CreateImageVariationBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let image = image.map_err(Error::InvalidRequest)?;
            let url = format!("{}/images/variations", client.baseurl,);
            let body = serde_json::to_value(body)
                .map_err(|e| Error::InvalidRequest(e.to_string()))?
                .as_object()
                .cloned()
                .ok_or_else(|| Error::InvalidRequest("Serializing body failed.".to_string()))?;
            let mut form = reqwest::multipart::Form::new().part("image", image);
            if let Some(v) = body.get("n") {
                let v = to_form_string(v)?;
                form = form.text("n", v);
            };
            if let Some(v) = body.get("response_format") {
                let v = to_form_string(v)?;
                form = form.text("response_format", v);
            };
            if let Some(v) = body.get("size") {
                let v = to_form_string(v)?;
                form = form.text("size", v);
            };
            if let Some(v) = body.get("user") {
                let v = to_form_string(v)?;
                form = form.text("user", v);
            };
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .multipart(form)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
