use crate::highlighter::{Highlighter, HighlighterContext};
use itertools::Itertools;
use leptos::logging::log;

#[derive(Default)]
pub struct ShellHighlighterContext {}

impl HighlighterContext for ShellHighlighterContext {
    
}

pub struct ShellHighlighter {}

const KEY_COLOR: &str = "text-blue-400";
const DEFAULT_COLOR: &str = "text-gray-100";
const STRING_COLOR: &str = "text-gray-100";
const COMMENT_COLOR: &str = "text-green-600";

impl Highlighter for ShellHighlighter {
    
    fn parse_analyze(&self, tokenized_string: Vec<&str>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let tokenized_string_length = tokenized_string.len();
        let mut current_index: usize = 0;
        let mut indent_count = 0;
        let mut indent_consumed = false;
        let mut comment = vec![];
        let mut is_comment = false;
        let mut stringquote = vec![];
        let mut is_stringquote: bool = false;
        let keywords = ["cp", "mv", "find", "grep", "xargs", "awk", "|", "kubectl"];

        while current_index < tokenized_string_length {
            let token = tokenized_string[current_index];
            if !token.is_empty() && !indent_consumed && indent_count > 0 {
                res.push(format!("<span class=\"w-{} inline-block\"></span>", indent_count));  
                indent_consumed = true;
                indent_count = 0;
            }
            if token == "\n" {
                if is_comment {
                    res.push(format!("<span class=\"{}\">{}</span>", COMMENT_COLOR, comment.join(" ")));
                }
                res.push("<br>".to_string());
                indent_count = 0;
                indent_consumed = false;
                is_comment = false;
                is_stringquote = false;
                stringquote.clear();
                comment.clear();
            }else if token.starts_with("http:"){
                
            }else if is_comment {
                comment.push(token);
            }else if token.is_empty() && !indent_consumed {
                // tabulation size;
                indent_count += 1;
            }else if token.starts_with("#") {
                is_comment = true;
                comment.push(token);
            }else if is_stringquote && !token.contains("\"") {
                stringquote.push(token);
                stringquote.push(" ");
            }else if token.contains("\"") {
                // String case
                let total_quotes = token.chars().filter(|b| *b == '\"').count();
                log!("Total quote for '{}' is {total_quotes}", token);
                if token.starts_with("\"") && token.ends_with("\"") && token.len() > 1{
                    // token is quoted, push directly and skip
                    res.push(format!("<span class=\"{}\">{}</span>", STRING_COLOR, token));
                } else if total_quotes == 1 && is_stringquote {
                    let (left, right) = token.split_once("\"").unwrap();
                    stringquote.push(left);
                    stringquote.push("\"");
                    is_stringquote = false;
                    res.push(format!("<span class=\"{}\">{}</span>", STRING_COLOR, stringquote.join("")));
                    stringquote.clear();
                    res.push(format!("<span class=\"{}\">{}</span>", DEFAULT_COLOR, right));
                } else if total_quotes == 1 && !is_stringquote{
                    let (left, right) = token.split_once("\"").unwrap();
                    res.push(format!("<span class=\"{}\">{}</span>", DEFAULT_COLOR, left));
                    stringquote.push("\"");
                    stringquote.push(right);
                    stringquote.push(" ");
                    is_stringquote = true;
                }else {
                    let splitted_quotes: Vec<&str> = token.split('\"').intersperse("\"").collect();
                    let mut splitted_index = 0;
                    while splitted_index < splitted_quotes.len() {
                        let splitted_token = splitted_quotes[splitted_index];
                        if is_stringquote && splitted_token == "\"" {
                            stringquote.push(splitted_token);
                            is_stringquote = false;
                            res.push(format!("<span class=\"{}\">{}</span>", STRING_COLOR, stringquote.join("")));
                            stringquote.clear();
                        }else if !is_stringquote && splitted_token == "\"" {
                            is_stringquote = true;
                            stringquote.push(splitted_token);
                        } else if is_stringquote{
                            stringquote.push(splitted_token);
                        } else{
                            res.push(format!("<span class=\"{}\">{}</span>", DEFAULT_COLOR, splitted_token));
                        }
                        splitted_index+=1;
                    }
                }
            }else if keywords.contains(&token) {
                res.push(format!("<span class=\"{}\">{}</span>", KEY_COLOR, token));
            }else{
                res.push(format!("<span class=\"{}\">{}</span>", DEFAULT_COLOR, token));
            }
            
            current_index += 1
        }

        res
    }

}