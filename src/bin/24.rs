use advent_of_code_2023::read_lines_as_vec;
use nalgebra::{Matrix6, Vector6};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

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
            p0: Point::new(pos[0], pos[1], pos[2]),
            v: Point::new(vel[0], vel[1], vel[2]),
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

fn part2(input: &[Line]) -> u64 {
    // Randomly select 3 lines
    let line_i = &input[0];
    let line_n = &input[1];
    let line_j = &input[4];

    let p0_i = (line_i.p0.x as f64, line_i.p0.y as f64, line_i.p0.z as f64);
    let p0_n = (line_n.p0.x as f64, line_n.p0.y as f64, line_n.p0.z as f64);
    let p0_j = (line_j.p0.x as f64, line_j.p0.y as f64, line_j.p0.z as f64);

    let v_i = (line_i.v.x as f64, line_i.v.y as f64, line_i.v.z as f64);
    let v_n = (line_n.v.x as f64, line_n.v.y as f64, line_n.v.z as f64);
    let v_j = (line_j.v.x as f64, line_j.v.y as f64, line_j.v.z as f64);

    // Calculate all the factors we'll need for the matrix
    let dvx_in = v_i.0 - v_n.0;
    let dvy_in = v_i.1 - v_n.1;
    let dvz_in = v_i.2 - v_n.2;
    let dx_in = p0_i.0 - p0_n.0;
    let dy_in = p0_i.1 - p0_n.1;
    let dz_in = p0_i.2 - p0_n.2;

    let dvx_ij = v_i.0 - v_j.0;
    let dvy_ij = v_i.1 - v_j.1;
    let dvz_ij = v_i.2 - v_j.2;
    let dx_ij = p0_i.0 - p0_j.0;
    let dy_ij = p0_i.1 - p0_j.1;
    let dz_ij = p0_i.2 - p0_j.2;

    let b0_i = p0_i.0*v_i.1 - p0_i.1*v_i.0; // x0_1*vy_i - y0_i*vx_i
    let b1_i = p0_i.2*v_i.1 - p0_i.1*v_i.2; // z0_i*vy_i - y0_i*vz_i
    let b2_i = p0_i.0*v_i.2 - p0_i.2*v_i.0; // x0_i*vz_i - z0_i*vx_i

    let b0_n = p0_n.0*v_n.1 - p0_n.1*v_n.0; // x0_n*vy_n - y0_n*vx_n
    let b1_n = p0_n.2*v_n.1 - p0_n.1*v_n.2; // z0_n*vy_n - y0_n*vz_n
    let b2_n = p0_n.0*v_n.2 - p0_n.2*v_n.0; // x0_n*vz_n - z0_n*vx_n

    let b0_j = p0_j.0*v_j.1 - p0_j.1*v_j.0; // x0_j*vy_j - y0_j*vx_j
    let b1_j = p0_j.2*v_j.1 - p0_j.1*v_j.2; // z0_j*vy_j - y0_j*vz_j
    let b2_j = p0_j.0*v_j.2 - p0_j.2*v_j.0; // x0_j*vz_j - z0_j*vx_j

    let m = Matrix6::new(
        dvy_in, -dvx_in, 0.0, -dy_in, dx_in, 0.0,
        0.0, -dvz_in, dvy_in, 0.0, dz_in, -dy_in,
        dvz_in, 0.0, -dvx_in, -dz_in, 0.0, dx_in,
        dvy_ij, -dvx_ij, 0.0, -dy_ij, dx_ij, 0.0,
        0.0, -dvz_ij, dvy_ij, 0.0, dz_ij, -dy_ij,
        dvz_ij, 0.0, -dvx_ij, -dz_ij, 0.0, dx_ij,
    );

    if !m.is_invertible() {
        panic!("Matrix is not invertible");
    }

    let v = Vector6::new(
        b0_i - b0_n,
        b1_i - b1_n,
        b2_i - b2_n,
        b0_i - b0_j,
        b1_i - b1_j,
        b2_i - b2_j,
    );

    let x = m.try_inverse().unwrap() * v;

    // Answer is the added x0, y0, z0 found
    (x[(0, 0)] + x[(1, 0)] + x[(2, 0)]) as u64
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

    let now = std::time::Instant::now();
    let r2 = part2(&hazel_lines);
    println!("Part 2 took {}us", now.elapsed().as_micros());
    println!("Part 2: {}", r2);
}
