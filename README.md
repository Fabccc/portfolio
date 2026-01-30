# Portfolio

Mon portfolio en Rust avec WebAssembly via leptos dans un Style terminal. parce que.
La doc est géniale d'ailleurs.

## Structure

Le terminal se comporte de 4 panels:
- Un panel de navigation en haut
- Un panel central de prévisualisation de fichier YAML kubernetes (avec syntaxe colorée)
  - Ce panel peut changer pour voir le projet déployé
- Un panel a droite
  - En haut l'utilisation CPU/RAM
  - En bas l'explorateur de ressources K8S

## Choix

- CSR car pas besoin de serveur, même en bundlant le tout on reste < 200ko
- Servi en Http3
- pas de crate `regex`, car augmente la taille du binaire.

## Installation

```shell
cargo install trunk
trunk build --release
miniserve dist/ --index "index.html" -p 8080
```

## Doc

- [Nerd Font Icons Search](https://www.nerdfonts.com/cheat-sheet)
- [Leptos Cookbook](https://book.leptos.dev/)
- [Rust Cookbook datetime](https://rust-lang-nursery.github.io/rust-cookbook/datetime.html)
- [Rust Cookbook]

## Special thanks

- [Leptos](https://github.com/leptos-rs/leptos)
- [Leptos Use](https://leptos-use.rs/)
- [Leptos Icons](https://github.com/carloskiki/leptos-icons)
- [K9S](https://github.com/derailed/k9s)
- [Kai Lentit](https://www.youtube.com/watch?v=TGfQu0bQTKc)
- [Archlinux](https://archlinux.org/)
- [Antigravity](https://antigravity.google/), pour m'avoir aidé dans les moments difficile (`map ... exists, but it's method trait bound were not satisfied`)