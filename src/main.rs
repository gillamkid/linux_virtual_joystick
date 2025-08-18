use eframe::egui;
use eframe::egui::{ColorImage, TextureHandle};

mod controller;

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([860.0, 430.0]), // Set width and height
        ..Default::default()
    };

    eframe::run_native(
        "Virtual UXV SRoC",
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
    buttons: Box<[controller::Button]>,
    img: TextureHandle,
}

const PNG_DATA: &[u8] = include_bytes!("sroc.png");

impl UI {
    pub fn new(cc: &eframe::CreationContext<'_>, axes: Box<[controller::AnalogAxis]>, buttons: Box<[controller::Button]>) -> Self {
        // create img
        let dyn_image = image::load_from_memory(&PNG_DATA).expect("Failed to load image");
        let size = [dyn_image.width() as _, dyn_image.height() as _];
        let rgba = dyn_image.to_rgba8();
        let pixels = rgba.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        let img = cc.egui_ctx.load_texture("img", color_image, Default::default());

        Self { axes, buttons, img }
    }
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // picture of the controller
            ui.put(
                egui::Rect::from_min_size(egui::pos2(150.0, 40.0), self.img.size_vec2()),
                egui::Image::new(&self.img)
            );

            for axis in self.axes.iter_mut() {
                let pos = egui::pos2(axis.pos_x as f32, axis.pos_y as f32);

                let is_vertical_slider = axis.name().contains(" Y");

                let size = if is_vertical_slider {
                    egui::vec2(30.0, 200.0)
                } else {
                    egui::vec2(200.0, 30.0)
                };

                let rect = egui::Rect::from_min_size(pos, size);

                let mut slider = egui::Slider::new(&mut axis.new_value, -100..=100).show_value(false);

                if is_vertical_slider {
                    slider = slider.vertical();
                }

                let response = ui.put(rect, slider);

                // recenter sliders if the mouse press has been released
                if axis.new_value != 0 && !response.is_pointer_button_down_on()  {
                    axis.new_value = 0;
                }
            }

            // Your existing button checkboxes
            for btn in self.buttons.iter_mut() {
                let name = btn.name();
                let pos = egui::pos2(btn.pos_x as f32, btn.pos_y as f32);
                let rect = egui::Rect::from_min_size(pos, egui::vec2(150.0, 20.0));
                let checkbox = egui::Checkbox::new(&mut btn.new_value, name);
                ui.put(rect, checkbox);
            }
        });

        // Update values
        for axis in self.axes.iter_mut() {
            axis.new_value();
        }
        for btn in self.buttons.iter_mut() {
            btn.new_value();
        }
    }
}
