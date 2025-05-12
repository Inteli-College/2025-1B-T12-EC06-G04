mod image_processor;
mod ui;

use dioxus_desktop::Config;

fn main() {
    dioxus_desktop::launch_cfg(ui::app, Config::default());
}
