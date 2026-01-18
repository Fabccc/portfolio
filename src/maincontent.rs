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
            <div class="flex flex-row border-b border-b-zinc-700 border-l border-l-zinc-700">
                <span class="py-2 px-4 mr-3 border-r border-r-zinc-700 bg-zinc-700">"main.yaml ✕"</span>
                <span class="py-2 px-4 mr-3 ">"on-premise.yaml"</span>
                <span class="py-2 px-4 mr-3 ">"my-homelab.yaml"</span>
                <span class="py-2 px-4 mr-3 ">"otomny.yaml"</span>
                <span class="py-2 px-4 mr-3 ">"certs.tf"</span>
            </div>
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

// <div class="scrollbar-thumb-rounded-full scrollbar-track-rounded-full scrollbar scrollbar-thumb-slate-700 scrollbar-track-slate-300 h-32 overflow-y-scroll">
//     <div class="h-64 bg-slate-400"></div>
// </div>