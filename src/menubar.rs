use leptos::prelude::*;
use leptos_use::use_media_query;

use crate::icon::{Icon, IconType};

#[component]
pub fn Menubar() -> impl IntoView {

    let text = r#".----..--.  .----. .-..----..-. .-.    .---.   .--..-.  .-..----. .----.
| {_ / {} \ | {}  }| || {_  |  `| |   /  ___} / {} \\ \/ / | {}  }| {_  
| | /  /\  \| {}  }| || {__ | |\  |   \     }/  /\  \}  {  | .-. \| {__ 
`-' `-'  `-'`----' `-'`----'`-' `-'    `---' `-'  `-'`--'  `-' `-'`----'"#;

    let text_sm = r#".----..--.  .----. .-..----..-. .-.    .---. 
| {_ / {} \ | {}  }| || {_  |  `| |   /  ___}
| | /  /\  \| {}  }| || {__ | |\  |   \     }
`-' `-'  `-'`----' `-'`----'`-' `-'    `---' "#;

    let text_xs = r#".----..--.  .----. 
| {_ / {} \ | {}  }
| | /  /\  \| {}  }
`-' `-'  `-'`----' "#;

    let is_tablet = use_media_query("(max-width: 980px)");
    let is_phone = use_media_query("(max-width: 680px)");

    view! {
        <div class="flex flex-row justify-between w-full items-center">
            <div class="flex flex-row gap-x-8 w-full items-center">
                <MenuItem title="Fichier"/>
                <MenuItem title="CV" leading_icon=IconType::DOWNLOAD/>
            </div>
            <div>
                <pre class="leading-5">
                    {move || if is_phone.get() {
                        text_xs
                    } else if is_tablet.get() {
                        text_sm
                    } else {
                        text
                    }}
                </pre>
            </div>
        </div>
    }
}

#[component]
pub fn MenuItem(
    title: &'static str,
    #[prop(optional)] leading_icon: Option<IconType>,
    #[prop(optional)] tailing_icon: Option<IconType>,
) -> impl IntoView {
    view! {
        <span class="text-4xl hover:bg-blue-700 hover:cursor-pointer">

            {leading_icon.map(move |icont| {
                view! {<Icon icon_type=icont/>}
            })}
            {title}
            {tailing_icon.map(move |icont| {
                view! {<Icon icon_type=icont/>}
            })}
        </span>
    }
}
