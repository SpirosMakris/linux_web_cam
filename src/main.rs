use eframe::egui::{self, Color32, ColorImage, TextureOptions};
use std::{fs::OpenOptions, io::Write};

mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

struct WebcamUi {
    v4l2_device: v4l2::V4l2VideoDevice,
}

impl WebcamUi {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let v4l2_device = v4l2::V4l2VideoDevice::new(&DEVICE_NAME);

        WebcamUi { v4l2_device }
    }
}

impl eframe::App for WebcamUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let v4l2_frame = self.v4l2_device.get_frame();

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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image((texture.id(), texture.size_vec2()));
            ui.heading("Hello World!");
        });
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

fn __main() {
    let device = v4l2::V4l2VideoDevice::new(&DEVICE_NAME);

    let frame = device.get_frame();

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test_2.yuv")
        .unwrap();
    output.write_all(frame.data()).unwrap();
}
