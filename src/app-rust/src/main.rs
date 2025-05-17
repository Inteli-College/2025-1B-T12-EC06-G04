use dioxus::{desktop::Config, desktop::WindowBuilder, prelude::*};
use dioxus_router::prelude::*;

mod folders;
use folders::Folders;

mod report;
use report::ReportView;

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_resizable(true)))
        .launch(|| {
        rsx! { Router::<Route> {} }
    });
}

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[route("/")]
    Folders {},

    #[route("/report")]
    ReportView {}
}
