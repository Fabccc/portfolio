use leptos::prelude::*;
use leptos_use::use_interval_fn;
use reactive_stores::Store;

use crate::state::{cluster::{ClusterState, ClusterStateView}, technology::{Technologies, TechnologyState, TechnologyStateStoreFields}};

#[component]
pub fn Sidebar() -> impl IntoView {
    const CONTROL_NODE_COUNT: i32 = 3;
    const DATA_NODE_COUNT: i32 = 6;

    let cluster_state = expect_context::<Store<ClusterState>>();

    use_interval_fn(
        move || {
            cluster_state.write().next_tick();
        },
        2500,
    );

    let control_nodes = (0..CONTROL_NODE_COUNT)
        .map(|n| {
            view! {
                <tr>
                    <td>{format!("Control-0{}", n + 1)}</td>
                    <td>{format!("192.168.1.{}", n + 2)}</td>
                    <td>"13/20"</td>
                </tr>*
            }
        })
        .collect::<Vec<_>>();

    let data_nodes = (0..DATA_NODE_COUNT)
        .map(|n| {
            view! {
                <tr>
                    <td>{format!("Data-0{}", n + 1)}</td>
                    <td>{format!("192.168.2.{}", n + 1)}</td>
                    <td>"13/20"</td>
                </tr>
            }
        })
        .collect::<Vec<_>>();
    

    view! {
        <div class="flex flex-col w-80 border-l border-l-zinc-700">
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

