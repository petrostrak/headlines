use std::fmt::format;

use eframe::{egui::CentralPanel, epi::App, run_native, NativeOptions};

struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a),
        });
        Headlines {
            articles: Vec::from_iter(iter),
        }
    }
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            for a in &self.articles {
                ui.label(&a.title);
                ui.label(&a.desc);
                ui.label(&a.url);
            }
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app = Headlines::new();
    let win_opts = NativeOptions::default();
    run_native(Box::new(app), win_opts);
}
