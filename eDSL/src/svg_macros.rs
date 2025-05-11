#[macro_export]
macro_rules! svg_elem {
    // circle (x, y, radius, colour)
    (circle($cx:literal, $cy:literal, $r:literal, $fill:literal)) => {
        crate::svg_edsl::Shape::Circle {
            cx: $cx,
            cy: $cy,
            r: $r,
            fill: $fill.to_string(),
        }
    };
    // rectangle (x, y, width, height, colour)
    (rect($x:literal, $y:literal, $width:literal, $height:literal, $fill:literal)) => {
        crate::svg_edsl::Shape::Rect {
            x: $x,
            y: $y,
            width: $width,
            height: $height,
            fill: $fill.to_string(),
        }
    };
    // line (start x-, y-coordinates, end x-, y-coordinates, colour)
    (line($x1:literal, $y1:literal, $x2:literal, $y2:literal, $stroke:literal)) => {
        crate::svg_edsl::Shape::Line {
            x1: $x1,
            y1: $y1,
            x2: $x2,
            y2: $y2,
            stroke: $stroke.to_string(),
        }
    };
    // text (x , y, font size, text, colour)
    (text($x:literal, $y:literal, $size:literal, $text:literal, $fill:literal)) => {
        crate::svg_edsl::Shape::Text {
            x: $x,
            y: $y,
            size: $size,
            text: $text.to_string(),
            fill: $fill.to_string(),
        }
    };
}

// get circle/rect/line/text and the expression -> into svg_elem!()
#[macro_export]
macro_rules! svg_expr {
    ($($method:ident ( $($args:expr),* )),*) => {
        vec![
            $(
                svg_elem!($method($($args),*))
            ),*
        ]
    };

    ($($anything:tt)*) => {
        compile_error!(concat!("Unbekanntes SVG-Element oder falsche Parameter: ", stringify!($($anything)*)));
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg_edsl::Shape;

    #[test]
    fn test_circle_macro() {
        let shape = svg_elem!(circle(10, 20, 30, "blue"));
        if let Shape::Circle { cx, cy, r, fill } = shape {
            assert_eq!(cx, 10);
            assert_eq!(cy, 20);
            assert_eq!(r, 30);
            assert_eq!(fill, "blue");
        } else {
            panic!("Expected Circle");
        }
    }

    #[test]
    fn test_rect_macro() {
        let shape = svg_elem!(rect(10, 20, 100, 200, "green"));
        if let Shape::Rect { x, y, width, height, fill } = shape {
            assert_eq!(x, 10);
            assert_eq!(y, 20);
            assert_eq!(width, 100);
            assert_eq!(height, 200);
            assert_eq!(fill, "green");
        } else {
            panic!("Expected Rect");
        }
    }

    #[test]
    fn test_line_macro() {
        let shape = svg_elem!(line(0, 0, 100, 100, "black"));
        if let Shape::Line { x1, y1, x2, y2, stroke } = shape {
            assert_eq!(x1, 0);
            assert_eq!(y1, 0);
            assert_eq!(x2, 100);
            assert_eq!(y2, 100);
            assert_eq!(stroke, "black");
        } else {
            panic!("Expected Line");
        }
    }

    #[test]
    fn test_text_macro() {
        let shape = svg_elem!(text(5, 10, 12, "Hello", "red"));
        if let Shape::Text { x, y, size, text, fill } = shape {
            assert_eq!(x, 5);
            assert_eq!(y, 10);
            assert_eq!(size, 12);
            assert_eq!(text, "Hello");
            assert_eq!(fill, "red");
        } else {
            panic!("Expected Text");
        }
    }
}