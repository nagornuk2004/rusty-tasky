mod app;

use relm4::RelmApp;

fn main() {
    let rusty_tasky = RelmApp::new("io.github.nagornuk2004.rusty-tasky");
    rusty_tasky.run::<app::AppModel>(());
}
