use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { 
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }

        div {
            class: "p-4 bg-green-500 text-white text-xl rounded",
            "Tailwind está funcionando!"
        }
    
    }
}