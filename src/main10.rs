use std::str::FromStr;
use sscanf::sscanf;
use aoc2018::input_lines;
use aoc2018::Matrix;

#[derive(Debug,PartialEq,Eq,Ord,PartialOrd)]
struct Point {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl FromStr for Point {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        let parts = sscanf!(s, "position=<{i64},{i64}>velocity=<{i64},{i64}>")?;
        Ok(Point {
            x: parts.0,
            y: parts.1,
            vx: parts.2,
            vy: parts.3,
        })
    }}

fn draw(points: &[Point], xmin: i64, xmax: i64, ymin: i64, ymax: i64) {
    let width = (xmax-xmin) as usize + 1;
    let height = (ymax-ymin) as usize + 1;
    let mut drawing = Matrix::new_default(width, height, false);
    for p in points {
        drawing[((p.x-xmin) as isize, (p.y - ymin) as isize)] = true;
    }
    println!("---");
    drawing.draw();
    println!("---");
}

fn main() {
    let mut points = input_lines::<Point>();

    println!("{} points loaded.", points.len());
    let mut prev_area: i64  = i64::MAX;
    let mut time = 0;
    loop {
        for p in points.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;
        }
        let xmin = points.iter().map(|p| p.x).min().unwrap();
        let ymin = points.iter().map(|p| p.y).min().unwrap();
        let xmax = points.iter().map(|p| p.x).max().unwrap();
        let ymax = points.iter().map(|p| p.y).max().unwrap();
        let area = (xmax - xmin)*(ymax - ymin);
        if area > prev_area {
            println!("After {} seconds", time);
            break;
        } else {
            time += 1;
            prev_area = area;
            if area < 1000 {
                draw(&points, xmin, xmax, ymin, ymax);
            }
        }
    }
    }
