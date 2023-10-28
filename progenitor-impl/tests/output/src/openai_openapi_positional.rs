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
    #[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

    ///CreateImageEditRequest
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
    ///    "image": {
    ///      "description": "The image to edit. Must be a valid PNG file, less
    /// than 4MB, and square. If mask is not provided, image must have
    /// transparency, which will be used as the mask.",
    ///      "type": "string",
    ///      "format": "binary"
    ///    },
    ///    "mask": {
    ///      "description": "An additional image whose fully transparent areas
    /// (e.g. where alpha is zero) indicate where `image` should be edited. Must
    /// be a valid PNG file, less than 4MB, and have the same dimensions as
    /// `image`.",
    ///      "type": "string",
    ///      "format": "binary"
    ///    },
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateImageEditRequest {
        ///The image to edit. Must be a valid PNG file, less than 4MB, and
        /// square. If mask is not provided, image must have transparency, which
        /// will be used as the mask.
        pub image: String,
        ///An additional image whose fully transparent areas (e.g. where alpha
        /// is zero) indicate where `image` should be edited. Must be a valid
        /// PNG file, less than 4MB, and have the same dimensions as `image`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mask: Option<String>,
        ///The number of images to generate. Must be between 1 and 10.
        #[serde(default = "defaults::create_image_edit_request_n")]
        pub n: Option<i64>,
        ///A text description of the desired image(s). The maximum length is
        /// 1000 characters.
        pub prompt: String,
        ///The format in which the generated images are returned. Must be one
        /// of `url` or `b64_json`.
        #[serde(default = "defaults::create_image_edit_request_response_format")]
        pub response_format: Option<CreateImageEditRequestResponseFormat>,
        ///The size of the generated images. Must be one of `256x256`,
        /// `512x512`, or `1024x1024`.
        #[serde(default = "defaults::create_image_edit_request_size")]
        pub size: Option<CreateImageEditRequestSize>,
        ///A unique identifier representing your end-user, which can help
        /// OpenAI to monitor and detect abuse. [Learn
        /// more](/docs/guides/safety-best-practices/end-user-ids).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
    }

    impl From<&CreateImageEditRequest> for CreateImageEditRequest {
        fn from(value: &CreateImageEditRequest) -> Self {
            value.clone()
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum CreateImageEditRequestResponseFormat {
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "b64_json")]
        B64Json,
    }

    impl From<&CreateImageEditRequestResponseFormat> for CreateImageEditRequestResponseFormat {
        fn from(value: &CreateImageEditRequestResponseFormat) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageEditRequestResponseFormat {
        fn to_string(&self) -> String {
            match *self {
                Self::Url => "url".to_string(),
                Self::B64Json => "b64_json".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageEditRequestResponseFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "url" => Ok(Self::Url),
                "b64_json" => Ok(Self::B64Json),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CreateImageEditRequestResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageEditRequestResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageEditRequestResponseFormat {
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum CreateImageEditRequestSize {
        #[serde(rename = "256x256")]
        _256x256,
        #[serde(rename = "512x512")]
        _512x512,
        #[serde(rename = "1024x1024")]
        _1024x1024,
    }

    impl From<&CreateImageEditRequestSize> for CreateImageEditRequestSize {
        fn from(value: &CreateImageEditRequestSize) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageEditRequestSize {
        fn to_string(&self) -> String {
            match *self {
                Self::_256x256 => "256x256".to_string(),
                Self::_512x512 => "512x512".to_string(),
                Self::_1024x1024 => "1024x1024".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageEditRequestSize {
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

    impl std::convert::TryFrom<&str> for CreateImageEditRequestSize {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageEditRequestSize {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageEditRequestSize {
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

    ///CreateImageVariationRequest
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
    ///    "image": {
    ///      "description": "The image to use as the basis for the variation(s).
    /// Must be a valid PNG file, less than 4MB, and square.",
    ///      "type": "string",
    ///      "format": "binary"
    ///    },
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateImageVariationRequest {
        ///The image to use as the basis for the variation(s). Must be a valid
        /// PNG file, less than 4MB, and square.
        pub image: String,
        ///The number of images to generate. Must be between 1 and 10.
        #[serde(default = "defaults::create_image_variation_request_n")]
        pub n: Option<i64>,
        ///The format in which the generated images are returned. Must be one
        /// of `url` or `b64_json`.
        #[serde(default = "defaults::create_image_variation_request_response_format")]
        pub response_format: Option<CreateImageVariationRequestResponseFormat>,
        ///The size of the generated images. Must be one of `256x256`,
        /// `512x512`, or `1024x1024`.
        #[serde(default = "defaults::create_image_variation_request_size")]
        pub size: Option<CreateImageVariationRequestSize>,
        ///A unique identifier representing your end-user, which can help
        /// OpenAI to monitor and detect abuse. [Learn
        /// more](/docs/guides/safety-best-practices/end-user-ids).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
    }

    impl From<&CreateImageVariationRequest> for CreateImageVariationRequest {
        fn from(value: &CreateImageVariationRequest) -> Self {
            value.clone()
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum CreateImageVariationRequestResponseFormat {
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "b64_json")]
        B64Json,
    }

    impl From<&CreateImageVariationRequestResponseFormat>
        for CreateImageVariationRequestResponseFormat
    {
        fn from(value: &CreateImageVariationRequestResponseFormat) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageVariationRequestResponseFormat {
        fn to_string(&self) -> String {
            match *self {
                Self::Url => "url".to_string(),
                Self::B64Json => "b64_json".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageVariationRequestResponseFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "url" => Ok(Self::Url),
                "b64_json" => Ok(Self::B64Json),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CreateImageVariationRequestResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageVariationRequestResponseFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageVariationRequestResponseFormat {
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum CreateImageVariationRequestSize {
        #[serde(rename = "256x256")]
        _256x256,
        #[serde(rename = "512x512")]
        _512x512,
        #[serde(rename = "1024x1024")]
        _1024x1024,
    }

    impl From<&CreateImageVariationRequestSize> for CreateImageVariationRequestSize {
        fn from(value: &CreateImageVariationRequestSize) -> Self {
            value.clone()
        }
    }

    impl ToString for CreateImageVariationRequestSize {
        fn to_string(&self) -> String {
            match *self {
                Self::_256x256 => "256x256".to_string(),
                Self::_512x512 => "512x512".to_string(),
                Self::_1024x1024 => "1024x1024".to_string(),
            }
        }
    }

    impl std::str::FromStr for CreateImageVariationRequestSize {
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

    impl std::convert::TryFrom<&str> for CreateImageVariationRequestSize {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CreateImageVariationRequestSize {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CreateImageVariationRequestSize {
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ErrorResponse {
        pub error: Error,
    }

    impl From<&ErrorResponse> for ErrorResponse {
        fn from(value: &ErrorResponse) -> Self {
            value.clone()
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ImagesResponse {
        pub created: i64,
        pub data: Vec<Image>,
    }

    impl From<&ImagesResponse> for ImagesResponse {
        fn from(value: &ImagesResponse) -> Self {
            value.clone()
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

        pub(super) fn create_image_edit_request_n() -> Option<i64> {
            Some(1_i64)
        }

        pub(super) fn create_image_edit_request_response_format(
        ) -> Option<super::CreateImageEditRequestResponseFormat> {
            Some(super::CreateImageEditRequestResponseFormat::Url)
        }

        pub(super) fn create_image_edit_request_size() -> Option<super::CreateImageEditRequestSize>
        {
            Some(super::CreateImageEditRequestSize::_1024x1024)
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

        pub(super) fn create_image_variation_request_n() -> Option<i64> {
            Some(1_i64)
        }

        pub(super) fn create_image_variation_request_response_format(
        ) -> Option<super::CreateImageVariationRequestResponseFormat> {
            Some(super::CreateImageVariationRequestResponseFormat::Url)
        }

        pub(super) fn create_image_variation_request_size(
        ) -> Option<super::CreateImageVariationRequestSize> {
            Some(super::CreateImageVariationRequestSize::_1024x1024)
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

#[allow(clippy::all)]
impl Client {
    ///Creates an image given a prompt
    ///
    ///Sends a `POST` request to `/images/generations`
    pub async fn create_image<'a>(
        &'a self,
        body: &'a types::CreateImageRequest,
    ) -> Result<ResponseValue<types::ImagesResponse>, Error<()>> {
        let url = format!("{}/images/generations", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
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
    pub async fn create_image_edit<'a>(
        &'a self,
        body: &'a types::CreateImageEditBody,
        mask: Option<Part>,
        image: Part,
    ) -> Result<ResponseValue<types::ImagesResponse>, Error<()>> {
        let url = format!("{}/images/edits", self.baseurl,);
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
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .multipart(form)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Creates a variation of a given image
    ///
    ///Sends a `POST` request to `/images/variations`
    ///
    ///Arguments:
    /// - `body`
    /// - `image`: The image to use as the basis for the variation(s). Must be a
    ///   valid PNG file, less than 4MB, and square.
    pub async fn create_image_variation<'a>(
        &'a self,
        body: &'a types::CreateImageVariationBody,
        image: Part,
    ) -> Result<ResponseValue<types::ImagesResponse>, Error<()>> {
        let url = format!("{}/images/variations", self.baseurl,);
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
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .multipart(form)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
