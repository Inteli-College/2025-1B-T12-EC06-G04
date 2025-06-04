use dioxus::prelude::*;
use dioxus_desktop::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            style: "text-align: center; padding: 2rem;",
            h1 { "Minha Página em Rust (Dioxus)" }
            img {
                src: "https://via.placeholder.com/300",
                alt: "Imagem de exemplo",
                style: "margin-top: 20px;"
            }
        }
    ))
}
