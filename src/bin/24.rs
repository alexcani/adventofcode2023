use advent_of_code_2023::read_lines_as_vec;
use advent_of_code_2023::util::point::*;

struct Line {
    p0: Point,
    v: Point,
}

fn parse(input: &[String]) -> Vec<Line> {
    let mut lines = Vec::new();

    for line in input {
        let mut parts = line.trim().split('@');
        let pos = parts.next().unwrap().trim().split(',').map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
        let vel = parts.next().unwrap().trim().split(',').map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
        lines.push(Line {
            p0: Point::new(pos[0], pos[1]),
            v: Point::new(vel[0], vel[1]),
        });
    }
    lines
}

fn part1(input: &[Line]) -> u32 {
    let mut crossing_lines = 0;
    for (i, line_a) in input.iter().enumerate() {
        let x0_1 = line_a.p0.x as f64;
        let y0_1 = line_a.p0.y as f64;
        let vx_1 = line_a.v.x as f64;
        let vy_1 = line_a.v.y as f64;

        if vx_1 == 0.0 || vy_1 == 0.0 {
            continue;
        }

        let b_1 = vy_1/vx_1;

        for line_b in input.iter().skip(i+1) {
            let x0_2 = line_b.p0.x as f64;
            let y0_2 = line_b.p0.y as f64;
            let vx_2 = line_b.v.x as f64;
            let vy_2 = line_b.v.y as f64;

            if vx_2 == 0.0 || vy_2 == 0.0 {
                continue;
            }

            let b_2 = vy_2/vx_2;

            if b_1 == b_2 {
                continue;
            }

            let x = (b_1*x0_1 - b_2*x0_2 + y0_2 - y0_1)/(b_1 - b_2);
            let y = b_1*(x - x0_1) + y0_1;

            let t1 = (x - x0_1)/vx_1;
            let t2 = (x - x0_2)/vx_2;
            if t1 < 0.0 || t2 < 0.0 {
                continue;
            }

            const MIN: f64 = 200000000000000.0;
            const MAX: f64 = 400000000000000.0;
            if (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y) {
                crossing_lines += 1;
            }
        }
    }

    crossing_lines
}

fn main() {
    let _lines = read_lines_as_vec("inputs/24.txt").unwrap();
    let _example = r#"19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3"#.lines().map(|l| l.trim().to_string()).collect::<Vec<String>>();

    let now = std::time::Instant::now();
    let hazel_lines = parse(&_lines);
    println!("Parsing took {}us", now.elapsed().as_micros());

    let now = std::time::Instant::now();
    let r1 = part1(&hazel_lines);
    println!("Part 1 took {}us", now.elapsed().as_micros());
    println!("Part 1: {}", r1);
}
