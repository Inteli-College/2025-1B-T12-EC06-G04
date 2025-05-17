use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod report;
use report::ReportView;

fn main() {
    dioxus::launch(|| {
        rsx! { Router::<Route> {} }
    });
}

#[allow(non_snake_case)]
pub fn Home() -> Element {
    rsx! {
        h1 { "Home Page" }
        Link { to: "/report", "Vá para outra paǵina" }
    }
}

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/report")]
    ReportView {},
}
