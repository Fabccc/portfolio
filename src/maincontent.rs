use leptos::prelude::*;

use crate::state::editor::EditorNav;

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
      maxReplicas: 1
      stack:
        - name: frontend
          type: spa
          framework: leptos 
          language: rust
          why: Why not
      ports:
        - containerPort: "80"
          name: "http"
---
kind: Ingress
metadata:
  name: portfolio-rs
  namespace: fabien-cayre
spec:
  ingressClassName: "nginx"
  tls:
    - hosts:
      - www.fabiencayre.fr
      secretName: certbot-fabiencayre
  rules:
    - http:
      - path: /
        pathType: Prefix
        backend:
           service:
             name: portfolio-rs
             port:
               name: http

"#;
    view! {
        <div class="w-full flex flex-col h-full">
            <EditorNav/>
            <div
              class="pl-3 pt-3 grow min-h-0
                     code-scrollbar overflow-y-scroll">
              <pre>
                  {yaml_file}
              </pre>
            </div>
        </div>
    }
}