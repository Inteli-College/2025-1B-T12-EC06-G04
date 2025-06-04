use dioxus::prelude::*;
use crate::use_router;
use dioxus_router::prelude::Router;

#[component]
pub fn ValidationScreen() -> Element {
    let router = use_router();

    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 {
                "Valide as fissuras detectadas"
            }
            div {
                class: "flex flex-col items-center justify-center",
                span {
                    "Verifique com atenção e valide as fissuras detectadas e clique nas que não estão corretas"
                }
                div {
                    class: "flex flex-col items-center justify-center",
                    span {
                        "Fissura"
                    }
                    span {
                        "Fissura"
                    }
                    
                }
            }
        }
    }
}