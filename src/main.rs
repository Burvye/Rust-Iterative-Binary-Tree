use embedded_graphics::*;
use embedded_graphics::{
    pixelcolor::PixelColor,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder},
};
use embedded_graphics_simulator::*;
use std::f32::consts::PI;

fn main() -> Result<(), core::convert::Infallible> {
    println!("Hello!");
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(500, 300));
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::CSS_ORANGE)
        .stroke_width(2)
        .fill_color(Rgb888::CYAN)
        .build();
    let beginning_point = Point { x: 250, y: 300 };
    Circle::new(beginning_point, 5)
        .into_styled(style)
        .draw(&mut display);
    line(
        beginning_point,
        9,
        PI / 12.0 + -PI / 2.0,
        90.0,
        &mut display,
    );
    line(
        beginning_point,
        9,
        -PI / 12.0 + -PI / 2.0,
        90.0,
        &mut display,
    );
    let output_settings = OutputSettingsBuilder::new().scale(2).build();

    Window::new("hl fractal math yay", &output_settings).show_static(&display);
    Ok(())
}
fn line(
    beginning: Point,
    step: i32,
    angle: f32,
    length: f32,
    display: &mut SimulatorDisplay<Rgb888>,
) {
    let mut stack = Vec::from([(beginning, step, angle, length)]);
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::CSS_ORANGE)
        .stroke_width(2)
        .fill_color(Rgb888::CYAN)
        .build();
    while let Some((point_rn, step, angle, length)) = stack.pop() {
        if step < 0 {
            continue;
        }

        let point_ltr = Point {
            x: point_rn.x + (length * angle.cos()) as i32,
            y: point_rn.y + (length * angle.sin()) as i32,
        };

        Line::new(point_rn, point_ltr)
            .into_styled(style)
            .draw(display);

        if step > 0 {
            stack.push((point_ltr, step - 1, angle + PI / 12.0, length * 0.70));
            stack.push((point_ltr, step - 1, angle - PI / 12.0, length * 0.70));
        }
    }
}
