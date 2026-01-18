use leptos::prelude::*;
use num::clamp;
use reactive_stores::Store;

const MAX_CPU_USAGE: f32 = 46.0;
const MIN_CPU_USAGE: f32 = 6.0;
const MAX_MEMORY_USAGE: f32 = 186.0;
const MIN_MEMORY_USAGE: f32 = 28.0;
const MAX_PODS_COUNT: i32 = 148;
const MIN_PODS_COUNT: i32 = 48;
const RANDOM_SEED: i32 = 220700;

#[derive(Clone, Debug, Store)]
pub struct ClusterState {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub pods_count: i32,
}

#[component]
pub fn ClusterStateView(#[prop(into)] cluster_state: Store<ClusterState>) -> impl IntoView {
    view! {
        <div class="w-full flex flex-row justify-around">
            <span
                class=move || tailwind_classname(cluster_state.memory_usage().get(), MAX_MEMORY_USAGE)
            >{move || format!("{:.1} Gi", cluster_state.memory_usage().get())}</span>
            <span>"/"</span>
            <span>"196Gi"</span>
        </div>
        <div class="w-full flex flex-row justify-around">
            <span
                class=move || tailwind_classname(cluster_state.cpu_usage().get(), MAX_CPU_USAGE)
            >{move || format!("{:.1} cpu", cluster_state.cpu_usage().get())}</span>
            <span>"/"</span>
            <span>"48 cpu"</span>
        </div>
        <div class="w-full flex flex-row justify-around">
            <span
                class=move || tailwind_classname(cluster_state.pods_count().get() as f32, MAX_PODS_COUNT as f32)
            >{move || format!("{} pods", cluster_state.pods_count().get())}</span>
            <span>"/"</span>
            <span>"160pods"</span>
        </div>
    }
}

impl Default for ClusterState {
    fn default() -> Self {
        fastrand::seed(RANDOM_SEED as u64);
        let cpu = fastrand_f32_range(MIN_CPU_USAGE, MAX_CPU_USAGE);
        let memory = fastrand_f32_range(MIN_MEMORY_USAGE, MAX_MEMORY_USAGE);
        let pods = fastrand::i32(MIN_PODS_COUNT..MAX_PODS_COUNT);
        Self {
            cpu_usage: cpu,
            memory_usage: memory,
            pods_count: pods,
        }
    }
}

impl ClusterState {
    pub fn next_tick(&mut self) {
        let cpu_new_tick = fastrand::f32();
        // cpu_new_tick < 1/3 -> down
        // cpu_new_tick > 1/3 < 2/3 -> nothing
        // cpu_new_tick > 2/3 -> up
        self.cpu_usage = if cpu_new_tick < 1.0 / 3.0 {
            clamp(
                self.cpu_usage - fastrand_f32_range(1.0, 2.0),
                MIN_CPU_USAGE,
                MAX_CPU_USAGE,
            )
        } else if cpu_new_tick > 2.0 / 3.0 {
            clamp(
                self.cpu_usage + fastrand_f32_range(1.0, 2.0),
                MIN_CPU_USAGE,
                MAX_CPU_USAGE,
            )
        } else {
            self.cpu_usage
        };

        let memory_new_tick = fastrand::f32();
        // memory_new_tick < 1/3 -> down
        // memory_new_tick > 1/3 < 2/3 -> nothing
        // memory_new_tick > 2/3 -> up
        self.memory_usage = if memory_new_tick < 1.0 / 3.0 {
            clamp(
                self.memory_usage - fastrand_f32_range(1.0, 2.0),
                MIN_MEMORY_USAGE,
                MAX_MEMORY_USAGE,
            )
        } else if memory_new_tick > 2.0 / 3.0 {
            clamp(
                self.memory_usage + fastrand_f32_range(1.0, 2.0),
                MIN_MEMORY_USAGE,
                MAX_MEMORY_USAGE,
            )
        } else {
            self.memory_usage
        };

        let pods_new_tick = fastrand::f32();
        // pods_new_tick < 1/3 -> down
        // pods_new_tick > 1/3 < 2/3 -> nothing
        // pods_new_tick > 2/3 -> up
        self.pods_count = if pods_new_tick < 1.0 / 3.0 {
            clamp(
                self.pods_count - fastrand::i32(1..5),
                MIN_PODS_COUNT,
                MAX_PODS_COUNT,
            )
        } else if pods_new_tick > 2.0 / 3.0 {
            clamp(
                self.pods_count + fastrand::i32(1..5),
                MIN_PODS_COUNT,
                MAX_PODS_COUNT,
            )
        } else {
            self.pods_count
        };
    }
}

// MARK: helper

fn fastrand_f32_range(min: f32, max: f32) -> f32 {
    (fastrand::f32() * (max - min)) + min
}

fn tailwind_classname(val: f32, max_val: f32) -> &'static str {
    if val < max_val * 0.45 {
        "text-green-500"
    } else if val < max_val * 0.55 {
        "text-lime-500"
    } else if val < max_val * 0.65 {
        "text-yellow-500"
    } else if val < max_val * 0.75 {
        "text-orange-500"
    } else if val < max_val * 0.85 {
        "text-red-500"
    } else {
        "text-red-700"
    }
}