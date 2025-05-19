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
use ui::en_us::EnUsInterface;
use ui::zh_cn::ZhCnInterface;

fn main() {
    // Choose language
    let lang = "zh_cn"; 

    let ui: Box<dyn ui::Interface> = match lang {
        "zh_cn" => Box::new(ZhCnInterface::new()),
        "en_us" => Box::new(EnUsInterface::new()),
        _ => panic!("Unsupported language"),
    };

    // Init
    ui.init();
    // Core
    ui.start();
    // Exit
    ui.exit();
}
