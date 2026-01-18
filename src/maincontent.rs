use leptos::prelude::*;

use crate::state::editor::{EditorContent, EditorNav};

// Idée:
// - portfolio
// - mon player musique Rump
// - Reverse engineering NFS Carbon modèle
// - Otomny
// - Mainlibre

// Format
// YAML resources kubernetes
// YAML pipeline gitlab
// YAML Helmchart
// TF Terraform
// Dockerfile
// YAML Ansible

#[component]
pub fn MainContent() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col h-full">
            <EditorNav/>
            <div
              class="pl-3 pt-3 grow min-h-0
                     code-scrollbar overflow-y-scroll">
              <EditorContent/>
            </div>
        </div>
    }
}