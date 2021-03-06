use drone_config::Config;
use drone_macros_core::compile_error;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, LitInt,
};

struct Input {}

impl Parse for Input {
    fn parse(_input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {})
    }
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
    let Input {} = parse_macro_input!(input as Input);
    let config = match Config::read_from_cargo_manifest_dir() {
        Ok(config) => config,
        Err(err) => compile_error!("{}: {}", drone_config::CONFIG_NAME, err),
    };
    let baud_rate = if let Some(log) = config.log {
        if let Some(swo) = log.swo {
            swo.baud_rate
        } else if let Some(dso) = log.dso {
            dso.baud_rate
        } else {
            compile_error!(
                "Missing one of `log.swo`, `log.dso` sections in `{}`",
                drone_config::CONFIG_NAME
            );
        }
    } else {
        compile_error!("Missing `log` section in `{}`", drone_config::CONFIG_NAME);
    };
    let baud_rate = LitInt::new(&baud_rate.to_string(), Span::call_site());
    quote!(#baud_rate).into()
}
