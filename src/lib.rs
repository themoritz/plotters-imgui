use earcutr::earcut;
use imgui::{im_str, Ui, WindowDrawList};
use plotters_backend::{
    text_anchor::{HPos, VPos},
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct ImguiError;

impl std::error::Error for ImguiError {}

impl Display for ImguiError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

pub struct ImguiBackend<'a> {
    ui: &'a Ui<'a>,
    draw_list: &'a WindowDrawList<'a>,
    size: (u32, u32),
}

impl<'a> ImguiBackend<'a> {
    pub fn new(ui: &'a Ui, draw_list: &'a WindowDrawList, size: (u32, u32)) -> Self {
        Self {
            ui,
            draw_list,
            size,
        }
    }
}

fn imgui_color(color: BackendColor) -> [f32; 4] {
    [
        f32::from(color.rgb.0) / 255.0,
        f32::from(color.rgb.1) / 255.0,
        f32::from(color.rgb.2) / 255.0,
        color.alpha as f32,
    ]
}

fn imgui_point(point: BackendCoord) -> [f32; 2] {
    [point.0 as f32, point.1 as f32]
}

impl<'a> DrawingBackend for ImguiBackend<'a> {
    type ErrorType = ImguiError;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let p = imgui_point(point);
        self.draw_list
            .add_rect(p, [p[0] + 1.0, p[1] + 1.0], imgui_color(color))
            .build();
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.draw_list
            .add_line(
                imgui_point(from),
                imgui_point(to),
                imgui_color(style.color()),
            )
            .thickness(style.stroke_width() as f32)
            .build();
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.draw_list
            .add_rect(
                imgui_point(upper_left),
                imgui_point(bottom_right),
                imgui_color(style.color()),
            )
            .thickness(style.stroke_width() as f32)
            .filled(fill)
            .build();
        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut path = path.into_iter();
        if let Some(mut coord) = path.next() {
            for next_coord in path {
                self.draw_line(coord, next_coord, style)?;
                coord = next_coord;
            }
        }
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = imgui_color(style.color());
        let path: Vec<BackendCoord> = path.into_iter().collect();

        let vertices: Vec<f64> = path
            .iter()
            .flat_map(|coords| vec![coords.0 as f64, coords.1 as f64])
            .collect();

        earcut(&vertices, &vec![], 2)
            .chunks_exact(3)
            .for_each(|triangle| {
                self.draw_list
                    .add_triangle(
                        [
                            vertices[2 * triangle[0]] as f32,
                            vertices[2 * triangle[0] + 1] as f32,
                        ],
                        [
                            vertices[2 * triangle[1]] as f32,
                            vertices[2 * triangle[1] + 1] as f32,
                        ],
                        [
                            vertices[2 * triangle[2]] as f32,
                            vertices[2 * triangle[2] + 1] as f32,
                        ],
                        color,
                    )
                    .filled(true)
                    .thickness(0.0)
                    .build();
            });

        Ok(())
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let str = im_str!("{}", text);
        let p = imgui_point(pos);

        let extents = self.ui.calc_text_size(&str, false, f32::MAX);
        let dx = match style.anchor().h_pos {
            HPos::Left => 0.0,
            HPos::Right => -extents[0],
            HPos::Center => -extents[0] / 2.0,
        };
        let dy = match style.anchor().v_pos {
            VPos::Top => extents[1],
            VPos::Center => extents[1] / 2.0,
            VPos::Bottom => 0.0,
        };

        self.draw_list
            .add_text([p[0] + dx, p[1] + dy], imgui_color(style.color()), str);
        Ok(())
    }
}
