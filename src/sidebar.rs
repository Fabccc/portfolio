
use leptos::prelude::*;
use leptos_use::use_interval_fn;
use reactive_stores::Store;

use crate::state::{State, StateView};


#[component]
pub fn Sidebar() -> impl IntoView {

    const CONTROL_NODE_COUNT: i32 = 3;
    const DATA_NODE_COUNT: i32 = 6;

    let state = expect_context::<Store<State>>();

    use_interval_fn(move || {
        state.write().next_tick();
    }, 1000);

    let control_nodes = (0..CONTROL_NODE_COUNT)
        .map(|n| view!{
            <tr>
                <td>{format!("Control-0{}",n+1)}</td>
                <td>{format!("192.168.1.{}",n+2)}</td>
                <td>"13/20"</td>
            </tr>
        })
        .collect::<Vec<_>>();

        let data_nodes = (0..DATA_NODE_COUNT)
        .map(|n| view!{
            <tr>
                <td>{format!("Data-0{}",n+1)}</td>
                <td>{format!("192.168.2.{}",n+1)}</td>
                <td>"13/20"</td>
            </tr>
        })
        .collect::<Vec<_>>();

    view! {
        <div class="flex flex-col border-l w-80 border-l-zinc-700 pt-4 pl-4">
            <div class="flex flex-col">
                <h1 class="text-center text-xl">"Usage"</h1>
                <div class="grid grid-rows-3 grid-cols-1 justify-center-safe">
                    <StateView store=state/>
                </div>
            </div>
            <div class="pt-8 text-xs">
                <h1 class="text-center text-xl">"Map"</h1>
                <table class="table-auto w-full">
                    <thead>
                        <tr>
                            <th scope="col">"Node"</th>
                            <th scope="col">"IP"</th>
                            <th scope="col">"Pods"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {control_nodes}
                        {data_nodes}
                    </tbody>
                </table>
            </div>
            <div class="pt-8 text-xs">
                <h1 class="text-center text-xl">"K8S Ressources"</h1>
            </div>
        </div>
    }
}
