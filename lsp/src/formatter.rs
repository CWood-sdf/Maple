use lsp_types::TextEdit;
use maple_rs::ast::ASTType;
use maple_rs::ast::AST;
use maple_rs::lexer::TokenType;
use maple_rs::parser::{ObjectKey, Parser};
fn format_block(
    ast: &Vec<Box<AST>>,
    indent: usize,
    indent_size: usize,
    log_file: &mut std::fs::File,
) -> Vec<String> {
    let mut formatted = vec![];
    for ast in ast {
        for line in format(ast, indent, indent_size, log_file) {
            formatted.push(line);
        }
    }
    formatted
        .iter()
        .map(|x| format!("{}{}", " ".repeat(indent_size), x))
        .collect()
}
fn format_operator(
    left: &Box<AST>,
    right: &Box<AST>,
    op: &str,
    indent: usize,
    indent_size: usize,
    log_file: &mut std::fs::File,
) -> Vec<String> {
    vec![format!(
        "{} {} {}",
        format(&left, indent, indent_size, log_file).join("\n"),
        op,
        format(&right, indent, indent_size, log_file).join("\n")
    )]
}
fn format(
    ast: &Box<AST>,
    indent: usize,
    indent_size: usize,
    log_file: &mut std::fs::File,
) -> Vec<String> {
    let x = match &ast.t {
        ASTType::Import(str) => vec![format!("import {}", str)],
        ASTType::DotAccess(l, r) => vec![format!(
            "{}.{}",
            format(&l, indent, indent_size, log_file).join("\n"),
            r
        )],
        ASTType::BracketAccess(l, r) => vec![format!(
            "{}[{}]",
            format(&l, indent, indent_size, log_file).join("\n"),
            format(&r, indent, indent_size, log_file).join("\n")
        )],
        ASTType::ArrayLiteral(a) => vec![format!(
            "[{}]",
            a.iter()
                .map(|x| format!("{}", format(&x, indent, indent_size, log_file).join("\n")))
                .collect::<Vec<String>>()
                .join(", ")
        )],
        ASTType::ObjectLiteral(l) => vec![format!(
            "{{{}{}}}",
            l.iter()
                .map(|x| format!(
                    "{}{} = {}",
                    " ".repeat(indent_size),
                    match &(*x).0 {
                        ObjectKey::String(str) => format!("{}", str),
                        ObjectKey::Number(num) => {
                            format!("{}", num)
                        }
                    },
                    format(&(*x).1, indent + indent_size, indent_size, log_file)
                        .iter()
                        .enumerate()
                        .map(|(i, x)| {
                            if i == 0 {
                                x.clone()
                            } else {
                                format!("{}{}", " ".repeat(indent_size), x)
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                ))
                .map(|x| format!("\n{}", x))
                .collect::<Vec<String>>()
                .join(","),
            match l.len() {
                0 => "".to_string(),
                _ => "\n".to_string(),
            }
        )],
        ASTType::CharacterLiteral(c) => vec![format!("'{}'", c)],
        ASTType::StringLiteral(s) => vec![format!(
            "\"{}\"",
            s.replace("\n", "\\n")
                .replace("\r", "\\r")
                .replace("\"", "\\\"")
                .replace("\t", "\\t")
                .replace("\0", "\\0")
        )],
        ASTType::NumberLiteral(n) => vec![format!("{}", n)],
        ASTType::BooleanLiteral(b) => vec![format!("{}", b)],
        ASTType::Paren(l) => vec![format!(
            "({})",
            format(&l, indent, indent_size, log_file).join("\n")
        )],
        ASTType::VariableDeclaration(name, is_const) => vec![format!(
            "{} {}",
            if *is_const { "const" } else { "var" },
            name
        )],
        ASTType::FunctionLiteral(f) => vec![format!(
            "fn ({}) {{\n{}\n{}}}",
            f.params.join(", "),
            format_block(&f.body, indent, indent_size, log_file).join("\n"),
            "".to_string()
        )],
        ASTType::FunctionCall(f, args) => vec![format!(
            "{}({})",
            format(&f, indent, indent_size, log_file).join("\n"),
            args.iter()
                .map(|x| format!("{}", format(&x, indent, indent_size, log_file).join("\n")))
                .collect::<Vec<String>>()
                .join(", ")
        )],
        ASTType::If(lit) => vec![format!(
            "if {} {{\n{}\n{}}} {} {}",
            format(&lit.cond, indent, indent_size, log_file).join("\n"),
            format_block(&lit.body, indent, indent_size, log_file).join("\n"),
            "".to_string(),
            lit.elseifs
                .iter()
                .map(|x| format!(
                    "elseif {} {{\n{}\n{}}}",
                    format(&x.0, indent, indent_size, log_file).join("\n"),
                    format_block(&x.1, indent, indent_size, log_file).join("\n"),
                    "".to_string()
                ))
                .collect::<Vec<String>>()
                .join(" "),
            match lit.else_body {
                Some(ref x) => format!(
                    "else {{\n{}\n{}}}",
                    format_block(&x, indent, indent_size, log_file).join("\n"),
                    "".to_string()
                ),
                None => "".to_string(),
            }
        )],
        ASTType::While(cond, block) => vec![format!(
            "while {} {{\n{}\n{}}}",
            format(&cond, indent, indent_size, log_file).join("\n"),
            format_block(&block, indent, indent_size, log_file).join("\n"),
            "".to_string()
        )],
        ASTType::OpPls(l, r) => format_operator(l, r, "+", indent, indent_size, log_file),
        ASTType::OpMns(l, r) => format_operator(l, r, "-", indent, indent_size, log_file),
        ASTType::OpTimes(l, r) => format_operator(l, r, "*", indent, indent_size, log_file), // *
        ASTType::OpDiv(l, r) => format_operator(l, r, "/", indent, indent_size, log_file),   // /
        ASTType::OpMnsPrefix(l) => {
            vec![format!(
                "-{}",
                format(&l, indent, indent_size, log_file).join("\n"),
            )]
        } // -
        ASTType::OpEq(l, r) => {
            let mut ret = Option::None;
            if let ASTType::VariableDeclaration(name, true) = &l.t {
                if let ASTType::FunctionLiteral(f) = &r.t {
                    ret = Some(vec![format!(
                        "fn {} ({}) {{\n{}\n{}}}",
                        name,
                        f.params.join(", "),
                        format_block(&f.body, indent, indent_size, log_file).join("\n"),
                        "".to_string()
                    )])
                }
            }
            match ret {
                Some(x) => x,
                None => format_operator(l, r, "=", indent, indent_size, log_file),
            }
        } // =
        ASTType::OpEqEq(l, r) => format_operator(l, r, "==", indent, indent_size, log_file), // ==
        ASTType::OpPlsEq(l, r) => format_operator(l, r, "+=", indent, indent_size, log_file), // +=
        ASTType::OpNotEq(l, r) => format_operator(l, r, "!=", indent, indent_size, log_file), // !=
        ASTType::OpNot(l) => {
            vec![format!(
                "!{}",
                format(&l, indent, indent_size, log_file).join("\n"),
            )]
        } // -
        ASTType::OpAndAnd(l, r) => format_operator(l, r, "&&", indent, indent_size, log_file), // &&
        ASTType::OpOrOr(l, r) => format_operator(l, r, "||", indent, indent_size, log_file), // ||
        ASTType::OpGt(l, r) => format_operator(l, r, ">", indent, indent_size, log_file),    // >
        ASTType::OpLt(l, r) => format_operator(l, r, "<", indent, indent_size, log_file),    // <
        ASTType::OpGtEq(l, r) => format_operator(l, r, ">=", indent, indent_size, log_file), // >=
        ASTType::OpLtEq(l, r) => format_operator(l, r, "<=", indent, indent_size, log_file), // <=
        ASTType::VariableAccess(s) => vec![format!("{}", s)],
        ASTType::Return(ret) => vec![format!(
            "return {}",
            format(&ret, indent, indent_size, log_file).join("\n")
        )],
        ASTType::Break => vec![format!("break")],
        ASTType::Continue => vec![format!("continue")],
    };
    let mut ret = vec![];
    for line in x {
        // if line.contains(&"\n".to_string()) {
        for l in line.split("\n") {
            ret.push(l.to_string());
        }
        // } else {
        //     ret.push(line);
        // }
    }
    ret
}
pub fn get_formatted_document(
    contents: String,
    log_file: &mut std::fs::File,
) -> Result<Vec<TextEdit>, Box<dyn std::error::Error>> {
    // let start_time = std::time::Instant::now();
    // let file_path = uri.to_string().replace("file://", "");
    // log_file.write_all(format!("Formatting {}\n", file_path).as_bytes())?;
    // log_file.write_all(format!("File path: {}\n", file_path).as_bytes())?;
    // let file_contents = std::fs::read_to_string(file_path.clone())?;
    let lines: Vec<String> = contents
        .clone()
        .split("\n")
        .map(|x| x.to_string())
        .map(|x| x.replace("\r", ""))
        .collect::<Vec<String>>();
    let mut i = 0;
    let mut formatted_lines: Vec<String> = vec![];
    while i < lines.len() {
        let mut inc = 1;
        if i + inc > lines.len() {
            return Err("sdf".into());
        }
        let mut parser = Parser::new(lines[i..(i + inc)].join("\n") + "\n");
        let mut ast_arr = parser.parse(true);
        while ast_arr.is_err() {
            inc += 1;
            if i + inc > lines.len() {
                return Err("sdf".into());
            }
            let s = lines[i..(i + inc)].join("\n") + "\n";
            parser = Parser::new(s);
            ast_arr = parser.parse(true);
        }
        let ast_arr = ast_arr.unwrap();
        let formatted = if ast_arr.len() == 1 {
            format(&ast_arr[0], 0, 4, log_file)
        } else if ast_arr.len() > 1 {
            format_block(&ast_arr, 0, 4, log_file)
        } else {
            vec![]
        };
        let mut new_lines = vec![];
        let non_ast_lines = lines[i..(i + inc)]
            .iter()
            .enumerate()
            .filter(|(_, x)| x.trim() == "" || x.trim().starts_with("//"))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        while non_ast_lines.contains(&new_lines.len()) {
            new_lines.push("".to_string());
            continue;
        }
        for line in formatted {
            while non_ast_lines.contains(&new_lines.len()) {
                new_lines.push("".to_string());
                continue;
            }
            new_lines.push(line);
        }
        let mut comment_lines = vec![];
        for line in &parser.lexer.comments {
            if let TokenType::Comment(s) = &line.t {
                if comment_lines.contains(&line.line) {
                    continue;
                }
                if new_lines[line.line].trim() == "" {
                    let next_line = if line.line + 1 < new_lines.len() {
                        new_lines[line.line + 1].clone()
                    } else {
                        "".to_string()
                    };
                    let indentation = next_line.len() - next_line.trim_start().len();
                    new_lines[line.line] = format!("{}{}", " ".repeat(indentation), s);
                } else {
                    new_lines[line.line] = format!("{} {}", new_lines[line.line], s);
                }
                comment_lines.push(line.line);
            }
        }
        formatted_lines.append(&mut new_lines);
        i += inc;
    }
    let mut edits: Vec<TextEdit> = vec![];
    while formatted_lines.last().unwrap() == &"".to_string() {
        formatted_lines.pop();
    }
    edits.push(TextEdit {
        range: lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types::Position {
                line: lines.len() as u32,
                character: lines.last().unwrap().len() as u32,
            },
        },
        new_text: formatted_lines.join("\n"),
    });
    Ok(edits)
}
