
use crate::highlighter::{Highlighter};

pub struct YamlHighlighterContext {}


pub struct YamlHighlighter {}

const KEY_COLOR: &str = "text-red-400";
const DEFAULT_COLOR: &str = "text-yellow-100";
const COMMENT_COLOR: &str = "text-gray-500";


impl Highlighter for YamlHighlighter {


    fn parse_analyze(&self, tokenized_string: Vec<&str>) -> Vec<String> {
        let tokenized_string_length = tokenized_string.len();
        let mut res: Vec<String> = vec![];
        let mut current_index: usize = 0;
        let mut is_multiline_string = false;
        let mut yaml_key_consumed = false;
        let mut indent_count = 0;
        let mut is_comment = false;
        let mut indent_consumed = false;
        let mut last_color = DEFAULT_COLOR;

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
                is_comment = false;
            }else if token.is_empty() && !indent_consumed {
                // tabulation size;
                indent_count += 1;
            }else if token.starts_with("http://") || token.starts_with("https://"){
                if last_color == KEY_COLOR{
                    last_color = DEFAULT_COLOR;
                }
                let val = format!("<a href=\"{token}\" class=\"{} underline\">{token}</a>", last_color);
                if is_comment {
                    res.push(format!("</span>{val}<span class=\"{}\">", COMMENT_COLOR));
                } else {
                    res.push(val);
                }
            }else if is_comment {
                res.push(token.to_string());
            }else if token.starts_with("#") {
                is_comment = true;
                res.push(format!("<span class=\"{COMMENT_COLOR}\">{token}"));
                last_color = COMMENT_COLOR;
            } else if token.ends_with(":") && !yaml_key_consumed {
                res.push(format!("<span class=\"{}\">{}</span>", KEY_COLOR, token));
                yaml_key_consumed = true;
                last_color = KEY_COLOR;
            } else if ! token.is_empty(){
                res.push(format!("<span class=\"{}\">{}</span>", DEFAULT_COLOR, token));
                last_color = DEFAULT_COLOR;
            }
            current_index+=1;
        }
    
        res
    }
}
