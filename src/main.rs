use eframe::egui::{Context, Ui, Vec2};
use eframe::{Frame, egui};
use rand::{RngCore, SeedableRng};
use rand_wyrand::WyRand;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::IntoStaticStr;

#[derive(PartialEq, IntoStaticStr, EnumIter, Clone, Copy)]
pub enum RenderMode {
    Normals,
    Raycast,
    Raytrace,
    Pathtracing,
}
struct App {
    samples: u32,
    elapsed: f64,
    render_mode: RenderMode,
    scene: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::right("right").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.samples, 1..=100).text("Samples"));
            ui.label(format!("Using Samples: {}", self.samples));
            for mode in RenderMode::iter() {
                ui.radio_value(
                    &mut self.render_mode,
                    mode,
                    <RenderMode as Into<&'static str>>::into(mode),
                );
            }
            egui::ComboBox::from_label("Scene")
                .selected_text(format!("{}", self.scene))
                .show_ui(ui, |ui| {
                    for i in 0..10 {
                        ui.selectable_value(&mut self.scene, i, format!("{}", i));
                    }
                });
            ui.label(format!("Rendering Time: {:.5}s", self.elapsed));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = get_max_quadratic_size(ui);
            let start_time = std::time::Instant::now();
            let mut pixels = render(
                [size.x as usize, size.y as usize],
                self.samples,
                self.render_mode,
            );

            ui.add(
                egui::Image::new(egui::load::SizedTexture::new(
                    &ctx.load_texture(
                        "raytraced",
                        egui::ColorImage::from_rgba_unmultiplied(
                            [size.x as usize, size.y as usize],
                            pixels.as_mut_slice(),
                        ),
                        Default::default(),
                    ),
                    size,
                ))
                .fit_to_exact_size(size),
            );
            self.elapsed = start_time.elapsed().as_secs_f64();
        });
    }
}

pub fn get_max_quadratic_size(ui: &mut Ui) -> Vec2 {
    let size = ui.available_size();
    let min_size = if size.x < size.y { size.x } else { size.y };
    let size = Vec2::from([min_size, min_size]);
    size
}

fn render(size: [usize; 2], samples: u32, _render_mode: RenderMode) -> Vec<u8> {
    let mut rng = WyRand::seed_from_u64(samples as u64);
    let mut pixels = vec![0u8; size[0] * size[1] * 4];
    rng.fill_bytes(&mut pixels);
    pixels
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rrrrrrr",
        options,
        Box::new(|_cc| {
            Ok(Box::new(App {
                samples: 1,
                elapsed: 0.0,
                render_mode: RenderMode::Raycast,
                scene: 1,
            }))
        }),
    )
}
