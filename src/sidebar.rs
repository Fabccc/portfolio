use leptos::prelude::*;
use leptos_use::use_interval_fn;
use reactive_stores::Store;

use crate::state::{cluster::{ClusterState, ClusterStateView}, technology::Technologies};

#[component]
pub fn Sidebar() -> impl IntoView {

    let cluster_state = expect_context::<Store<ClusterState>>();

    use_interval_fn(
        move || {
            cluster_state.write().next_tick();
        },
        2500,
    );

    view! {
        <div class="flex flex-col w-84 border-l border-l-zinc-700">
            <div class="flex flex-col">
                <h1 class="text-xl text-center">"Usage"</h1>
                <div class="grid grid-cols-1 grid-rows-3 justify-center-safe">
                    <ClusterStateView cluster_state=cluster_state />
                </div>
            </div>
            <Technologies/>
        </div>
    }
}

