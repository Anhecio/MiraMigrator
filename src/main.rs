// MiraMigrator Logo
const LOGO: &str = r"
• ▌ ▄ ·. ▪  ▄▄▄   ▄▄▄· • ▌ ▄ ·. ▪   ▄▄ • ▄▄▄   ▄▄▄· ▄▄▄▄▄      ▄▄▄  
·██ ▐███▪██ ▀▄ █·▐█ ▀█ ·██ ▐███▪██ ▐█ ▀ ▪▀▄ █·▐█ ▀█ •██  ▪     ▀▄ █·
▐█ ▌▐▌▐█·▐█·▐▀▀▄ ▄█▀▀█ ▐█ ▌▐▌▐█·▐█·▄█ ▀█▄▐▀▀▄ ▄█▀▀█  ▐█.▪ ▄█▀▄ ▐▀▀▄ 
██ ██▌▐█▌▐█▌▐█•█▌▐█ ▪▐▌██ ██▌▐█▌▐█▌▐█▄▪▐█▐█•█▌▐█ ▪▐▌ ▐█▌·▐█▌.▐▌▐█•█▌
▀▀  █▪▀▀▀▀▀▀.▀  ▀ ▀  ▀ ▀▀  █▪▀▀▀▀▀▀·▀▀▀▀ .▀  ▀ ▀  ▀  ▀▀▀  ▀█▄▀▪.▀  ▀
    ";

// MiraMigrator Version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// mod
mod api;
mod scan;
mod ui;
mod utils;

// use
use ui::zh_cn::ZhCnInterface;

fn main() {
    let ui = ZhCnInterface::new();
    // Init
    ui.init();
    // Core
    ui.start();
    // Exit
    ui.exit();
}
