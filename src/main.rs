use std::error::Error;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::fs::File;

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types::request::HoverRequest;
use lsp_types::*;

mod lsp_util;
mod x86_csv;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("/tmp/xor_lsp.log").unwrap(),
    )
    .unwrap();

    let csv_file = File::open("misc/x86spec.csv").unwrap();
    let map = x86_csv::parse(csv_file).unwrap();

    info!("starting xor-lsp LSP server");

    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        // definition_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params, map)?;
    io_threads.join()?;

    info!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
    map: std::collections::HashMap<String, Vec<x86_csv::X86Data>>,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    info!("starting main loop");
    for msg in &connection.receiver {
        info!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                info!("got request: {req:?}");
                /*
                match cast::<GotoDefinition>(req.clone()) {
                    Ok((id, params)) => {
                        info!("got gotoDefinition request #{id}: {params:?}");
                        let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response { id, result: Some(result), error: None };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                */
                match cast::<HoverRequest>(req.clone()) {
                    Ok((id, params)) => {
                        info!("got hover request #{id}: {params:?}");
                        let word = lsp_util::get_word_from_file_params(
                            &params.text_document_position_params,
                        )
                        .unwrap();
                        if !map.contains_key(&word) {
                            // TODO: empty or no response?
                            continue;
                        }
                        let data = map.get(&word).unwrap();
                        let mut value = String::new();
                        for d in data {
                            value = format!("{}# {}\n{}\n", value, d.syntax, d.desc);
                        }

                        let result = Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value,
                            }),
                            range: None,
                        });
                        let result = serde_json::to_value(&result).unwrap();
                        let result = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(result.clone()))?;
                        continue;
                    }
                    Err(_) => {
                        info!("HoverRequest Err");
                    }
                }
                // ...
            }
            Message::Response(resp) => {
                info!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                info!("got notification: {not:?}");
            }
        }
    }
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
