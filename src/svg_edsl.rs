// use crate::math_edsl::Expr;
use std::fs::File;
use std::io::Write;

use crate::math_edsl::Expr;

//define an enum to represent the shapes in SVG
pub enum Shape {
    Circle {
        cx: i32,
        cy: i32,
        r: i32,
        fill: String,
    },
    Rect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fill: String,
    },
    Line {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        stroke: String,
    },
    Text {
        x: i32,
        y: i32,
        size: i32,
        text: String,
        fill: String,
    },
}

//represents an SVG canvas containing the shapes & stores all shapes added to the canvas
pub struct SvgCanvas {
    width: i32,
    height: i32,
    shapes: Vec<Shape>,
}

//creates a new SVG canvas with a given height and width & an empty vector (array) of shapes
impl SvgCanvas {
    pub fn add_function_graph(&mut self, expr: &Expr, color: &str) {
        let x_min = -10.0;
        let x_max = 10.0;
        let step = 0.05;

        let mut prev_x = x_min;
        let mut prev_y = expr.eval(prev_x);
        let mut x = prev_x + step;

        while x <= x_max {
            let y = expr.eval(x);
            let screen_x1 = (prev_x * 10.0).round() as i32 + 250;
            let screen_y1 = (250.0 - (prev_y * 10.0)).round() as i32;
            let screen_x2 = (x * 10.0).round() as i32 + 250;
            let screen_y2 = (250.0 - (y * 10.0)).round() as i32;

            self.add_shape(Shape::Line {
                x1: screen_x1,
                y1: screen_y1,
                x2: screen_x2,
                y2: screen_y2,
                stroke: color.to_string(),
            });

            prev_x = x;
            prev_y = y;
            x += step;
        }
    }

    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            shapes: vec![],
        }
    }

    //adds a shape to the canvas
    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    //generates an SVG string representing the canvas and its shapes
    pub fn to_svg(&self) -> String {
        let mut svg = format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
            self.width, self.height
        );
        //convert each shape into its corresponding SVG tag
        for shape in &self.shapes {
            match shape {
                Shape::Circle { cx, cy, r, fill } => {
                    svg.push_str(&format!(
                        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n",
                        cx, cy, r, fill
                    ));
                }
                Shape::Rect {
                    x,
                    y,
                    width,
                    height,
                    fill,
                } => {
                    svg.push_str(&format!(
                        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                        x, y, width, height, fill
                    ));
                }
                Shape::Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    stroke,
                } => {
                    svg.push_str(&format!(
                        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" />\n",
                        x1, y1, x2, y2, stroke
                    ));
                }
                Shape::Text {
                    x,
                    y,
                    size,
                    fill,
                    text,
                } => {
                    svg.push_str(&format!(
                        "<text x=\"{}\" y=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>",
                        x, y, size, fill, text
                    ));
                }
            }
        }
        svg.push_str("</svg>");
        svg
    }

    //saves the generated SVG string to a file
    pub fn save(&self, filename: &str) {
        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(self.to_svg().as_bytes())
            .expect("Could not create file");
    }
}
