use std::fs::File;
use std::io::Write;

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

    pub fn new(width: i32, height: i32, shapes: Vec<Shape>) -> Self {
        Self {
            width,
            height,
            shapes,
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

    // saves the generated SVG string to a file
    pub fn save(&self, filename: &str) {
        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(self.to_svg().as_bytes()).expect("Could not write to file");
        println!("{} successfully saved", filename)
    }
}

// tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_circle() {
        let shape = Shape::Circle {
            cx: 50,
            cy: 50,
            r: 25,
            fill: "blue".to_string(),
        };
        let canvas = SvgCanvas::new(100, 100, vec![shape]);
        let svg = canvas.to_svg();
        assert!(svg.contains("<circle cx=\"50\" cy=\"50\" r=\"25\" fill=\"blue\" />"));
    }
    
    #[test]
    fn test_svg_rect() {
        let shape = Shape::Rect {
            x: 10,
            y: 20,
            width: 80,
            height: 40,
            fill: "green".to_string(),
        };
        let canvas = SvgCanvas::new(100, 100, vec![shape]);
        let svg = canvas.to_svg();
        assert!(svg.contains("<rect x=\"10\" y=\"20\" width=\"80\" height=\"40\" fill=\"green\" />"));
    }
    
    #[test]
    fn test_svg_line() {
        let shape = Shape::Line {
            x1: 0,
            y1: 0,
            x2: 100,
            y2: 100,
            stroke: "black".to_string(),
        };
        let canvas = SvgCanvas::new(120, 120, vec![shape]);
        let svg = canvas.to_svg();
        assert!(svg.contains("<line x1=\"0\" y1=\"0\" x2=\"100\" y2=\"100\" stroke=\"black\" />"));
    }
    
    #[test]
    fn test_svg_text() {
        let shape = Shape::Text {
            x: 15,
            y: 30,
            size: 16,
            text: "Test".to_string(),
            fill: "red".to_string(),
        };
        let canvas = SvgCanvas::new(200, 50, vec![shape]);
        let svg = canvas.to_svg();
        assert!(svg.contains("<text x=\"15\" y=\"30\" font-size=\"16\" fill=\"red\">Test</text>"));
    }    
}
