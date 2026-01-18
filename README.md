# Portfolio

Mon portfolio en Rust avec WebAssembly via leptos dans un Style terminal. 
La doc est géniale d'ailleurs.

## Structure

Le terminal se comporte de 4 panels:
- Un panel de navigation en haut
- Un panel central de prévisualisation de fichier YAML kubernetes (avec syntaxe colorée)
  - Ce panel peut changer pour voir le projet déployé
- Un panel a droite
  - En haut l'utilisation CPU/RAM
  - En bas l'explorateur de ressources K8S

## Idée

D'avoir une sorte de VIM en mode terminal, d'avoir des ressources kubernetes en mode yaml, et de devoir les déployer sur un [K9S](https://github.com/derailed/k9s) (le cli de k8s) afin de voir le contenu du projet en question. C'est ça ma vision.


## Doc

- [Nerd Font Icons Search](https://www.nerdfonts.com/cheat-sheet)
- [Leptos Cookbook](https://book.leptos.dev/)
- [Rust Cookbook]
- [Rust Cookbook datetime](https://rust-lang-nursery.github.io/rust-cookbook/datetime.html)


## Special thanks

- [K9S](https://github.com/derailed/k9s)
- [Leptos](https://github.com/leptos-rs/leptos)
- [Antigravity](https://antigravity.google/), pour m'avoir aidé dans les moments difficile (`map FnOnce exists, but it's method trait bound were not satisfied`)
- [Kai Lentit](https://www.youtube.com/watch?v=TGfQu0bQTKc)