use std::io::Write;

use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, WorkDoneProgressOptions,
};
fn main_server(
    connection: &Connection,
    params: &serde_json::Value,
    log_file: &mut std::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
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
                if req.method == "textDocument/didOpen" {
                    let params = req.params;
                    let params = serde_json::to_value(params).unwrap();
                    let params = serde_json::to_string(&params).unwrap();
                    std::fs::write(
                        "/home/cwood/projects/maple/lsp/did_open_params.json",
                        params,
                    )?;
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
                    log_file.write_all(b"Saved\n")?;
                    connection.sender.send(Message::Notification(Notification {
                        params: serde_json::to_value(lsp_types::PublishDiagnosticsParams {
                            uri: lsp_types::Url::parse(
                                "file:///home/cwood/projects/maple/rust/maple.mpl",
                            )?,
                            version: None,
                            diagnostics: vec![lsp_types::Diagnostic {
                                range: lsp_types::Range {
                                    start: lsp_types::Position {
                                        line: 1,
                                        character: 1,
                                    },
                                    end: lsp_types::Position {
                                        line: 1,
                                        character: 3,
                                    },
                                },
                                severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                                code: None,
                                code_description: None,
                                source: None,
                                message: "Hello, world!".to_string(),
                                related_information: None,
                                tags: None,
                                data: None,
                            }],
                        })
                        .unwrap(),
                        method: "textDocument/publishDiagnostics".to_string(),
                    }))?;
                }
            }
        };
    }
    // }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut log_file = std::fs::File::create("/home/cwood/projects/maple/lsp/log.txt")?;
    let (connection, io_threads) = Connection::stdio();
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

    io_threads.join()?;
    Ok(())
}
