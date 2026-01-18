use leptos::prelude::*;

mod bottombar;
mod highlighter;
mod icon;
mod maincontent;
mod menubar;
mod sidebar;

use crate::{bottombar::Bottombar, maincontent::MainContent, menubar::Menubar, sidebar::Sidebar};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col h-full w-full p-3 m-0">
            <div class="flex flex-row border-b  border-b-zinc-700">
                <Menubar />
            </div>
            <div class="flex flex-col h-full">
                <div class="h-full flex flex-row">
                    <MainContent />
                    <Sidebar />
                </div>
                <div class="basis-0 border-t pt-3 border-t-zinc-700">
                    <Bottombar/>
                </div>
            </div>
        </div>
    }
}
