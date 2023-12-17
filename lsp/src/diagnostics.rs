use std::io::Write;

use lsp_server::{Connection, Message, Notification};
use lsp_types::Url;
use maple_rs::parser::Parser;

fn clear_diagnostics(connection: &Connection, uri: &Url) -> Result<(), Box<dyn std::error::Error>> {
    connection.sender.send(Message::Notification(Notification {
        params: serde_json::to_value(lsp_types::PublishDiagnosticsParams {
            uri: uri.clone(),
            version: None,
            diagnostics: vec![],
        })?,
        method: "textDocument/publishDiagnostics".to_string(),
    }))?;
    Ok(())
}
pub fn publish_diagnostics(
    contents: String,
    connection: &Connection,
    uri: &Url,
    log_file: &mut std::fs::File,
) -> Result<bool, Box<dyn std::error::Error>> {
    // log_file.write_all(format!("Publishing diagnostics for {}\n", uri).as_bytes())?;
    let mut parser = Parser::new(contents);
    let ast = parser.parse(true);
    match ast {
        Ok(_) => {
            clear_diagnostics(connection, &uri)?;
            Ok(true)
        }
        Err(e) => {
            log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
            clear_diagnostics(connection, &uri)?;
            connection.sender.send(Message::Notification(Notification {
                params: serde_json::to_value(lsp_types::PublishDiagnosticsParams {
                    uri: uri.clone(),
                    version: None,
                    diagnostics: vec![lsp_types::Diagnostic {
                        range: lsp_types::Range {
                            start: lsp_types::Position {
                                line: e.get_token_from_error().line as u32,
                                character: e.get_token_from_error().char_start as u32,
                            },
                            end: lsp_types::Position {
                                line: e.get_token_from_error().line as u32,
                                character: e.get_token_from_error().char_end as u32,
                            },
                        },
                        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: None,
                        message: e.get_msg(),
                        related_information: None,
                        tags: None,
                        data: None,
                    }],
                })?,
                method: "textDocument/publishDiagnostics".to_string(),
            }))?;
            Ok(false)
        }
    }
}
