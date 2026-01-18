use leptos::prelude::*;

use crate::highlighter::language::Lang;

mod language;

#[component]
pub fn CodeBlock(
    #[prop()]
    lang: Lang
){
}