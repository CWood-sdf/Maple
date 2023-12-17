use diagnostics::publish_diagnostics;
use lsp_types::TextDocumentSyncKind;
use std::io::Write;
// use maple_rs::error::MapleError;
mod diagnostics;
mod formatter;
mod variables;
use crate::formatter::get_formatted_document;

use lsp_server::{Connection, Message, Response};
use lsp_types::{DidSaveTextDocumentParams, ServerCapabilities, TextEdit, WorkDoneProgressOptions};
fn main_server(
    connection: &Connection,
    _params: &serde_json::Value,
    log_file: &mut std::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: String;
    let mut formats: Vec<TextEdit> = vec![];
    // let mut current_format_req: Option<RequestId> = None;
    // loop {
    // let message = connection
    //     .receiver
    //     .recv_timeout(std::time::Duration::from_millis(1000));
    // let message = match message {
    //     Ok(message) => message,
    //     Err(e) => {
    //         log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
    //         continue;
    //     }
    // };
    // std::thread::sleep(std::time::Duration::from_millis(1000));
    for message in &connection.receiver {
        match message {
            Message::Request(req) => {
                log_file.write_all(format!("Request: {:?}\n", req).as_bytes())?;
                if req.method == "textDocument/formatting" {
                    let id = req.id.clone();
                    let response = Response::new_ok(id, Some(formats.clone()));
                    // current_format_req = Some(req.id.clone());
                    connection.sender.send(Message::Response(response))?;
                    log_file.write_all(b"Formatting\n")?;
                    // log_file.write_all(
                    //     format!("File contents: {}\n", formats[0].new_text).as_bytes(),
                    // )?;
                    // let params: lsp_types::DocumentFormattingParams =
                    //     serde_json::from_value(req.params.clone()).unwrap();
                    // match format_document(connection, &params.text_document.uri, log_file, req.id) {
                    //     Ok(_) => {}
                    //     Err(e) => {
                    //         log_file.write_all(
                    //             format!("Error while formatting: {:?}\n", e).as_bytes(),
                    //         )?;
                    //     }
                    // }
                } else if req.method == "shutdown" || req.method == "exit" {
                    log_file.write_all(b"Shutting down\n")?;
                    return Ok(());
                }
                // let response = match req.method.as_str() {
                //     "textDocument/hover" => {
                //         let id = req.id.clone();
                //         let response = Response::new_ok(id, "Hello, world!".to_string());
                //         connection.sender.send(Message::Response(response))?;
                //         continue;
                //     }
                //     _ => Response::new_err(
                //         req.id.clone(),
                //         lsp_server::ErrorCode::MethodNotFound as i32,
                //         "Method not found".to_string(),
                //     ),
                // };
                // connection.sender.send(Message::Response(response))?;
            }
            Message::Response(resp) => {
                log_file.write_all(format!("Response: {:?}\n", resp).as_bytes())?;
            }
            Message::Notification(n) => {
                log_file.write_all(format!("Notification: {:?}\n", n).as_bytes())?;
                if n.method == "textDocument/didSave" {
                    let params: DidSaveTextDocumentParams =
                        serde_json::from_value(n.params.clone()).unwrap();
                    let uri = params.text_document.uri;
                    file_contents =
                        std::fs::read_to_string(uri.to_string().replace("file://", ""))?;
                    match publish_diagnostics(file_contents.clone(), connection, &uri, log_file) {
                        Ok(false) => {
                            formats = vec![];
                        }
                        Ok(true) => {
                            formats = match get_formatted_document(file_contents.clone(), log_file)
                            {
                                Ok(formats) => formats,
                                Err(e) => {
                                    log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
                                    vec![]
                                }
                            };
                        }
                        Err(e) => {
                            formats = vec![];
                            log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
                        }
                    }
                } else if n.method == "textDocument/didChange" {
                    let params: lsp_types::DidChangeTextDocumentParams =
                        serde_json::from_value(n.params.clone()).unwrap();
                    file_contents = params.content_changes[0].text.clone();
                    let uri = params.text_document.uri;
                    match publish_diagnostics(file_contents.clone(), connection, &uri, log_file) {
                        Ok(false) => {
                            formats = vec![];
                        }
                        Ok(true) => {
                            formats = match get_formatted_document(file_contents.clone(), log_file)
                            {
                                Ok(formats) => formats,
                                Err(e) => {
                                    log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
                                    vec![]
                                }
                            };
                        }
                        Err(e) => {
                            formats = vec![];
                            log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
                        }
                    }
                    // log_file.write_all(
                    //     format!("Params: {}\n", params.content_changes[0].text).as_bytes(),
                    // )?;
                } else if n.method == "textDocument/didOpen" {
                    let params: lsp_types::DidOpenTextDocumentParams =
                        serde_json::from_value(n.params.clone()).unwrap();
                    let uri = params.text_document.uri;
                    file_contents =
                        std::fs::read_to_string(uri.to_string().replace("file://", ""))?;
                    match publish_diagnostics(file_contents.clone(), connection, &uri, log_file) {
                        Ok(false) => {
                            formats = vec![];
                        }
                        Ok(true) => {
                            formats = match get_formatted_document(file_contents.clone(), log_file)
                            {
                                Ok(formats) => formats,
                                Err(e) => {
                                    log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
                                    vec![]
                                }
                            };
                        }
                        Err(e) => {
                            formats = vec![];
                            log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
                        }
                    }
                }
            }
        };
    }
    // }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut log_file = std::fs::File::create("/home/cwood/projects/maple/lsp/log.txt")?;
    let (connection, _) = Connection::stdio();
    let server_capabilities = ServerCapabilities {
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        diagnostic_provider: Some(lsp_types::DiagnosticServerCapabilities::Options(
            lsp_types::DiagnosticOptions {
                workspace_diagnostics: true,
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(true),
                },
                identifier: Some("maple".to_string()),
                inter_file_dependencies: true,
            },
        )),
        document_formatting_provider: Some(lsp_types::OneOf::Left(true)),
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL,
        )),
        ..Default::default()
    };
    log_file.write_all(b"Started server\n")?;
    let capabilities = serde_json::to_value(&server_capabilities).unwrap();
    log_file.write_all(format!("Capabilities: {:?}\n", capabilities).as_bytes())?;
    let init_params = connection.initialize(capabilities)?;
    // log_file.write_all(format!("Init params: {:?}\n", init_params).as_bytes())?;
    std::fs::write(
        "/home/cwood/projects/maple/lsp/init_params.json",
        init_params.to_string(),
    )?;

    main_server(&connection, &init_params, &mut log_file)?;

    log_file.write_all(b"Shutting down\n")?;
    // smh io_threads.join() blocks the task from ending
    // io_threads.join()?;
    // log_file.write_all(b"Shutting down\n")?;
    Ok(())
}
