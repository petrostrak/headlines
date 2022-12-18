use std::borrow::Cow;

use eframe::{
    egui::{CentralPanel, ScrollArea, Vec2},
    epi::App,
    run_native, NativeOptions,
};
use headlines::Headlines;

mod headlines;

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            })
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app = Headlines::new();
    let mut win_opts = NativeOptions::default();
    win_opts.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_opts);
}
