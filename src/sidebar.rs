
use leptos::prelude::*;


#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="flex flex-col border-l w-64 border-l-zinc-700 pt-4 pl-4">
            <div class="flex flex-col">
                <table>
                    <tbody>
                        <tr>
                            <td class="text-green-500">64.8</td>
                            <td>/</td>
                            <td>148Gi</td>
                        </tr>
                        <tr>
                            <td class="text-yellow-500">28.7</td>
                            <td>/</td>
                            <td>48Cpu</td>
                        </tr>
                        <tr>
                            <td class="text-lime-500">74</td>
                            <td>/</td>
                            <td>160pods</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div>
            </div>
        </div>
    }
}
