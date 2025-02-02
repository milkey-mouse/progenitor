pub mod operations {
    #![doc = r" [`When`](httpmock::When) and [`Then`](httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::openai_openapi_builder::*;
    pub struct CreateImageWhen(httpmock::When);
    impl CreateImageWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/images/generations$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::CreateImageRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct CreateImageThen(httpmock::Then);
    impl CreateImageThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ImagesResponse) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CreateImageEditWhen(httpmock::When);
    impl CreateImageEditWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/images/edits$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::CreateImageEditBody) -> Self {
            Self(self.0.json_body_obj(value))
        }

        pub fn mask(self, value: Option<()>) -> Self {
            if let Some(()) = value {
                Self(self.0.body_contains("form-data; name=\"mask\""))
            } else {
                self
            }
        }

        pub fn image(self, value: ()) -> Self {
            Self(self.0.body_contains("form-data; name=\"image\""))
        }
    }

    pub struct CreateImageEditThen(httpmock::Then);
    impl CreateImageEditThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ImagesResponse) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CreateImageVariationWhen(httpmock::When);
    impl CreateImageVariationWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/images/variations$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::CreateImageVariationBody) -> Self {
            Self(self.0.json_body_obj(value))
        }

        pub fn image(self, value: ()) -> Self {
            Self(self.0.body_contains("form-data; name=\"image\""))
        }
    }

    pub struct CreateImageVariationThen(httpmock::Then);
    impl CreateImageVariationThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ImagesResponse) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
}

#[doc = r" An extension trait for [`MockServer`](httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn create_image<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CreateImageWhen, operations::CreateImageThen);
    fn create_image_edit<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CreateImageEditWhen, operations::CreateImageEditThen);
    fn create_image_variation<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CreateImageVariationWhen, operations::CreateImageVariationThen);
}

impl MockServerExt for httpmock::MockServer {
    fn create_image<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CreateImageWhen, operations::CreateImageThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CreateImageWhen::new(when),
                operations::CreateImageThen::new(then),
            )
        })
    }

    fn create_image_edit<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CreateImageEditWhen, operations::CreateImageEditThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CreateImageEditWhen::new(when),
                operations::CreateImageEditThen::new(then),
            )
        })
    }

    fn create_image_variation<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CreateImageVariationWhen, operations::CreateImageVariationThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CreateImageVariationWhen::new(when),
                operations::CreateImageVariationThen::new(then),
            )
        })
    }
}
