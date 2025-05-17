
use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod home;
use home::Home;
mod ui;
mod image_processor;

fn main() {
    dioxus::launch(|| {
        rsx! { Router::<Route> {} }
    });
}

#[allow(non_snake_case)]
pub fn Home() -> Element {
    rsx! {
        h1 { "Home Page" }
        Link { to: "/another-page", "Vá para outra paǵina" }
    }
}

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[route("/")]
    Home {},
}