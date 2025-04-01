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
