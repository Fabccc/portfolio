use icondata::{
    Icon, SiAnsible, SiCilium, SiFlux, SiGitlab, SiGrafana, SiHarbor, SiKubernetes, SiRancher,
    SiSentry,
};
use leptos::{logging::log, prelude::*};
use leptos_icons::Icon;
use leptos_use::{UseIntervalReturn, use_interval};
use reactive_stores::Store;

const TYPEWRITER_CHAR_PER_ITER: usize = 3;
const TYPEWRITER_DELAY_BETWEEN_TWO_CHAR: usize = 20;

#[derive(Store, Debug, Clone)]
pub struct TechnologyState {
    active_technology: Option<TechnologyEntry>,
    #[store(key: &'static str = |row| row.key)]
    technologies: Vec<TechnologyEntry>,
}

impl Default for TechnologyState {
    fn default() -> Self {
        Self {
            active_technology: None,
            technologies: vec![
            TechnologyEntry::new("kubernetes", r#"J'ai pleinement plongé dans l'outil Kubernetes lors de mon arrivé en CDI chez Multi-Impact.
C'est un outil plutôt polyvalent mais plutôt couteux à utiliser, je ne pense pas aux solutions one-fit-all"#, SiKubernetes),
            TechnologyEntry::new("grafana", r#"Grafana est l'outil central à mettre en place pour visualiser rapidement une large quantité de source de données.
Le fait de ne pas avoir 15 sites pour visualiser les données de 20 services est un énorme plus.
Il est également très simple à mettre en place, et propose des fonctionnalités d'entreprise dans la version OSS"#, SiGrafana),
            TechnologyEntry::new("flux", r#"Ma démarche en tant que DevOps est d'appliquer certains principes du développement
dans l'infrastructure. FluxCD est un bon outil pour synchroniser des ressources Kubernetes d'un dépôt git dans un cluster via des ressources natives (CRDs).
La gestion d'upgrade/downgrade devient quasi automatique."#, SiFlux),
            TechnologyEntry::new("gitlab", r#"La référence open source de la forge logicielle, je le considère même plus puissant que Github.
La version opensource permet de faire énormement de choses. Je m'en sers pour maintenir les pipelines de linting, de tests, de déploiement via les CICD components.
Je teste même les composants eux mêmes via les pipelines. Un bon processus CI/CD de bout en bout renforce la stabilité des applicatifs."#, SiGitlab),
            TechnologyEntry::new("cilium", r#"C'est THE interface de réseau pour conteneur. Avec son intégration eBPF, il pulvérise tous les records de performances,
assurant une forte isolation des conteneurs, tout en maximisant le traffic réseau. De la couche 4 à la couche 7 du modèle OSI,
il peut tout filtrer au préalable via ses CRDs de type Network Policies. Je suis entrain de l'étudier pour en faire notre
Gateway API ainsi que notre Service Mesh"#, SiCilium),
            TechnologyEntry::new("rancher", r#"Les CSR sont un lointain souvenir pour moi. La centralisation qu'offre Rancher pour
la gestion des utilisateurs, des clusters, des rôles ainsi que des mises à jours est adapté lorsqu'on gère plusieurs clusters avec des équipes de développeurs."#, SiRancher),
            TechnologyEntry::new("sentry", r#"Présent à mon arrivé, j'ai renforcé l'utilisation du Sentry dans les applications avec
le tracing distribués, le session replay ainsi que les user feedback. C'est un outil très appreciés des PM et des devs, permettant d'identifier
rapidement les problèmes, mais également de retrouver tout le contexte de l'erreur (lien front/backend/micro service/queue)."#, SiSentry),
            TechnologyEntry::new("ansible", r#"Finis les scripts bash et le crontab. Avec AWX et Ansible, l'automatisation
de tâches devient un jeu d'enfant. J'ai proposé de remplacer notre vieux serveur d'automatisation par cet outil.
Travailler avec des playbooks ansible nous à permis de renforcer la stabilité des jobs de backups, de mises à jours de certificats
et de remontage en recette."#, SiAnsible),
            TechnologyEntry::new("harbor", r#"Une des premières propositions que j'ai effecuté chez Multi-Impact est d'ajouter un
registre d'image OCI local. Quand Docker, dans la même période, réduisait le pull d'image a 100/jours,
nous ne pouvions pas soutenir le rythme. GoHarbor est un registre CNCF d'archive au format OCI, donc pas que les images Docker,
mais aussi les charts Helm. Il agit comme un proxy a dockerhub, githubcr, gitlabcr et k8s registry"#, SiHarbor),
        ] }
    }
}

#[derive(Store, Debug, Clone, PartialEq)]
struct TechnologyEntry {
    key: &'static str,
    content: &'static str,
    icon: Icon,
}

impl TechnologyEntry {
    fn new(key: &'static str, content: &'static str, icon: Icon) -> Self {
        Self { key, content, icon }
    }
}

#[component]
pub fn Technologies() -> impl IntoView {
    let state = expect_context::<Store<TechnologyState>>();
    let tech: Signal<Option<TechnologyEntry>> = Signal::from(state.active_technology());

    let UseIntervalReturn { counter, reset, resume, pause, .. } = use_interval(TYPEWRITER_DELAY_BETWEEN_TWO_CHAR as u64);
    
    let counter_scaled = move || counter.get() * TYPEWRITER_CHAR_PER_ITER as u64;

    let clamped_counter = Signal::derive(move || {
        if let Some(t) = state.active_technology().get() {
            let active_technology_len = t.content.len();
            if (counter_scaled() as usize) >= active_technology_len {
                pause();
                return active_technology_len;
            }
            return counter_scaled() as usize;
        }
        0
    });

    let set_tech = SignalSetter::map(move |new_tech: TechnologyEntry| {
        if let Some(old_tech) = state.active_technology().get() &&
            new_tech.key != old_tech.key {
            reset();
            state.active_technology().set(Some(new_tech));
        }else if state.active_technology().get().is_none() {
            reset();
            state.active_technology().set(Some(new_tech));
        }
        resume();
    });


    let technologies_panes = move || {
        state
            .technologies()
            .into_iter()
            .map(move |tech_entry| {
                view! { <Technology tech={move || tech_entry.get()} current_hover=tech set_hover=set_tech /> }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="pt-4 mt-4 text-xs border-t border-t-zinc-700">
            <h1 class="mb-4 text-xl text-center">"Ma stack actuelle"</h1>
            <div class="px-2 grid grid-cols-3 grid-rows-3 gap-2 place-items-center align-middle">{technologies_panes}</div>
            <div>
                <TechnologyContent counter=clamped_counter state=state />
            </div>
        </div>
    }
}

#[component]
fn TechnologyContent(
    #[prop(into)] counter: Signal<usize>,
    #[prop(into)] state: Store<TechnologyState>) -> impl IntoView {
    
    move || {
        if let Some(entry) = state.active_technology().get() {
            let content_as_string = Signal::derive(move || String::from(entry.content));
            let sliced = move || {
                if content_as_string.get().is_empty() {
                    return String::from("");
                }
                let cp = content_as_string.clone().get();
                // utf8 et les charact
                let mut indices=cp.char_indices().map(|(i, _)| i);
                let end = match indices.nth(counter.get()){
                    Some (e) => e,
                    None => content_as_string.read().len()
                };
                String::from(&content_as_string.read()[0..end])
            };
            view! { 
                <div class="px-2 mt-4">
                    <span class="text-sm ">{sliced}</span>
                    <span class="w-0.75 h-3 blinker self-baseline inline-block"></span> 
                </div>
            }.into_any()
        } else {
            view! { <p></p> }.into_any()
        }
    }
}

#[component]
fn Technology(
    #[prop(into)] tech: Signal<TechnologyEntry>,
    #[prop(into)] current_hover: Signal<Option<TechnologyEntry>>,
    #[prop(into)] set_hover: SignalSetter<TechnologyEntry>,
) -> impl IntoView {
    move || {
        let tech_name = tech.get().key;
        let tech_icon = tech.get().icon;
        if let Some(current_tech) = current_hover.get()
        && current_tech.key == tech_name {
            view! {
                <div
                    class="content-center w-20 h-20 bg-blue-500 border border-blue-500 rounded-xs"
                    on:mouseenter=move |_| {
                        log!("{}", current_tech.key);
                        set_hover.set(current_tech.clone());
                    }
                >
                    <Icon icon=tech_icon width="4em" height="4em" style="margin: auto;" />
                </div>
            }
            .into_any()
        } else {
            view! {
                <div
                    class="content-center w-20 h-20 border bg-blue-500/20 border-blue-500/20 rounded-xs"
                    on:mouseenter=move |_| {
                        log!("{}", tech.get().key);
                        set_hover.set(tech.get());
                    }
                >
                    <Icon icon=tech_icon width="4em" height="4em" style="margin: auto;" />
                </div>
            }.into_any()
        }
    }
}
