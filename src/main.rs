use eframe::egui::{self, Color32, ColorImage, TextureHandle, TextureOptions};
use std::{
    iter::repeat,
    sync::mpsc::{self, Receiver, Sender},
};

mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

struct WebcamUi {
    frame_rx: Receiver<TextureHandle>,
    ui_action_tx: Sender<UiAction>,
    last_texture: Option<TextureHandle>,
    selected_size: usize,
    available_frame_sizes: Vec<(u32, u32)>,
}

impl WebcamUi {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let v4l2_device = v4l2::V4l2VideoDevice::new(&DEVICE_NAME);
        v4l2_device.print_formats();

        let available_frame_sizes = v4l2_device.get_frame_sizes();
        println!("Available frame sizes: {:?}", available_frame_sizes);

        // v4l2_device.set_frame_size(1);

        let (frame_tx, frame_rx) = mpsc::channel();
        let (ui_action_tx, ui_action_rx) = mpsc::channel();

        let ctx = cc.egui_ctx.clone();

        std::thread::spawn(move || feed_gui(ctx, v4l2_device, frame_tx, ui_action_rx));

        WebcamUi {
            frame_rx,
            ui_action_tx,
            // @FIXME get correct initial size
            selected_size: 0,
            available_frame_sizes,
            last_texture: None,
        }
    }
}

fn size_to_str(size: &(u32, u32)) -> String {
    format!("{}x{}", size.0, size.1)
}

impl eframe::App for WebcamUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(v) = self.frame_rx.try_recv() {
            self.last_texture = Some(v);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let prev_selected_size = self.selected_size;
            egui::ComboBox::from_label("Frame sizes")
                .selected_text(format!(
                    "{:?}",
                    self.available_frame_sizes[self.selected_size]
                ))
                .show_ui(ui, |ui| {
                    for i in 0..self.available_frame_sizes.len() {
                        if ui
                            .selectable_value(
                                &mut self.selected_size,
                                i,
                                size_to_str(&self.available_frame_sizes[i]),
                            )
                            .clicked()
                        {
                            self.selected_size = i;
                        };
                    }
                });

            if self.selected_size != prev_selected_size {
                self.ui_action_tx
                    .send(UiAction::ChangeSize(self.selected_size))
                    .unwrap();
            }

            if let Some(texture) = &self.last_texture {
                ui.image((texture.id(), texture.size_vec2()));
            }

            ui.heading("Hello World!");
        });
    }
}

pub enum UiAction {
    ChangeSize(usize),
}

fn feed_gui(
    ctx: egui::Context,
    mut v4l2_device: v4l2::V4l2VideoDevice,
    tx: Sender<TextureHandle>,
    rx: Receiver<UiAction>,
) {
    loop {
        if let Ok(ui_action) = rx.try_recv() {
            match ui_action {
                UiAction::ChangeSize(idx) => {
                    println!("Trying to change size to index {idx}");
                    v4l2_device.set_frame_size(idx);
                }
            }
        }

        let v4l2_frame = v4l2_device.get_frame();

        // YUYV encoded
        let data = v4l2_frame.data();

        let ys = data.iter().step_by(2);
        let us = data
            .iter()
            .skip(1)
            .step_by(4)
            .flat_map(|u| repeat(u).take(2));
        let vs = data
            .iter()
            .skip(3)
            .step_by(4)
            .flat_map(|u| repeat(u).take(2));

        let color_data: Vec<Color32> = ys
            .zip(us)
            .zip(vs)
            .map(|((y, u), v)| {
                let y = *y as f32 - 16.;
                let u = *u as f32 - 128.;
                let v = *v as f32 - 128.;

                let r = 1.164 * y + 1.596 * v;
                let g = 1.164 * y - 0.392 * u - 0.813 * v;
                let b = 1.164 * y + 2.017 * u;

                egui::Color32::from_rgb(r as u8, g as u8, b as u8)
            })
            .collect();

        // Just get black & white data
        // let color_data: Vec<Color32> = data
        //     .iter()
        //     // Discard U and V data
        //     .step_by(2)
        //     .map(|y| egui::Color32::from_gray(*y))
        //     .collect();

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
