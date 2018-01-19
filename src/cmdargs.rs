extern crate clap;

use chrono::Duration;
use mongo_driver;
use std::num;

type MongoUri = mongo_driver::client::Uri;

quick_error! {
    /// Commandline parsing errors.
    #[derive(Debug)]
    pub enum Error {
        /// A required argument has not been provided.
        NoArgument(arg: String) {
            description("Argument not found")
            display("No argument '{}' provided", arg)
        }
        /// URI parsing failure.
        ParseUri(uri: String) {
            description("Can't parse URI")
            display("Can't parse URI {}", uri)
        }
        /// Can't parse a value of an argument.
        ParseInt(err: num::ParseIntError) {
            cause(err)
            from()
        }
    }
}

/// Database options.
#[derive(Debug, Clone)]
pub struct DbOptions {
    /// Database URI.
    pub uri: MongoUri,
    /// Database name.
    pub db_name: String,
    /// Collection name in the database.
    pub collection_name: String,
}

#[derive(Debug)]
/// Command line options.
pub struct Options {
    /// Database options.
    pub db_options: DbOptions,
    /// Web server address (in the form of `ip:port`).
    pub web_addr: String,
    /// Verbosity level.
    pub verbose: usize,
    /// Handlebars templates path.
    pub templates_path: String,
    /// Handlebars templates extension.
    pub templates_ext: String,
    /// Web server public URL prefix that will be appended to download-links generated by the
    /// server.
    pub url_prefix: String,
    /// Default expiration time for pastes.
    pub default_ttl: Duration,
    /// Path to the static files.
    pub static_files_path: String,
}

/// A helper to simplify a creation of a "no argument" error.
fn no_arg(arg: &str) -> Error {
    Error::NoArgument(arg.into())
}

fn parse_uri(arg: &str) -> Result<MongoUri, Error> {
    match MongoUri::new(arg.to_string()) {
        Some(uri) => Ok(uri),
        None => Err(Error::ParseUri(arg.to_string())),
    }
}

/// Parses command line arguments.
pub fn parse() -> Result<Options, Error> {
    let args = build_cli().get_matches();
    let uri = parse_uri(args.value_of("DB_URI").ok_or(no_arg("DB_URI"))?)?;
    let db_name = args.value_of("DB_NAME").ok_or(no_arg("DB_NAME"))?
                      .to_string();
    let collection_name = args.value_of("COLLECTION_NAME").ok_or(no_arg("COLLECTION_NAME"))?
                              .to_string();
    let verbose = args.occurrences_of("VERBOSE") as usize;
    let web_addr = args.value_of("WEB_ADDR").ok_or(no_arg("WEB_ADDR"))?
                       .to_string();
    let templates_path = args.value_of("TEMPLATES_PATH").ok_or(no_arg("TEMPLATES_PATH"))?
                             .to_string();
    let templates_ext = args.value_of("TEMPLATES_EXT").ok_or(no_arg("TEMPLATES_EXT"))?
                            .to_string();
    let url_prefix = args.value_of("URL_PREFIX").ok_or(no_arg("URL_PREFIX"))?
                         .to_string();
    let default_ttl = args.value_of("DEFAULT_TTL").ok_or(no_arg("DEFAULT_TTL"))?
                          .parse()?;
    let static_files_path = args.value_of("STATIC_PATH").ok_or(no_arg("STATIC_PATH"))?
                                .to_string();

    Ok(Options { db_options: DbOptions { uri,
                                         db_name,
                                         collection_name, },
                 web_addr,
                 verbose,
                 templates_path,
                 templates_ext,
                 url_prefix,
                 default_ttl: Duration::days(default_ttl),
                 static_files_path, })
}

/// Builds command line arguments.
fn build_cli() -> clap::App<'static, 'static> {
    use self::clap::{App, Arg};
    App::new("Pastebin web server")
        .about("Launches a pastebin web server.")
        .arg(Arg::with_name("DB_URI").long("db-uri")
                                      .value_name("URI")
                                      .takes_value(true)
                                      .required(true)
                                      .help("Database URI (mongodb://...)"))
        .arg(Arg::with_name("DB_NAME").long("db-name")
                                      .value_name("name")
                                      .takes_value(true)
                                      .required(true)
                                      .help("Name of the database"))
        .arg(Arg::with_name("COLLECTION_NAME").long("collection")
                                              .value_name("name")
                                              .takes_value(true)
                                              .required(true)
                                              .help("Collection name"))
        .arg(Arg::with_name("VERBOSE").long("verbose")
                                      .short("v")
                                      .takes_value(false)
                                      .required(false)
                                      .multiple(true)
                                      .help("Verbosity level"))
        .arg(Arg::with_name("WEB_ADDR").long("web-addr")
                                      .value_name("address")
                                      .takes_value(true)
                                      .required(true)
                                      .default_value("localhost:8000")
                                      .help("Web server address"))
        .arg(Arg::with_name("TEMPLATES_PATH").long("templates")
                                              .value_name("path")
                                              .takes_value(true)
                                              .required(true)
                                              .help("Path to the templates folder"))
        .arg(Arg::with_name("TEMPLATES_EXT").long("templates-ext")
                                              .value_name("extension")
                                              .takes_value(true)
                                              .default_value(".tera")
                                              .help("Templates extension"))
        .arg(Arg::with_name("URL_PREFIX").long("url-prefix")
                                         .value_name("url")
                                         .takes_value(true)
                                         .required(true)
                                         .help("Download links prefix"))
        .arg(Arg::with_name("DEFAULT_TTL").long("default-ttl")
                                         .value_name("seconds")
                                         .takes_value(true)
                                         .default_value("7")
                                         .help("Default pastes expiration time in days"))
        .arg(Arg::with_name("STATIC_PATH").long("static-path")
                                         .value_name("path")
                                         .takes_value(true)
                                         .required(true)
                                         .help("Path to the static files"))
}
