use eframe::egui::{self, Color32, ColorImage, TextureHandle, TextureOptions};
use std::sync::mpsc::{self, Receiver, Sender};

mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

struct WebcamUi {
    rx: Receiver<TextureHandle>,
    last_texture: Option<TextureHandle>,
}

impl WebcamUi {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let v4l2_device = v4l2::V4l2VideoDevice::new(&DEVICE_NAME);
        let (tx, rx) = mpsc::channel();
        let ctx = cc.egui_ctx.clone();

        std::thread::spawn(move || feed_gui(ctx, v4l2_device, tx));

        WebcamUi {
            rx,
            last_texture: None,
        }
    }
}

impl eframe::App for WebcamUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(v) = self.rx.try_recv() {
            self.last_texture = Some(v);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.last_texture {
                ui.image((texture.id(), texture.size_vec2()));
            }
            ui.heading("Hello World!");
        });
    }
}

fn feed_gui(ctx: egui::Context, v4l2_device: v4l2::V4l2VideoDevice, tx: Sender<TextureHandle>) {
    loop {
        let v4l2_frame = v4l2_device.get_frame();

        // YUYV encoded
        let data = v4l2_frame.data();

        // Just get black & white data
        let color_data: Vec<Color32> = data
            .iter()
            // Discard U and V data
            .step_by(2)
            .map(|y| egui::Color32::from_gray(*y))
            .collect();

        let image = ColorImage {
            size: [v4l2_frame.width(), v4l2_frame.height()],
            pixels: color_data,
        };

        let texture = ctx.load_texture("our only image", image, TextureOptions::LINEAR);
        tx.send(texture).unwrap();
        ctx.request_repaint();
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Web Cam",
        native_options,
        Box::new(|cc| Box::new(WebcamUi::new(cc))),
    )
    .unwrap();
}
