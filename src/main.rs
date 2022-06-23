use crossbeam::channel::unbounded;

use eframe::{run_native, NativeOptions};

use egui::{CentralPanel, ScrollArea, SidePanel, TextEdit, TopBottomPanel, Visuals, Window};
use tracing::subscriber::set_global_default;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
use widgets::candles_graph::graph::Graph;
use widgets::symbols::Symbols;

mod network;
mod sources;
mod widgets;
use tokio;

struct TemplateApp {
    candle_plot: Graph,
    symbols: Symbols,
    debug_visible: bool,
    dark_mode: bool,
}

impl TemplateApp {
    fn new(_ctx: &eframe::CreationContext<'_>) -> Self {
        let (s, r) = unbounded();
        let plot = Graph::new(r);
        Self {
            dark_mode: true,
            candle_plot: plot,
            symbols: Symbols::new(s),
            debug_visible: false,
        }
    }

    fn render_center_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut self.candle_plot);
        });
    }

    fn render_top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui
                .button({
                    match self.dark_mode {
                        true => "🔆",
                        false => "🌙",
                    }
                })
                .clicked()
            {
                self.dark_mode = !self.dark_mode
            }
        });
    }

    fn render_bottom_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::bottom("bot panel").show(ctx, |ui| {
            if ui.button("debug").clicked() {
                self.debug_visible = !self.debug_visible;
            }
        });
    }

    fn render_side_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("side_panel").show(ctx, |ui| ui.add(&mut self.symbols));
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dark_mode {
            ctx.set_visuals(Visuals::dark())
        } else {
            ctx.set_visuals(Visuals::light())
        }

        if self.debug_visible {
            Window::new("debug").show(ctx, |ui| {
                ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let mut text = "text";
                        TextEdit::multiline(&mut text).desired_rows(10).show(ui);
                    });
            });
        }

        self.render_top_panel(ctx);
        self.render_bottom_panel(ctx);
        self.render_side_panel(ctx);
        self.render_center_panel(ctx);
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        info!("called save")
    }
}

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    set_global_default(subscriber).expect("failed to set default tracing subscriber");
}

#[tokio::main]
async fn main() {
    init_tracing();

    run_native(
        "hedgegraph",
        NativeOptions::default(),
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    );
}
