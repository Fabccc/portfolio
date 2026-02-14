use leptos::prelude::*;
use leptos_use::use_media_query;

use crate::icon::{Icon, IconType};

#[component]
pub fn Menubar() -> impl IntoView {
    let text = r#"  _                     _          _   _ 
 |_ _. |_  o  _  ._    /   /\ \_/ |_) |_ 
 | (_| |_) | (/_ | |   \_ /--\ |  | \ |_ 
                                         "#;

    let text_sm = r#"  _                     _   
 |_ _. |_  o  _  ._    /    
 | (_| |_) | (/_ | |   \_ o 
                            "#;

    let text_xs = r#"  _        
 |_ _. |_  
 | (_| |_) 
           "#;

    let is_tablet = use_media_query("(max-width: 980px)");
    let is_phone = use_media_query("(max-width: 680px)");

    view! {
        <div class="flex flex-row justify-between w-full items-center px-3">
            <div class="flex flex-row gap-x-8 w-full items-center">
                <MenuItem title="CV" leading_icon=IconType::Download url="/assets/CV.pdf"/>
                <MenuItem title="LinkedIN" url="https://www.linkedin.com/in/fabien-cayre/"/>
                <MenuItem title="Github" url="https://github.com/Fabccc"/>
            </div>
            <div>
                <pre class="leading-4">
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
    #[prop(optional)] url: Option<&'static str>
) -> impl IntoView {
    let content = 
    view!{
        {leading_icon.map(move |icont| {
                view! {<Icon icon_type=icont/>}
        })}
        {title}
        {tailing_icon.map(move |icont| {
            view! {<Icon icon_type=icont/>}
        })}
    };
    if let Some(urllink) = url {
        view!{
            <a href=urllink rel="external" class="text-2xl hover:bg-green-500" target="_blank">
                {content}
            </a>
        }.into_any()
    }else{
        view!{
            <span class="text-2xl hover:bg-green-500 hover:cursor-pointer">
                {content}
            </span>
        }.into_any()
    }
    
    
}
