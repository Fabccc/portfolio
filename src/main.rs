use leptos::prelude::*;
use reactive_stores::Store;

mod bottombar;
mod highlighter;
mod icon;
mod maincontent;
mod menubar;
mod sidebar;
mod state;

use crate::{
    bottombar::Bottombar,
    maincontent::MainContent,
    menubar::Menubar,
    sidebar::Sidebar,
    state::{cluster::ClusterState, editor::EditorState},
};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(ClusterState::default()));
    provide_context(Store::new(EditorState::default()));

    view! {
        <div class="flex flex-col h-screen w-full m-0">
            <div class="flex flex-row border-b  border-b-zinc-700">
                <Menubar />
            </div>
            <div class="flex flex-col min-h-0 h-full grow">
                <div class=" flex flex-row min-h-0 h-full grow">
                    <MainContent />
                    <Sidebar />
                </div>
                <div class="basis-0 border-t py-3 border-t-zinc-700">
                    <Bottombar/>
                </div>
            </div>
        </div>
    }
}
