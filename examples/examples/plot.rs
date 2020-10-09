use imgui::*;
use plotters::prelude::*;
use plotters_imgui::ImguiBackend;

mod support;

fn main() {
    let mut system = support::init(file!());
    let style = system.imgui.style_mut().use_light_colors();
    style.window_rounding = 0.0;
    style.scrollbar_rounding = 0.0;

    let mut shift: f64 = 0.0;

    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello Plotters"))
            .position([270.0, 50.0], Condition::FirstUseEver)
            .size([200.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Shift wave by sliding here:"));

                Slider::new(im_str!("Shift"))
                    .range(0.0..=2.0)
                    .display_format(im_str!("%.2f"))
                    .build(ui, &mut shift);

                let dl = ui.get_background_draw_list();
                let root = ImguiBackend::new(&ui, &dl, (400, 300)).into_drawing_area();

                let mut chart = ChartBuilder::on(&root)
                    .margin(20)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(0.0..6.0, -1.5..1.5).unwrap();

                chart.configure_mesh().draw().unwrap();

                chart
                    .draw_series(LineSeries::new(
                        (0..=600).map(|x| x as f64 / 100.0).map(|x| (x, (x - shift).sin())),
                        &BLACK.mix(0.5),
                    )).unwrap();


            });
    });
}
