use crate::openai_openapi_builder::*;
pub struct Cli<T: CliConfig> {
    client: Client,
    config: T,
}

impl<T: CliConfig> Cli<T> {
    pub fn new(client: Client, config: T) -> Self {
        Self { client, config }
    }

    pub fn get_command(cmd: CliCommand) -> clap::Command {
        match cmd {
            CliCommand::CreateImage => Self::cli_create_image(),
            CliCommand::CreateImageEdit => Self::cli_create_image_edit(),
            CliCommand::CreateImageVariation => Self::cli_create_image_variation(),
        }
    }

    pub fn cli_create_image() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("n")
                    .long("n")
                    .value_parser(clap::value_parser!(i64))
                    .required(false)
                    .help("The number of images to generate. Must be between 1 and 10."),
            )
            .arg(
                clap::Arg::new("prompt")
                    .long("prompt")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help(
                        "A text description of the desired image(s). The maximum length is 1000 \
                         characters.",
                    ),
            )
            .arg(
                clap::Arg::new("response-format")
                    .long("response-format")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::CreateImageRequestResponseFormat::Url.to_string(),
                            types::CreateImageRequestResponseFormat::B64Json.to_string(),
                        ]),
                        |s| types::CreateImageRequestResponseFormat::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help(
                        "The format in which the generated images are returned. Must be one of \
                         `url` or `b64_json`.",
                    ),
            )
            .arg(
                clap::Arg::new("size")
                    .long("size")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::CreateImageRequestSize::_256x256.to_string(),
                            types::CreateImageRequestSize::_512x512.to_string(),
                            types::CreateImageRequestSize::_1024x1024.to_string(),
                        ]),
                        |s| types::CreateImageRequestSize::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help(
                        "The size of the generated images. Must be one of `256x256`, `512x512`, \
                         or `1024x1024`.",
                    ),
            )
            .arg(
                clap::Arg::new("user")
                    .long("user")
                    .value_parser(clap::value_parser!(String))
                    .required(false)
                    .help(
                        "A unique identifier representing your end-user, which can help OpenAI to \
                         monitor and detect abuse. [Learn \
                         more](/docs/guides/safety-best-practices/end-user-ids).\n",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Creates an image given a prompt.")
    }

    pub fn cli_create_image_edit() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help(
                        "The image to edit. Must be a valid PNG file, less than 4MB, and square. \
                         If mask is not provided, image must have transparency, which will be \
                         used as the mask.",
                    ),
            )
            .arg(
                clap::Arg::new("mask")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help(
                        "An additional image whose fully transparent areas (e.g. where alpha is \
                         zero) indicate where `image` should be edited. Must be a valid PNG file, \
                         less than 4MB, and have the same dimensions as `image`.",
                    ),
            )
            .arg(
                clap::Arg::new("n")
                    .long("n")
                    .value_parser(clap::value_parser!(i64))
                    .required(false)
                    .help("The number of images to generate. Must be between 1 and 10."),
            )
            .arg(
                clap::Arg::new("prompt")
                    .long("prompt")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help(
                        "A text description of the desired image(s). The maximum length is 1000 \
                         characters.",
                    ),
            )
            .arg(
                clap::Arg::new("response-format")
                    .long("response-format")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::CreateImageEditBodyResponseFormat::Url.to_string(),
                            types::CreateImageEditBodyResponseFormat::B64Json.to_string(),
                        ]),
                        |s| types::CreateImageEditBodyResponseFormat::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help(
                        "The format in which the generated images are returned. Must be one of \
                         `url` or `b64_json`.",
                    ),
            )
            .arg(
                clap::Arg::new("size")
                    .long("size")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::CreateImageEditBodySize::_256x256.to_string(),
                            types::CreateImageEditBodySize::_512x512.to_string(),
                            types::CreateImageEditBodySize::_1024x1024.to_string(),
                        ]),
                        |s| types::CreateImageEditBodySize::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help(
                        "The size of the generated images. Must be one of `256x256`, `512x512`, \
                         or `1024x1024`.",
                    ),
            )
            .arg(
                clap::Arg::new("user")
                    .long("user")
                    .value_parser(clap::value_parser!(String))
                    .required(false)
                    .help(
                        "A unique identifier representing your end-user, which can help OpenAI to \
                         monitor and detect abuse. [Learn \
                         more](/docs/guides/safety-best-practices/end-user-ids).\n",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Creates an edited or extended image given an original image and a prompt.")
    }

    pub fn cli_create_image_variation() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help(
                        "The image to use as the basis for the variation(s). Must be a valid PNG \
                         file, less than 4MB, and square.",
                    ),
            )
            .arg(
                clap::Arg::new("n")
                    .long("n")
                    .value_parser(clap::value_parser!(i64))
                    .required(false)
                    .help("The number of images to generate. Must be between 1 and 10."),
            )
            .arg(
                clap::Arg::new("response-format")
                    .long("response-format")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::CreateImageVariationBodyResponseFormat::Url.to_string(),
                            types::CreateImageVariationBodyResponseFormat::B64Json.to_string(),
                        ]),
                        |s| types::CreateImageVariationBodyResponseFormat::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help(
                        "The format in which the generated images are returned. Must be one of \
                         `url` or `b64_json`.",
                    ),
            )
            .arg(
                clap::Arg::new("size")
                    .long("size")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::CreateImageVariationBodySize::_256x256.to_string(),
                            types::CreateImageVariationBodySize::_512x512.to_string(),
                            types::CreateImageVariationBodySize::_1024x1024.to_string(),
                        ]),
                        |s| types::CreateImageVariationBodySize::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help(
                        "The size of the generated images. Must be one of `256x256`, `512x512`, \
                         or `1024x1024`.",
                    ),
            )
            .arg(
                clap::Arg::new("user")
                    .long("user")
                    .value_parser(clap::value_parser!(String))
                    .required(false)
                    .help(
                        "A unique identifier representing your end-user, which can help OpenAI to \
                         monitor and detect abuse. [Learn \
                         more](/docs/guides/safety-best-practices/end-user-ids).\n",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Creates a variation of a given image.")
    }

    pub async fn execute(&self, cmd: CliCommand, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        match cmd {
            CliCommand::CreateImage => self.execute_create_image(matches).await,
            CliCommand::CreateImageEdit => self.execute_create_image_edit(matches).await,
            CliCommand::CreateImageVariation => self.execute_create_image_variation(matches).await,
        }
    }

    pub async fn execute_create_image(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.create_image();
        if let Some(value) = matches.get_one::<i64>("n") {
            request = request.body_map(|body| body.n(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("prompt") {
            request = request.body_map(|body| body.prompt(value.clone()))
        }

        if let Some(value) =
            matches.get_one::<types::CreateImageRequestResponseFormat>("response-format")
        {
            request = request.body_map(|body| body.response_format(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::CreateImageRequestSize>("size") {
            request = request.body_map(|body| body.size(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("user") {
            request = request.body_map(|body| body.user(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::CreateImageRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_create_image(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_image_edit(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_image_edit();
        if let Some(path) = matches.get_one::<std::path::PathBuf>("image") {
            let part = self.over.request_file_part(path).unwrap();
            request = request.image(part);
        }

        if let Some(path) = matches.get_one::<std::path::PathBuf>("mask") {
            let part = self.over.request_file_part(path).unwrap();
            request = request.mask(Some(part));
        }

        if let Some(value) = matches.get_one::<i64>("n") {
            request = request.body_map(|body| body.n(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("prompt") {
            request = request.body_map(|body| body.prompt(value.clone()))
        }

        if let Some(value) =
            matches.get_one::<types::CreateImageEditBodyResponseFormat>("response-format")
        {
            request = request.body_map(|body| body.response_format(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::CreateImageEditBodySize>("size") {
            request = request.body_map(|body| body.size(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("user") {
            request = request.body_map(|body| body.user(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::CreateImageEditBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_create_image_edit(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_create_image_variation(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.create_image_variation();
        if let Some(path) = matches.get_one::<std::path::PathBuf>("image") {
            let part = self.over.request_file_part(path).unwrap();
            request = request.image(part);
        }

        if let Some(value) = matches.get_one::<i64>("n") {
            request = request.body_map(|body| body.n(value.clone()))
        }

        if let Some(value) =
            matches.get_one::<types::CreateImageVariationBodyResponseFormat>("response-format")
        {
            request = request.body_map(|body| body.response_format(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::CreateImageVariationBodySize>("size") {
            request = request.body_map(|body| body.size(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("user") {
            request = request.body_map(|body| body.user(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::CreateImageVariationBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_create_image_variation(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.item_success(&r);
                Ok(())
            }
            Err(r) => {
                self.config.item_error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
}

pub trait CliConfig {
    fn item_success<T>(&self, value: &ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn item_error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn execute_create_image(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateImage,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_image_edit(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateImageEdit,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_create_image_variation(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CreateImageVariation,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn request_file_part(
        &self,
        path: &std::path::PathBuf,
    ) -> Result<reqwest::multipart::Part, String> {
        use std::io::Read;
        let mut file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
        let mut value = Vec::new();
        file.read_to_end(&mut value).map_err(|e| e.to_string())?;
        let part = reqwest::multipart::Part::bytes(value);
        Ok(if let Some(file_name) = path.file_name() {
            part.file_name(file_name.to_string_lossy().into_owned())
        } else {
            part
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    CreateImage,
    CreateImageEdit,
    CreateImageVariation,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::CreateImage,
            CliCommand::CreateImageEdit,
            CliCommand::CreateImageVariation,
        ]
        .into_iter()
    }
}
