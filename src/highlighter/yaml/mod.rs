
use crate::highlighter::{Highlighter, HighlighterContext};

#[derive(Default)]
pub struct YamlHighlighterContext {}

impl HighlighterContext for YamlHighlighterContext {
    
}

pub struct YamlHighlighter {}

const KEY_COLOR: &str = "text-red-400";
const DEFAULT_COLOR: &str = "text-yellow-100";
const COMMENT_COLOR: &str = "text-green-900";


impl Highlighter for YamlHighlighter {


    fn parse_analyze(&self, tokenized_string: Vec<&str>) -> Vec<String> {
        let tokenized_string_length = tokenized_string.len();
        let mut res: Vec<String> = vec![];
        let mut current_index: usize = 0;
        let mut is_multiline_string = false;
        let mut yaml_key_consumed = false;
        let mut indent_count = 0;
        let mut indent_consumed = false;

        while current_index < tokenized_string_length {
            let token = tokenized_string[current_index];
            if !token.is_empty() && !indent_consumed && indent_count > 0 {
                res.push(format!("<span class=\"w-{} inline-block\"></span>", indent_count));  
                indent_consumed = true;
                indent_count = 0;
            }
            if token == "\n" {
                res.push("<br>".to_string());
                yaml_key_consumed = false;
                indent_count = 0;
                indent_consumed = false;
            }else if token.is_empty() && !indent_consumed {
                // tabulation size;
                indent_count += 1;
            } else if token.starts_with("#") {
                let mut comment = vec![] ;
                let mut eol_index = 0;
                while tokenized_string[current_index + eol_index] != "\n" {
                    if tokenized_string[current_index + eol_index] == "\n" {
                        break
                    }
                    comment.push(tokenized_string[current_index + eol_index]);
                    eol_index+=1;
                }
                current_index += eol_index;
                res.push(format!("<span class=\"{}\">{}</span>", COMMENT_COLOR, comment.join(" ")));
            } else if token.ends_with(":") && !yaml_key_consumed {
                res.push(format!("<span class=\"{}\">{}</span>", KEY_COLOR, token));
                yaml_key_consumed = true;
            } else if ! token.is_empty(){
                res.push(format!("<span class=\"{}\">{}</span>", DEFAULT_COLOR, token));
            }
            current_index+=1;
        }
    
        res
    }
}
