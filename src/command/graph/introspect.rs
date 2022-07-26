use crate::Result;
use reqwest::blocking::Client;
use saucer::{clap, Parser};
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

use rover_client::{
    blocking::GraphQLClient,
    operations::graph::introspect::{self, GraphIntrospectInput},
};

use crate::command::RoverOutput;
use crate::utils::parsers::parse_header;

#[derive(Debug, Serialize, Parser)]
pub struct Introspect {
    /// The endpoint of the graph to introspect
    #[serde(skip_serializing)]
    pub endpoint: Url,

    /// headers to pass to the endpoint. Values must be key:value pairs.
    /// If a value has a space in it, use quotes around the pair,
    /// ex. -H "Auth:some key"

    // The `name` here is for the help text and error messages, to print like
    // --header <key:value> rather than the plural field name --header <headers>
    #[clap(name="key:value", multiple=true, long="header", short='H', parse(try_from_str = parse_header))]
    #[serde(skip_serializing)]
    pub headers: Option<Vec<(String, String)>>,
}

impl Introspect {
    pub fn run(&self, client: Client) -> Result<RoverOutput> {
        let client = GraphQLClient::new(&self.endpoint.to_string(), client)?;

        // add the flag headers to a hashmap to pass along to rover-client
        let mut headers = HashMap::new();
        if let Some(arg_headers) = &self.headers {
            for (header_key, header_value) in arg_headers {
                headers.insert(header_key.to_string(), header_value.to_string());
            }
        };

        let introspection_response = introspect::run(GraphIntrospectInput { headers }, &client)?;

        Ok(RoverOutput::Introspection(
            introspection_response.schema_sdl,
        ))
    }
}
