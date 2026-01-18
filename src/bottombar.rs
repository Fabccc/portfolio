use leptos::prelude::*;
use leptos_use::use_interval_fn;use chrono::{DateTime, Datelike, Local};

const DATE_FORMAT: &str = "%d/%m/%Y | %H:%M:%S";
const DATE_BIRTH: &str = "22-07-2000 21:19:15.000 +2000";

#[component]
pub fn Bottombar() -> impl IntoView {

    fn format_current_time() -> String {
        let local_time = Local::now();
        format!("{}", local_time.format(DATE_FORMAT))
    }
    fn format_cli_version() -> String {
        let birth_date = DateTime::parse_from_str(DATE_BIRTH, "%d-%m-%Y %H:%M:%S%.3f %z").unwrap();
        let now_date = Local::now();
        let between = now_date.naive_local() - birth_date.naive_local();
        let years_between = (between.num_days() as f32).div_euclid(365.25);
        let remaining_month = (birth_date.month()+now_date.month())%12-1;
        format!("v1.{}y.{}m", years_between, remaining_month)
    }

    let (current_time, set_current_time) = signal(format_current_time());
    let (cli_version, set_cli_version) = signal(format_cli_version());

    use_interval_fn(
        move || {
            set_current_time.set(format_current_time());
            set_cli_version.set(format_cli_version());
        },
        500,
    );

    view!{
        <div class="flex flex-row space-x-4 justify-between w-full items-center">
            <div class="flex flex-row gap-x-4 items-center">
                <span>{current_time}</span>
            </div>
            <div>
                <span title="Oui, c'est écrit en Rust!">"FABIENCAYRE-CLI-RS "{cli_version}</span>
            </div>
        </div>
    }
}