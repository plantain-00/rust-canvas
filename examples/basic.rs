extern crate cssparser;
extern crate euclid;
extern crate image;
extern crate rustcanvas;

use std::fs::File;
use std::f64::consts::PI;

use cssparser::{RGBA};
use euclid::{Point2D, Size2D, Rect};
use image::png::{PNGEncoder};
use image::{ColorType};
use rustcanvas::{create_canvas, CanvasContextType, FillOrStrokeStyle};

fn main() {
  let canvas = create_canvas(1920, 1080, CanvasContextType::CTX2D);
  let mut ctx = canvas.ctx;
  ctx.set_line_width(10.0);
  ctx.set_stroke_style(FillOrStrokeStyle::Color(RGBA::new(66, 165, 245, 255)));
  ctx.move_to(&Point2D::new(100.0, 100.0));
  ctx.line_to(&Point2D::new(600.0, 600.0));
  ctx.move_to(&Point2D::new(700.0, 200.0));
  ctx.save_context_state();
  ctx.stroke();
  ctx.set_stroke_style(FillOrStrokeStyle::Color(RGBA::new(244, 143, 177, 255)));
  ctx.save_context_state();
  ctx.bezier_curve_to(&Point2D::new(760.0, 300.0), &Point2D::new(920.0, 425.0), &Point2D::new(1100.0, 200.0));
  ctx.stroke();
  ctx.set_fill_style(FillOrStrokeStyle::Color(RGBA::new(233, 193, 127, 255)));
  ctx.arc(&Point2D::new(700.0, 600.0), 400.0, 0.0, 2.0 * PI as f32, false);
  ctx.fill();
  ctx.set_fill_style(FillOrStrokeStyle::Color(RGBA::new(0, 0, 0, 255)));
  ctx.set_font_style("200px \"PingFang TC\"");
  ctx.fill_text("哈哈".to_string(), 1000.0, 800.0, Some(200.0));
  ctx.set_fill_style(FillOrStrokeStyle::Color(RGBA::new(244, 143, 177, 255)));
  ctx.fill_text("二豆".to_string(), 300.0, 800.0, None);
  ctx.set_stroke_style(FillOrStrokeStyle::Color(RGBA::new(66, 165, 245, 255)));
  ctx.stroke_text("来呀打我啊".to_string(), 300.0, 400.0, None);
  ctx.set_font_style("200px \"Monaco\"");
  ctx.fill_text("Hello Moto".to_string(), 500.0, 700.0, None);
  ctx.close_path();
  let canvas_size = Size2D::new(1920.0, 1080.0);
  let size_i32 = canvas_size.to_i32();
  let pixels = ctx.image_data(Rect::new(Point2D::new(0i32, 0i32), size_i32), canvas_size);

  let f = File::create("./test.png").unwrap();

  let png = PNGEncoder::new(f);
  assert_eq!(pixels.len(), 1920 * 1080 * 4);
  png.encode(&pixels, 1920, 1080, ColorType::RGBA(8)).unwrap();
}
