mod app;
mod fonts;
mod board; 

use app::MyApp;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();

    eframe::run_native(
        "我的扫雷窗口",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
