use leptos::prelude::*;

const USE_CSS_CLASS: bool = true;

#[derive(PartialEq)]
pub enum IconType {
    DOWNLOAD,
}

impl IconType {
    fn class_name(&self) -> &'static str {
        match self {
            Self::DOWNLOAD => "nf nf-fa-heart",
        }
    }
}

impl std::fmt::Display for IconType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DOWNLOAD => "󱑢",
            }
        )
    }
}

#[component]
pub fn Icon(#[prop()] icon_type: IconType) -> impl IntoView {
    if USE_CSS_CLASS {
        view! {
            <i class=format!("{}", icon_type.class_name())></i>
        }
        .into_any()
    } else {
        view! {
            <span>{format!("{}", icon_type)}</span>
        }
        .into_any()
    }
}
