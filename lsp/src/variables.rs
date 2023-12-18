use std::io::Write;

use lsp_types::{Position, Range};
use maple_rs::{
    ast::{ASTType, Block, AST},
    parser::Parser,
};
#[derive(Debug)]
pub struct VariableDefinition {
    pub name: String,
    visible: Range,
    pub definition: Range,
    scope_level: u32,
}

#[derive(Debug)]
pub struct Variables {
    variables: Vec<VariableDefinition>,
}

pub fn line_in_range(line: u32, range: &Range) -> bool {
    line >= range.start.line && line <= range.end.line
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            variables: Vec::new(),
        }
    }
    pub fn add_variable(
        &mut self,
        name: String,
        visible: Range,
        definition: Range,
        scope_level: u32,
    ) {
        self.variables.push(VariableDefinition {
            name,
            visible,
            definition,
            scope_level,
        });
    }
    pub fn get_variable_definition(
        &self,
        name: String,
        ref_line: u32,
    ) -> Option<&VariableDefinition> {
        let mut max_level = -1;
        let mut ret = None;
        for variable in &self.variables {
            if variable.name == name
                && line_in_range(ref_line, &variable.visible)
                && variable.scope_level as i32 > max_level
            {
                max_level = variable.scope_level as i32;
                ret = Some(variable);
            }
        }
        ret
    }
    pub fn variable_exists(&self, name: &String, ref_line: u32) -> bool {
        for variable in &self.variables {
            if &variable.name == name && line_in_range(ref_line, &variable.visible) {
                return true;
            }
        }
        false
    }
    pub fn get_all_visible_variables(&self, ref_line: u32) -> Vec<&VariableDefinition> {
        let mut ret: Vec<&VariableDefinition> = Vec::new();
        for variable in &self.variables {
            if line_in_range(ref_line, &variable.visible) {
                if ret
                    .iter()
                    .filter(|&&x| x.name == variable.name && x.scope_level > variable.scope_level)
                    .count()
                    == 0
                {
                    ret.push(variable);
                } else {
                    ret = ret
                        .iter()
                        .filter(|&&x| {
                            x.name != variable.name || x.scope_level <= variable.scope_level
                        })
                        .map(|x| *x)
                        .collect();
                    ret.push(variable);
                }
            }
        }
        ret
    }
}
#[derive(Debug)]
pub struct ReferenceError {
    pub name: String,
    pub line: u32,
    pub char_start: u32,
    pub char_end: u32,
}

fn get_later_line(l: &Box<AST>, r: &Box<AST>) -> usize {
    let l = get_last_line(&l);
    let r = get_last_line(&r);
    if l > r {
        l
    } else {
        r
    }
}
fn get_last_line(ast: &Box<AST>) -> usize {
    match &ast.t {
        ASTType::OpLtEq(l, r) => get_later_line(l, r),
        ASTType::VariableAccess(_) => ast.token.line,
        ASTType::Return(l) => get_last_line(&l),
        ASTType::Break => ast.token.line,
        ASTType::Continue => ast.token.line,
        ASTType::If(lit) => {
            if lit.else_body.is_some() && lit.else_body.as_ref().unwrap().len() > 0 {
                let else_body = lit.else_body.as_ref().unwrap();
                get_last_line(&else_body[else_body.len() - 1])
            } else if lit.elseifs.iter().filter(|x| x.1.len() > 0).count() > 0 {
                get_last_line(
                    lit.elseifs
                        .iter()
                        .filter(|x| x.1.len() > 0)
                        .last()
                        .unwrap()
                        .1
                        .last()
                        .unwrap(),
                )
            } else if lit.body.len() > 0 {
                get_last_line(&lit.body[lit.body.len() - 1])
            } else {
                ast.token.line
            }
        }
        ASTType::OpEq(l, r) => get_later_line(l, r),
        ASTType::While(_, block) => {
            if block.len() == 0 {
                ast.token.line
            } else {
                get_last_line(&block[block.len() - 1])
            }
        }
        ASTType::OpMnsPrefix(l) => get_last_line(&l),
        ASTType::OpEqEq(l, r) => get_later_line(l, r),
        ASTType::OpPlsEq(l, r) => get_later_line(l, r),
        ASTType::OpNotEq(l, r) => get_later_line(l, r),
        ASTType::OpNot(l) => get_last_line(&l),
        ASTType::OpAndAnd(l, r) => get_later_line(l, r),
        ASTType::OpOrOr(l, r) => get_later_line(l, r),
        ASTType::OpGtEq(l, r) => get_later_line(l, r),
        ASTType::Import(_) => ast.token.line,
        ASTType::OpTimes(l, r) => get_later_line(l, r),
        ASTType::OpDiv(l, r) => get_later_line(l, r),
        ASTType::DotAccess(_, _) => ast.token.line,
        ASTType::StringLiteral(_) => ast.token.line,
        ASTType::OpLt(l, r) => get_later_line(l, r),
        ASTType::BracketAccess(_, _) => ast.token.line,
        ASTType::ObjectLiteral(o) => {
            if o.len() == 0 {
                ast.token.line
            } else {
                get_last_line(&o[o.len() - 1].1)
            }
        }
        ASTType::ArrayLiteral(a) => {
            if a.len() == 0 {
                ast.token.line
            } else {
                get_last_line(&a[a.len() - 1])
            }
        }
        ASTType::OpGt(l, r) => get_later_line(l, r),
        ASTType::CharacterLiteral(_) => ast.token.line,
        ASTType::NumberLiteral(_) => ast.token.line,
        ASTType::BooleanLiteral(_) => ast.token.line,
        ASTType::Paren(l) => get_last_line(&l),
        ASTType::OpPls(l, r) => get_later_line(l, r),
        ASTType::VariableDeclaration(_, _) => ast.token.line,
        ASTType::FunctionLiteral(f) => {
            if f.body.len() == 0 {
                ast.token.line
            } else {
                get_last_line(&f.body[f.body.len() - 1])
            }
        }
        ASTType::OpMns(l, r) => get_later_line(l, r),
        ASTType::FunctionCall(l, r) => {
            if r.len() == 0 {
                get_last_line(&l)
            } else {
                get_last_line(&r[r.len() - 1])
            }
        }
    }
}

fn parse_binop(
    l: &Box<AST>,
    r: &Box<AST>,
    variables: &mut Variables,
    scope_level: u32,
    block_end: Position,
) -> Vec<ReferenceError> {
    let mut ret = parse_ast(l, variables, scope_level, block_end);
    ret.extend(parse_ast(r, variables, scope_level, block_end));
    ret
}
fn parse_ast(
    ast: &Box<AST>,
    variables: &mut Variables,
    scope_level: u32,
    block_end: Position,
) -> Vec<ReferenceError> {
    match &ast.t {
        ASTType::While(l, block) => {
            let mut ret = parse_ast(&l, variables, scope_level, block_end);
            let new_scope_level = scope_level + 1;
            if block.len() > 0 {
                let last_line = get_last_line(block.last().unwrap());
                ret.extend(parse_block(
                    &block,
                    variables,
                    new_scope_level,
                    Position {
                        line: last_line as u32,
                        character: 0,
                    },
                ));
            }
            ret
        }
        ASTType::VariableAccess(name) => {
            if !variables.variable_exists(&name, ast.token.line as u32) {
                vec![ReferenceError {
                    name: name.clone(),
                    line: ast.token.line as u32,
                    char_start: ast.token.char_start as u32,
                    char_end: ast.token.char_end as u32,
                }]
            } else {
                vec![]
            }
        }
        ASTType::OpNotEq(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpOrOr(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpGtEq(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpAndAnd(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpNot(l) => parse_ast(&l, variables, scope_level, block_end),
        ASTType::OpGt(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpPlsEq(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpLt(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpEqEq(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpMnsPrefix(l) => parse_ast(&l, variables, scope_level, block_end),
        ASTType::OpEq(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpLtEq(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::Return(l) => parse_ast(&l, variables, scope_level, block_end),
        ASTType::OpPls(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpMns(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpTimes(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::OpDiv(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::Continue => vec![],
        ASTType::Break => vec![],
        ASTType::If(lit) => {
            let mut ret = parse_ast(&lit.cond, variables, scope_level, block_end);
            let new_scope_level = scope_level + 1;
            if lit.body.len() > 0 {
                let last_line = get_last_line(lit.body.last().unwrap());
                ret.extend(parse_block(
                    &lit.body,
                    variables,
                    new_scope_level,
                    Position {
                        line: last_line as u32,
                        character: 0,
                    },
                ));
            }
            for elseif in &lit.elseifs {
                ret.extend(parse_ast(&elseif.0, variables, scope_level, block_end));
                if elseif.1.len() > 0 {
                    let last_line = get_last_line(elseif.1.last().unwrap());
                    ret.extend(parse_block(
                        &elseif.1,
                        variables,
                        new_scope_level,
                        Position {
                            line: last_line as u32,
                            character: 0,
                        },
                    ));
                }
            }
            if lit.else_body.is_some() && lit.else_body.as_ref().unwrap().len() > 0 {
                let else_body = lit.else_body.as_ref().unwrap();
                let last_line = get_last_line(else_body.last().unwrap());
                ret.extend(parse_block(
                    else_body,
                    variables,
                    new_scope_level,
                    Position {
                        line: last_line as u32,
                        character: 0,
                    },
                ));
            }
            ret
        }
        ASTType::ArrayLiteral(a) => a
            .iter()
            .map(|x| parse_ast(x, variables, scope_level, block_end))
            .flatten()
            .collect(),
        ASTType::CharacterLiteral(_) => vec![],
        ASTType::StringLiteral(_) => vec![],
        ASTType::NumberLiteral(_) => vec![],
        ASTType::BooleanLiteral(_) => vec![],
        ASTType::Paren(l) => parse_ast(&l, variables, scope_level, block_end),
        ASTType::FunctionLiteral(f) => {
            let last_line = get_last_line(&ast);
            let block_end = Position {
                line: last_line as u32,
                character: 0,
            };
            let new_scope_level = scope_level + 1;
            for param in &f.params {
                let name = &param.name;
                variables.add_variable(
                    name.to_string(),
                    Range {
                        start: Position {
                            line: ast.token.line as u32,
                            character: param.char_start as u32,
                        },
                        end: block_end,
                    },
                    Range {
                        start: Position {
                            line: ast.token.line as u32,
                            character: param.char_start as u32,
                        },
                        end: Position {
                            line: ast.token.line as u32,
                            character: param.char_end as u32,
                        },
                    },
                    new_scope_level,
                );
            }
            parse_block(&f.body, variables, new_scope_level, block_end)
        }
        ASTType::FunctionCall(l, r) => {
            let mut ret = parse_ast(&l, variables, scope_level, block_end);
            for v in r {
                ret.extend(parse_ast(&v, variables, scope_level, block_end));
            }
            ret
        }
        ASTType::VariableDeclaration(name, _) => {
            variables.add_variable(
                name.to_string(),
                Range {
                    start: Position {
                        line: ast.token.line as u32,
                        character: ast.token.char_start as u32,
                    },
                    end: block_end,
                },
                Range {
                    start: Position {
                        line: ast.token.line as u32,
                        character: ast.token.char_start as u32,
                    },
                    end: Position {
                        line: ast.token.line as u32,
                        character: ast.token.char_end as u32,
                    },
                },
                scope_level,
            );
            vec![]
        }
        ASTType::Import(_) => vec![],
        ASTType::DotAccess(l, _) => parse_ast(&l, variables, scope_level, block_end),
        ASTType::BracketAccess(l, r) => parse_binop(&l, &r, variables, scope_level, block_end),
        ASTType::ObjectLiteral(o) => o
            .iter()
            .map(|(_, v)| parse_ast(&v, variables, scope_level, block_end))
            .flatten()
            .collect(),
    }
}

pub fn parse_block(
    block: &Block,
    variables: &mut Variables,
    scope_level: u32,
    block_end: Position,
) -> Vec<ReferenceError> {
    if block.len() == 0 {
        return vec![];
    }
    // let block_start = Position {
    //     line: block[0].token.line,
    //     character: 0,
    // };
    let mut ret = vec![];
    for statement in block {
        ret.extend(parse_ast(statement, variables, scope_level, block_end));
    }
    ret
}

pub fn parse_file(
    contents: &String,
    log_file: &mut std::fs::File,
) -> Result<(Vec<ReferenceError>, Variables), Box<dyn std::error::Error>> {
    let mut parser = Parser::new(contents.clone());
    let ast = parser.parse(true);
    let ast = match ast {
        Ok(ast) => ast,
        Err(_) => {
            return Err("Error parsing file".into());
        }
    };
    let scope_level = 0;
    let mut variables = Variables::new();
    let block_end = Position {
        line: contents.lines().count() as u32,
        character: 0,
    };
    variables.add_variable(
        "std".to_string(),
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: block_end,
        },
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: block_end,
        },
        0,
    );
    let errs = parse_block(&ast, &mut variables, scope_level, block_end);
    // log_file.write_all(format!("Variables: {:?}\n", variables).as_bytes())?;
    log_file.write_all(format!("Errors: {:?}\n", errs).as_bytes())?;
    Ok((errs, variables))
}
