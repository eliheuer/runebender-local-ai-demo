//! Writes the Shapes family: `Shapes-Regular.ufo`, `Shapes-Bold.ufo`,
//! and `Shapes.designspace`.
//!
//! Every glyph is a simple geometric shape drawn by a formula that
//! takes a size and a weight. Regular is weight 0; Bold is weight 60.
//! Because both masters come from the same formula, every glyph has
//! the same points in the same order in both, which is what a bolden
//! model trains on.
//!
//! Two shapes, octagon and chevron, are drawn in Regular only. They
//! are what you bolden with the model you train.
//!
//! Usage: `cargo run --release -- <out-dir>`

use norad::{Codepoints, Contour, ContourPoint, Font, Glyph, PointType};
use std::f64::consts::PI;
use std::path::Path;

/// Horizontal space on each side of a shape.
const SIDEBEARING: f64 = 60.0;
/// How much heavier Bold is, in font units.
const BOLD: f64 = 60.0;
/// Circle control-point distance as a fraction of the radius.
const KAPPA: f64 = 0.552_284_75;

/// The sizes a shape comes in, as a suffix and a box size in font
/// units. The 560 size has no suffix and carries a Unicode value so it
/// can be typed. Many sizes of each shape give the model enough pairs
/// to learn "push every edge outward" rather than memorise glyphs.
const SIZES: [(&str, f64); 8] = [
    (".300", 300.0),
    (".360", 360.0),
    (".420", 420.0),
    (".480", 480.0),
    ("", 560.0),
    (".620", 620.0),
    (".660", 660.0),
    (".700", 700.0),
];

struct Shape {
    name: &'static str,
    codepoint: char,
    /// Drawn in both masters, so the model learns from it.
    in_bold: bool,
    draw: fn(f64, f64) -> Vec<Vec<(f64, f64, PointType)>>,
}

const SHAPES: &[Shape] = &[
    Shape { name: "circle", codepoint: '\u{25CF}', in_bold: true, draw: circle },
    Shape { name: "square", codepoint: '\u{25A0}', in_bold: true, draw: square },
    Shape { name: "triangle", codepoint: '\u{25B2}', in_bold: true, draw: triangle },
    Shape { name: "diamond", codepoint: '\u{25C6}', in_bold: true, draw: diamond },
    Shape { name: "pentagon", codepoint: '\u{2B1F}', in_bold: true, draw: pentagon },
    Shape { name: "hexagon", codepoint: '\u{2B22}', in_bold: true, draw: hexagon },
    Shape { name: "star", codepoint: '\u{2605}', in_bold: true, draw: star },
    Shape { name: "plus", codepoint: '\u{271A}', in_bold: true, draw: plus },
    Shape { name: "cross", codepoint: '\u{2716}', in_bold: true, draw: cross },
    Shape { name: "bar", codepoint: '\u{2759}', in_bold: true, draw: bar },
    Shape { name: "dash", codepoint: '\u{25AC}', in_bold: true, draw: dash },
    Shape { name: "ring", codepoint: '\u{25EF}', in_bold: true, draw: ring },
    Shape { name: "frame", codepoint: '\u{25A1}', in_bold: true, draw: frame },
    Shape { name: "arrow", codepoint: '\u{27A1}', in_bold: true, draw: arrow },
    Shape { name: "octagon", codepoint: '\u{2BC4}', in_bold: false, draw: octagon },
    Shape { name: "chevron", codepoint: '\u{276F}', in_bold: false, draw: chevron },
];

type Pts = Vec<(f64, f64, PointType)>;

/// Centre of a shape of size `s` at weight `w`: the shape grows by `w`
/// on every side, so its box is `s + 2w` wide and tall, sitting on the
/// baseline with the sidebearing to its left.
fn centre(s: f64, w: f64) -> (f64, f64) {
    (SIDEBEARING + (s + 2.0 * w) / 2.0, (s + 2.0 * w) / 2.0)
}

fn line(x: f64, y: f64) -> (f64, f64, PointType) {
    (x, y, PointType::Line)
}

/// A regular polygon with `n` sides, circumradius `r`, first vertex at
/// angle `start` (radians, counter-clockwise from +x), around `c`.
fn polygon(c: (f64, f64), n: usize, r: f64, start: f64) -> Pts {
    (0..n)
        .map(|i| {
            let a = start + 2.0 * PI * i as f64 / n as f64;
            line(c.0 + r * a.cos(), c.1 + r * a.sin())
        })
        .collect()
}

/// A regular polygon thickened by `w`: the offset of a polygon with
/// circumradius `r` is a polygon with circumradius `r + w / cos(pi/n)`.
fn regular(s: f64, w: f64, n: usize, start: f64) -> Pts {
    let r = s / 2.0 + w / (PI / n as f64).cos();
    polygon(centre(s, w), n, r, start)
}

/// A circle as four cubic segments, counter-clockwise when `r > 0`,
/// clockwise when `r < 0` (for a counter).
fn circle_pts(c: (f64, f64), r: f64) -> Pts {
    let k = KAPPA * r;
    let (cx, cy) = c;
    let on = |x: f64, y: f64| (x, y, PointType::Curve);
    let off = |x: f64, y: f64| (x, y, PointType::OffCurve);
    vec![
        on(cx + r, cy),
        off(cx + r, cy + k),
        off(cx + k, cy + r),
        on(cx, cy + r),
        off(cx - k, cy + r),
        off(cx - r, cy + k),
        on(cx - r, cy),
        off(cx - r, cy - k),
        off(cx - k, cy - r),
        on(cx, cy - r),
        off(cx + k, cy - r),
        off(cx + r, cy - k),
    ]
}

/// An axis-aligned rectangle, counter-clockwise when `hw, hh > 0`.
fn rect(c: (f64, f64), hw: f64, hh: f64) -> Pts {
    vec![
        line(c.0 - hw, c.1 - hh),
        line(c.0 + hw, c.1 - hh),
        line(c.0 + hw, c.1 + hh),
        line(c.0 - hw, c.1 + hh),
    ]
}

fn circle(s: f64, w: f64) -> Vec<Pts> {
    vec![circle_pts(centre(s, w), s / 2.0 + w)]
}

fn square(s: f64, w: f64) -> Vec<Pts> {
    vec![rect(centre(s, w), s / 2.0 + w, s / 2.0 + w)]
}

fn triangle(s: f64, w: f64) -> Vec<Pts> {
    vec![regular(s, w, 3, PI / 2.0)]
}

fn diamond(s: f64, w: f64) -> Vec<Pts> {
    vec![regular(s, w, 4, PI / 2.0)]
}

fn pentagon(s: f64, w: f64) -> Vec<Pts> {
    vec![regular(s, w, 5, PI / 2.0)]
}

fn hexagon(s: f64, w: f64) -> Vec<Pts> {
    vec![regular(s, w, 6, 0.0)]
}

fn octagon(s: f64, w: f64) -> Vec<Pts> {
    vec![regular(s, w, 8, PI / 8.0)]
}

/// A five-pointed star. Weight pushes the outer points out and the
/// inner points in, so the arms get thicker.
fn star(s: f64, w: f64) -> Vec<Pts> {
    let c = centre(s, w);
    let outer = s / 2.0 + w;
    let inner = s * 0.22 - w * 0.5;
    let pts = (0..10)
        .map(|i| {
            let a = PI / 2.0 + PI * i as f64 / 5.0;
            let r = if i % 2 == 0 { outer } else { inner };
            line(c.0 + r * a.cos(), c.1 + r * a.sin())
        })
        .collect();
    vec![pts]
}

/// A plus sign: arm half-width `a`, reach `r`, twelve corners,
/// counter-clockwise from the bottom of the right arm.
fn plus_pts(c: (f64, f64), a: f64, r: f64) -> Pts {
    let (x, y) = c;
    vec![
        line(x + a, y - a),
        line(x + r, y - a),
        line(x + r, y + a),
        line(x + a, y + a),
        line(x + a, y + r),
        line(x - a, y + r),
        line(x - a, y + a),
        line(x - r, y + a),
        line(x - r, y - a),
        line(x - a, y - a),
        line(x - a, y - r),
        line(x + a, y - r),
    ]
}

fn plus(s: f64, w: f64) -> Vec<Pts> {
    vec![plus_pts(centre(s, w), s * 0.14 + w, s / 2.0 + w)]
}

/// The plus rotated 45 degrees, scaled so it fits the same box.
fn cross(s: f64, w: f64) -> Vec<Pts> {
    let c = centre(s, w);
    let k = std::f64::consts::FRAC_1_SQRT_2;
    let pts = plus_pts((0.0, 0.0), s * 0.14 + w, (s / 2.0 + w) * k * 1.3)
        .into_iter()
        .map(|(x, y, t)| (c.0 + (x - y) * k, c.1 + (x + y) * k, t))
        .collect();
    vec![pts]
}

fn bar(s: f64, w: f64) -> Vec<Pts> {
    vec![rect(centre(s, w), s * 0.14 + w, s / 2.0 + w)]
}

fn dash(s: f64, w: f64) -> Vec<Pts> {
    vec![rect(centre(s, w), s / 2.0 + w, s * 0.14 + w)]
}

/// A circle with a circular counter. The stroke is `0.18 s`, and
/// weight adds to it on both sides.
fn ring(s: f64, w: f64) -> Vec<Pts> {
    let c = centre(s, w);
    vec![
        circle_pts(c, s / 2.0 + w),
        circle_pts(c, -(s / 2.0 - s * 0.18 - w)),
    ]
}

/// A square with a square counter, the same stroke as the ring.
fn frame(s: f64, w: f64) -> Vec<Pts> {
    let c = centre(s, w);
    let o = s / 2.0 + w;
    let i = s / 2.0 - s * 0.18 - w;
    vec![rect(c, o, o), rect(c, -i, i)]
}

/// An arrow pointing right: a shaft and a triangular head.
fn arrow(s: f64, w: f64) -> Vec<Pts> {
    let c = centre(s, w);
    let (x, y) = c;
    let reach = s / 2.0 + w;
    let shaft = s * 0.12 + w;
    let head = s * 0.3 + w;
    let neck = x + reach - s * 0.45 - w;
    vec![vec![
        line(x - reach, y - shaft),
        line(neck, y - shaft),
        line(neck, y - head),
        line(x + reach, y),
        line(neck, y + head),
        line(neck, y + shaft),
        line(x - reach, y + shaft),
    ]]
}

/// A chevron pointing right, stroke `0.2 s` plus weight.
fn chevron(s: f64, w: f64) -> Vec<Pts> {
    let c = centre(s, w);
    let (x, y) = c;
    let reach = s / 2.0 + w;
    let t = s * 0.2 + w;
    vec![vec![
        line(x - reach * 0.4, y - reach),
        line(x + reach * 0.6, y),
        line(x - reach * 0.4, y + reach),
        line(x - reach * 0.4 - t, y + reach - t),
        line(x + reach * 0.6 - t * 1.4, y),
        line(x - reach * 0.4 - t, y - reach + t),
    ]]
}

fn glyph(name: &str, codepoint: Option<char>, s: f64, w: f64, draw: fn(f64, f64) -> Vec<Pts>) -> Glyph {
    let mut g = Glyph::new(name);
    g.width = (s + 2.0 * w + 2.0 * SIDEBEARING).round();
    if let Some(c) = codepoint {
        g.codepoints = Codepoints::new([c]);
    }
    for contour in draw(s, w) {
        let points = contour
            .into_iter()
            .map(|(x, y, typ)| {
                let smooth = typ == PointType::Curve;
                ContourPoint::new(x.round(), y.round(), typ, smooth, None, None)
            })
            .collect();
        g.contours.push(Contour::new(points, None));
    }
    g
}

fn master(style: &str, w: f64) -> Font {
    let mut font = Font::new();
    let info = &mut font.font_info;
    info.family_name = Some("Shapes".into());
    info.style_name = Some(style.into());
    info.units_per_em = Some(1000u32.into());
    info.ascender = Some(800.0);
    info.descender = Some(-200.0);
    info.x_height = Some(500.0);
    info.cap_height = Some(700.0);
    info.version_major = Some(1);
    let layer = font.layers.default_layer_mut();
    for shape in SHAPES {
        if w > 0.0 && !shape.in_bold {
            continue;
        }
        for (suffix, size) in SIZES {
            let name = format!("{}{}", shape.name, suffix);
            let codepoint = suffix.is_empty().then_some(shape.codepoint);
            layer.insert_glyph(glyph(&name, codepoint, size, w, shape.draw));
        }
    }
    font
}

const DESIGNSPACE: &str = r#"<?xml version='1.0' encoding='UTF-8'?>
<designspace format="5.0">
  <axes>
    <axis tag="wght" name="Weight" minimum="400" maximum="700" default="400"/>
  </axes>
  <sources>
    <source filename="Shapes-Regular.ufo" name="Shapes Regular" familyname="Shapes" stylename="Regular">
      <location><dimension name="Weight" xvalue="400"/></location>
    </source>
    <source filename="Shapes-Bold.ufo" name="Shapes Bold" familyname="Shapes" stylename="Bold">
      <location><dimension name="Weight" xvalue="700"/></location>
    </source>
  </sources>
</designspace>
"#;

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: make-shapes <out-dir>");
        std::process::exit(2);
    });
    let out = Path::new(&out);
    std::fs::create_dir_all(out).expect("create the output directory");
    for (style, w) in [("Regular", 0.0), ("Bold", BOLD)] {
        let path = out.join(format!("Shapes-{style}.ufo"));
        if path.exists() {
            std::fs::remove_dir_all(&path).expect("remove the old UFO");
        }
        let font = master(style, w);
        font.save(&path).expect("write the UFO");
        println!("{} glyphs -> {}", font.default_layer().len(), path.display());
    }
    let ds = out.join("Shapes.designspace");
    std::fs::write(&ds, DESIGNSPACE).expect("write the designspace");
    println!("-> {}", ds.display());
}
