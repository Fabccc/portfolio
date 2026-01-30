use leptos::{logging::log, prelude::*};
use reactive_stores::Store;

use crate::highlighter::{CodeBlock, language::Lang};

#[derive(Store, Debug, Clone)]
pub struct EditorState {
    active_editor: String,
    #[store(key: String = |row| row.key.clone())]
    editors: Vec<EditorEntry>,
}

#[derive(Store, Debug, Clone, PartialEq)]
struct EditorEntry {
    key: String,
    file_content: &'static str,
    lang: Option<Lang>
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            active_editor: "main.yaml".to_string(),
            editors: vec![
                EditorEntry {
                    key: "main.yaml".to_string(),
                    file_content: include_str!("ops/main.yaml"),
                    lang: Some(Lang::Yaml),
                },
                EditorEntry {
                    key: "multi-impact.Dockerfile".to_string(),
                    file_content: include_str!("ops/multi-impact.Dockerfile"),
                    lang: Some(Lang::Dockerfile),
                },
                EditorEntry {
                    key: "self-hosted.yaml".to_string(),
                    file_content: include_str!("ops/self-hosted.yaml"),
                    lang: Some(Lang::Yaml),
                },
                EditorEntry {
                    key: "contributions.sh".to_string(),
                    file_content: include_str!("ops/contributions.sh"),
                    lang: Some(Lang::Shell),
                },
                EditorEntry {
                    key: "otomny.tf".to_string(),
                    file_content: include_str!("ops/otomny.tf"),
                    lang: None,
                },
            ],
        }
    }
}

#[component]
pub fn EditorNav() -> impl IntoView {
    let editor_state: Store<EditorState> = expect_context::<Store<EditorState>>();
    let active_editor = editor_state.active_editor();
    let set_active = SignalSetter::map(move |v| {
        active_editor.set(v);
    });

    let tab_panes = move || {
        editor_state.editors()
        .into_iter()
        .map(move |editor_entry| {
            view! { <TabPane file_name=editor_entry.key() active_file=active_editor set_active=set_active /> }
        })
        .collect::<Vec<_>>()
    };

    view! {
        <div class="flex flex-row border-b border-l border-b-zinc-700 border-l-zinc-700">
            {tab_panes}
        </div>
    }
}

#[component]
pub fn EditorContent() -> impl IntoView {
    let editor_state: Store<EditorState> = expect_context::<Store<EditorState>>();
    let editors = editor_state.editors();
    let active_editor = editor_state.active_editor();
    move || {
        editors
            .into_iter()
            .find(|i| i.clone().key().get() == active_editor.get())
            .map(|val| {
                let val_file_content= val.clone().file_content();
                let val_lang = val.clone().lang();
                let val_file_content_string = Signal::derive(move || val_file_content.get().to_string());
                if val_lang.read().is_some() {
                    let val_lang= Signal::derive(move || val_lang.get().unwrap());
                    view! { <CodeBlock lang=val_lang content=val_file_content_string /> }.into_any()
                }else{
                    view! { <pre>{val.file_content()}</pre> }.into_any()
                }
            })
    }
}

// <span class="py-2 px-4 mr-3 hover:after:content-['✕'] hover:after:ml-2 hover:after:absolute">{k.clone()}</span>

#[component]
fn TabPane(
    #[prop(into)] file_name: Signal<String>,
    #[prop(into)] active_file: Signal<String>,
    #[prop(into)] set_active: SignalSetter<String>,
) -> impl IntoView {
    let is_active = move || file_name.get() == active_file.get();
    move || {
        if is_active() {
            view! {
                <span class="py-2 pr-6 pl-3 border-r border-r-zinc-700 bg-zinc-700 after:content-['✕'] after:ml-0.5 after:absolute">
                    {file_name.get()}
                </span>
            }
            .into_any()
        } else {
            view! {
                <span
                    class="py-2 pr-6 pl-3 hover:cursor-pointer hover:bg-zinc-800 hover:after:content-['✕'] hover:after:ml-0.5 hover:after:absolute"
                    on:click=move |_| {
                        log!("{:?}", file_name.get());
                        set_active.set(file_name.get())
                    }
                >
                    {file_name.get()}
                </span>
            }.into_any()
        }
    }
}
