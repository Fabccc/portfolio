use chrono::Local;
use leptos::{logging::log, prelude::*};
use itertools::Itertools;

use crate::highlighter::{dockerfile::DockerfileHighlighter, language::Lang, shell::ShellHighlighter, yaml::YamlHighlighter};

pub mod language;
pub mod yaml;
pub mod dockerfile;
pub mod shell;
pub mod lexer;


pub trait HighlighterContext {}

pub trait Highlighter {

    fn parse_analyze(&self, tokenized_string: Vec<&str>) -> Vec<String>;

}


#[component]
pub fn CodeBlock(
        #[prop()]
        lang: Signal<Lang>,
        #[prop()]
        content: Signal<String>
    ) -> impl IntoView {
    let lang_cloned = lang.clone();
    let highlighter = move || find_highlighter(lang.get());
    let content = move || {
        let start = Local::now().time();
        let analyzer =  highlighter();
        let content_binded = content.clone().get();

        let tokenized_string: Vec<&str> = content_binded.split(' ')
            .flat_map(|s| s.split("\n").intersperse("\n"))
            .collect();
        let res = analyzer.parse_analyze(tokenized_string);
        let res = res.join(" ");
        
        let end = Local::now().time();
        log!("Rendering of content took {}ms with lang {}", (end-start).num_milliseconds(), lang.get());
        res
    };

    view! { <div inner_html=content></div> }
}

fn find_highlighter(lang: Lang) -> Box<dyn Highlighter>{
    match lang  {
        Lang::Yaml => Box::new(YamlHighlighter {}),
        Lang::Dockerfile => Box::new(DockerfileHighlighter {}),
        Lang::Shell => Box::new(ShellHighlighter {})
    }
}
