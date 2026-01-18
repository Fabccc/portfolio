use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Clone, Debug, Store)]
pub struct EditorState {
    active_editor: String   
}


#[component]
pub fn EditorNav() -> impl IntoView {

    view! {
        <div class="flex flex-row border-b border-b-zinc-700 border-l border-l-zinc-700">
            <span class="py-2 px-4 mr-3 border-r border-r-zinc-700 bg-zinc-700">"main.yaml ✕"</span>
            <span class="py-2 px-4 mr-3 hover:after:content-['✕'] hover:after:absolute">"on-premise.yaml"</span>
            <span class="py-2 px-4 mr-3 hover:after:content-['✕'] hover:after:absolute">"my-homelab.yaml"</span>
            <span class="py-2 px-4 mr-3 hover:after:content-['✕'] hover:after:absolute">"otomny.yaml"</span>
            <span class="py-2 px-4 mr-3 hover:after:content-['✕'] hover:after:absolute">"certs.tf"</span>
        </div>
    }
}

#[component]
fn TabPane(
  #[prop()]
  file_name: &'static str,
  #[prop()]
  active: ReadSignal<bool>,
  #[prop()]
  set_active: WriteSignal<bool>
) -> impl IntoView {
  if active.get() {
    view! {
      <span class="py-2 px-4 mr-3 border-r border-r-zinc-700 bg-zinc-700">{format!("{} ✕", file_name)}</span>
    }.into_any()
  }else {
    view! {
      <span class="py-2 px-4 mr-3 hover:after:content-['✕'] hover:after:absolute">{file_name}</span>
    }.into_any()
  }

}