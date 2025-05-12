mod ui;
mod image_processor;

use dioxus_desktop::Config;

fn main() {
    dioxus_desktop::launch_cfg(ui::app, Config::default());
}
