use lsp_types::Range;
use maple_rs::parser::Parser;
pub struct VariableDefinition {
    name: String,
    visible: Range,
    definition: Range,
    scope_level: u32,
}

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
}

pub fn parse_file(contents: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Parser::new(contents.clone());
    let ast = parser.parse(true);
    let ast = match ast {
        Ok(ast) => ast,
        Err(e) => {
            return Err("Error parsing file".into());
        }
    };
    Ok(())
}
