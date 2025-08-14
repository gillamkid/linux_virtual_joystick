use eframe::egui;
use eframe::egui::{ColorImage, TextureHandle};

mod controller;

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 500.0]), // Set width and height
        ..Default::default()
    };

    eframe::run_native(
        "Linux Virtual Joystick",
        native_options,
        Box::new(|cc| {
            let (axes, buttons) = controller::build_uninput().unwrap();
            Box::new(UI::new(cc, axes, buttons))
        }),
    ).unwrap();
    Ok(())
}

struct UI {
    axes: Box<[controller::AnalogAxis]>,
    button: Box<[controller::Button]>,
    texture: TextureHandle,
}

impl UI {
    pub fn new(cc: &eframe::CreationContext<'_>, axes: Box<[controller::AnalogAxis]>, button: Box<[controller::Button]>) -> Self {
        // Load image from file
        let image_bytes = std::fs::read("src/sroc.png").expect("Failed to read image");
        let dyn_image = image::load_from_memory(&image_bytes).expect("Failed to load image");

        // Convert to egui image format
        let size = [dyn_image.width() as _, dyn_image.height() as _];
        let rgba = dyn_image.to_rgba8();
        let pixels = rgba.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        // Create a texture
        let texture = cc.egui_ctx.load_texture("controller_image", color_image, Default::default());

        Self { axes, button, texture }
    }
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Draw picture at top
            let scale = 1.0;
            let size = self.texture.size_vec2() * scale;
            let pos = egui::pos2(210.0, 0.0);
                let rect = egui::Rect::from_min_size(pos, size);
            ui.put(
                rect,
                egui::Image::new(&self.texture)
                    .fit_to_exact_size(size)
            );

            // Your existing axis sliders
            for axis in self.axes.iter_mut() {
                let name = axis.name();
                let pos = egui::pos2(axis.pos_x as f32, axis.pos_y as f32);
                let rect = egui::Rect::from_min_size(pos, egui::vec2(200.0, 30.0));
                let slider = egui::Slider::new(&mut axis.new_value, -100..=100).text(name);
                ui.put(rect, slider);
            }

            // Your existing button checkboxes
            for button in self.button.iter_mut() {
                let name = button.name();
                let pos = egui::pos2(button.pos_x as f32, button.pos_y as f32);
                let rect = egui::Rect::from_min_size(pos, egui::vec2(150.0, 20.0));
                let checkbox = egui::Checkbox::new(&mut button.new_value, name);
                ui.put(rect, checkbox);
            }
        });

        // Update values
        for axis in self.axes.iter_mut() {
            axis.new_value();
        }
        for button in self.button.iter_mut() {
            button.new_value();
        }
    }
}
