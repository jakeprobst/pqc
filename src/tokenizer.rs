use parser::Parser;


#[derive(Debug)]
pub enum TokenizeError {
}

pub struct Tokenizer {
    pub script: String,
}

pub fn tokenize_script(script: String) -> Vec<String> {
    let mut result = Vec::new();

    let mut current = String::new();
    let mut in_quote = false;
    let mut in_comment = false;
    for c in script.chars() {
        if in_quote {
            current.push(c);
            if c == '"' {
                in_quote = false;
            }
        }
        else if in_comment {
            if c == '\n' {
                in_comment = false;
            }
        }
        else if c == '#' {
            in_comment = true;
        }
        else if c == '(' || c == ')' {
            if current.len() > 0 {
                result.push(current);
                current = String::new();
            }
            result.push(c.to_string());
        }
        else if c.is_whitespace() {
            if current.len() > 0 {
                result.push(current);
                current = String::new();
            }
        }
        else if c == '"' {
            current.push(c);
            in_quote = true;
        }
        else {
            current.push(c);
        }
    }

    return result;
}



impl Tokenizer {
    pub fn new(script: String) -> Tokenizer {
        Tokenizer {
            script: script
        }
    }
    
    pub fn tokenize(self) -> Result<Parser, TokenizeError> {
        Ok(Parser::new(tokenize_script(self.script)))
    }
}



