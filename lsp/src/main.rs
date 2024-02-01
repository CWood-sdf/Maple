use diagnostics::publish_diagnostics;
use lsp_types::{CompletionItem, Location, TextDocumentSyncKind};
use maple_rs::lexer::TokenType;
use std::io::Write;
use variables::Variables;
// use maple_rs::error::MapleError;
mod diagnostics;
mod formatter;
mod variables;
use crate::formatter::get_formatted_document;

use lsp_server::{Connection, Message, Response};
use lsp_types::{DidSaveTextDocumentParams, ServerCapabilities, TextEdit, WorkDoneProgressOptions};
fn handle_request(
    file_contents: &String,
    req: &lsp_server::Request,
    log_file: &mut std::fs::File,
    connection: &Connection,
    formats: &Vec<TextEdit>,
    ref_errs: &Vec<crate::variables::ReferenceError>,
    variables: &Variables,
) -> Result<(), Box<dyn std::error::Error>> {
    log_file.write_all(format!("Request: {:?}\n", req).as_bytes())?;
    if req.method == "textDocument/formatting" {
        let id = req.id.clone();
        let response = Response::new_ok(id, Some(formats.clone()));
        connection.sender.send(Message::Response(response))?;
        log_file.write_all(b"Formatting\n")?;
    } else if req.method == "shutdown" || req.method == "exit" {
        log_file.write_all(b"Shutting down\n")?;
        return Ok(());
    } else if req.method == "textDocument/publishDiagnostics" {
        let parsed_params: lsp_types::PublishDiagnosticsParams =
            serde_json::from_value(req.params.clone()).unwrap();
        let uri = parsed_params.uri;
        match publish_diagnostics(file_contents.clone(), connection, &uri, log_file, &ref_errs) {
            _ => {}
        }
    } else if req.method == "textDocument/definition" {
        let parsed_params: lsp_types::TextDocumentPositionParams =
            serde_json::from_value(req.params.clone()).unwrap();
        let line = parsed_params.position.line as usize;
        let line_str = file_contents.lines().nth(line).unwrap();
        let mut lexer = maple_rs::lexer::Lexer::new(line_str.to_string());
        let char = parsed_params.position.character as usize;
        while lexer.get_current_token().char_end <= char {
            let _ = lexer.get_next_token();
        }
        let var_name = match lexer.get_current_token().t {
            TokenType::Ident(s) => Some(s),
            _ => None,
        };
        if var_name.is_none() {
            let response = Response::new_ok(req.id.clone(), Option::<Location>::None);
            connection.sender.send(Message::Response(response))?;
            return Ok(());
        }
        let var_name = var_name.unwrap();
        let position = variables.get_variable_definition(var_name, line as u32);
        if position.is_none() {
            let response = Response::new_ok(req.id.clone(), Option::<Location>::None);
            connection.sender.send(Message::Response(response))?;
            return Ok(());
        }
        let position = position.unwrap();
        let location = Location {
            uri: parsed_params.text_document.uri,
            range: position.definition,
        };
        connection.sender.send(Message::Response(Response::new_ok(
            req.id.clone(),
            Some(location),
        )))?;
    } else if req.method == "textDocument/completion" {
        // let parsed_params: lsp_types::CompletionParams =
        //     serde_json::from_value(req.params.clone()).unwrap();
        let position_params: lsp_types::TextDocumentPositionParams =
            serde_json::from_value(req.params.clone()).unwrap();
        let line = position_params.position.line as usize;
        let visible = variables.get_all_visible_variables(line as u32);
        let mut completion_items = visible
            .iter()
            .map(|v| CompletionItem {
                label: v.name.clone(),
                kind: Some(lsp_types::CompletionItemKind::VARIABLE),
                tags: None,
                detail: None,
                deprecated: Some(false),
                preselect: None,
                sort_text: None,
                filter_text: None,
                insert_text: None,
                ..Default::default()
            })
            .collect::<Vec<CompletionItem>>();
        let keywords = vec![
            "if", "elseif", "else", "while", "return", "break", "continue", "var", "const", "fn",
            "import",
        ];
        completion_items.extend(keywords.iter().map(|k| CompletionItem {
            label: k.to_string(),
            kind: Some(lsp_types::CompletionItemKind::KEYWORD),
            tags: None,
            detail: None,
            deprecated: Some(false),
            preselect: None,
            sort_text: None,
            filter_text: None,
            insert_text: None,
            ..Default::default()
        }));
        connection.sender.send(Message::Response(Response::new_ok(
            req.id.clone(),
            Some(lsp_types::CompletionList {
                is_incomplete: false,
                items: completion_items,
            }),
        )))?;
        //dsf
    }
    Ok(())
}
fn update_everything(
    ref_errs: &mut Vec<crate::variables::ReferenceError>,
    variables: &mut Variables,
    log_file: &mut std::fs::File,
    formats: &mut Vec<TextEdit>,
    connection: &Connection,
    uri: &lsp_types::Url,
    file_contents: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let update_start_time = std::time::Instant::now();
    (*ref_errs, *variables) = match variables::parse_file(&file_contents, log_file) {
        Ok((ref_errs, variables)) => (ref_errs, variables),
        Err(e) => {
            log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
            (vec![], Variables::new())
        }
    };
    match publish_diagnostics(file_contents.clone(), connection, &uri, log_file, &ref_errs) {
        Ok(false) => {
            *formats = vec![];
        }
        Ok(true) => {
            *formats = match get_formatted_document(file_contents.clone(), log_file) {
                Ok(formats) => formats,
                Err(e) => {
                    log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
                    vec![]
                }
            };
        }
        Err(e) => {
            *formats = vec![];
            log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
        }
    }
    let update_end_time = std::time::Instant::now();
    log_file.write_all(
        format!(
            "Update time: {}us\n",
            update_end_time
                .duration_since(update_start_time)
                .as_micros()
        )
        .as_bytes(),
    )?;
    Ok(())
}
fn handle_notification(
    n: &lsp_server::Notification,
    file_contents: &mut String,
    ref_errs: &mut Vec<crate::variables::ReferenceError>,
    variables: &mut Variables,
    log_file: &mut std::fs::File,
    formats: &mut Vec<TextEdit>,
    connection: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    log_file.write_all(format!("Notification: {:?}\n", n).as_bytes())?;
    if n.method == "textDocument/didSave" {
        let params: DidSaveTextDocumentParams = serde_json::from_value(n.params.clone()).unwrap();
        let uri = params.text_document.uri;
        *file_contents = std::fs::read_to_string(uri.to_string().replace("file://", ""))?;
        update_everything(
            ref_errs,
            variables,
            log_file,
            formats,
            connection,
            &uri,
            file_contents,
        )?;
    } else if n.method == "textDocument/didChange" {
        let params: lsp_types::DidChangeTextDocumentParams =
            serde_json::from_value(n.params.clone()).unwrap();
        *file_contents = params.content_changes[0].text.clone();
        let uri = params.text_document.uri;
        update_everything(
            ref_errs,
            variables,
            log_file,
            formats,
            connection,
            &uri,
            file_contents,
        )?;
        // log_file.write_all(
        //     format!("Params: {}\n", params.content_changes[0].text).as_bytes(),
        // )?;
    } else if n.method == "textDocument/didOpen" {
        let params: lsp_types::DidOpenTextDocumentParams =
            serde_json::from_value(n.params.clone()).unwrap();
        let uri = params.text_document.uri;
        *file_contents = std::fs::read_to_string(uri.to_string().replace("file://", ""))?;
        update_everything(
            ref_errs,
            variables,
            log_file,
            formats,
            connection,
            &uri,
            file_contents,
        )?;
    }
    Ok(())
}
fn main_server(
    connection: &Connection,
    _params: &serde_json::Value,
    log_file: &mut std::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: String = "".to_string();
    let mut formats: Vec<TextEdit> = vec![];
    let mut ref_errs = vec![];
    let mut variables = Variables::new();
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
                handle_request(
                    &file_contents,
                    &req,
                    log_file,
                    connection,
                    &formats,
                    &ref_errs,
                    &variables,
                )?;
            }
            Message::Response(resp) => {
                log_file.write_all(format!("Response: {:?}\n", resp).as_bytes())?;
            }
            Message::Notification(n) => {
                handle_notification(
                    &n,
                    &mut file_contents,
                    &mut ref_errs,
                    &mut variables,
                    log_file,
                    &mut formats,
                    connection,
                )?;
            }
        };
    }
    // }
    Ok(())
}
fn actual_main(log_file: &mut std::fs::File) -> Result<(), Box<dyn ::std::error::Error>> {
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
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: Some(vec![]),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(true),
            },
            completion_item: Some(lsp_types::CompletionOptionsCompletionItem {
                label_details_support: Some(true),
            }),
        }),
        ..Default::default()
    };
    log_file.write_all(b"Started server\n")?;
    let capabilities = serde_json::to_value(&server_capabilities).unwrap();
    log_file.write_all(format!("Capabilities: {:?}\n", capabilities).as_bytes())?;
    let init_params = connection.initialize(capabilities)?;
    // log_file.write_all(format!("Init params: {:?}\n", init_params).as_bytes())?;
    // std::fs::write(
    //     "~/projects/maple/lsp/init_params.json",
    //     init_params.to_string(),
    // )?;

    main_server(&connection, &init_params, log_file)?;

    log_file.write_all(b"Shutting down\n")?;
    // smh io_threads.join() blocks the task from ending
    // io_threads.join()?;
    // log_file.write_all(b"Shutting down\n")?;
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Starting server");
    let dir = match std::env::var("HOME") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(Box::new(e));
        }
    };
    let mut log_file = match std::fs::File::options()
        .create(true)
        .append(true)
        .open(format!("{}{}", dir, "/.local/state/nvim/lsp.log"))
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(Box::new(e));
        }
    };
    match actual_main(&mut log_file) {
        Ok(_) => {
            log_file.write_all(b"Success\n")?;
            Ok(())
        }
        Err(e) => {
            log_file.write_all(format!("Error: {:?}\n", e).as_bytes())?;
            eprintln!("{:?}", e);
            Err(e)
        }
    }
}
