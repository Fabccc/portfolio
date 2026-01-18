use leptos::prelude::*;

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
    let yaml_file = r#"apiVersion: portfolio/v1
kind: Project
metadata:
  name: portfolio-rs
  namespace: fabien-cayre
spec:
  template:
    spec:
      restartPolicy: Never
      source: https://github.com/Fabccc/portfolio
      type: personal-project
      stacks:
      - name: spa
        framework: leptos
        language: rust
        why: Why not
"#;
    view! {
        <div class="w-full">
            <div class="flex flex-col">
                <div class="flex flex-row border-b border-b-zinc-700 border-l border-l-zinc-700">
                    <span class="py-2 px-4 mr-3 border-r border-r-zinc-700 bg-zinc-700">"main.yaml ✕"</span>
                    <span class="py-2 px-4 mr-3 ">"on-premise.yaml"</span>
                    <span class="py-2 px-4 mr-3 ">"my-homelab.yaml"</span>
                    <span class="py-2 px-4 mr-3 ">"otomny.yaml"</span>
                    <span class="py-2 px-4 mr-3 ">"certs.tf"</span>
                </div>
                <pre class="pl-4 pt-3">
                    {yaml_file}
                </pre>
            </div>
        </div>
    }
}
