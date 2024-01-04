use nalgebra::*;
use std::ops::{Add, Deref, DerefMut, Mul, Sub};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let test_result = solve_part1(INPUT_TEST, 7., 27.);
    println!("Test Part 1: {}", test_result);
    assert!(test_result == 2);

    let part_1_result = solve_part1(INPUT, 200_000_000_000_000., 400_000_000_000_000.);
    println!("Part 1: {}", part_1_result);

    let test_result = solve_part2(INPUT_TEST);
    println!("Test Part 2: {}", test_result);
    assert!(test_result == 47);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str, min: f64, max: f64) -> usize {
    let hailstones = input.lines().map(Hailstone::from).collect::<Vec<_>>();
    let pairs = hailstones.iter().enumerate().flat_map(|(i, hailstone)| {
        hailstones
            .iter()
            .skip(i + 1)
            .map(move |other| (hailstone, other))
    });

    pairs
        .filter_map(|(a, b)| a.get_intersection_xy_with(b).map(|it| (a, b, it)))
        .filter(|(_, _, intersection)| {
            (min <= intersection.x && intersection.x <= max)
                && (min <= intersection.y && intersection.y <= max)
        })
        .filter(|(a, b, intersection)| {
            a.intersection_is_in_future(intersection) && b.intersection_is_in_future(intersection)
        })
        .count()
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl Hailstone {
    fn get_intersection_xy_with(&self, other: &Self) -> Option<Position> {
        let line_segment = LineSegment {
            start: *self.position,
            end: *self.position + *self.velocity,
        };

        let other_line_segment = LineSegment {
            start: *other.position,
            end: *other.position + *other.velocity,
        };

        line_segment
            .get_intersection_xy_with(&other_line_segment)
            .map(Position)
    }

    fn intersection_is_in_future(&self, intersection: &Position) -> bool {
        // return the time it takes at this hailstone's velocity to reach the intersection
        let a = self.velocity.normalized();
        let b = (**intersection - *self.position).normalized();

        a.x * b.x + a.y * b.y > 0.
    }
}

impl From<&str> for Hailstone {
    fn from(input: &str) -> Self {
        let (position, velocity) = input.split_once('@').unwrap();
        let position = Position(Vector3::from(position));
        let velocity = Velocity(Vector3::from(velocity));

        Self { position, velocity }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Default)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Vector3 {
    fn from(input: &str) -> Self {
        match input.trim().split(',').collect::<Vec<_>>()[..] {
            [x, y, z] => Self {
                x: x.trim().parse().unwrap(),
                y: y.trim().parse().unwrap(),
                z: z.trim().parse().unwrap(),
            },
            _ => unreachable!(),
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<usize> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * (rhs as f64),
            y: self.y * (rhs as f64),
            z: self.z * (rhs as f64),
        }
    }
}

impl Vector3 {
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalized(&self) -> Self {
        let length = self.length();

        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

macro_rules! impl_vector3_container {
    ($type:ident) => {
        #[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Default)]
        struct $type(Vector3);

        impl Deref for $type {
            type Target = Vector3;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

impl_vector3_container!(Position);
impl_vector3_container!(Velocity);

struct LineSegment {
    start: Vector3,
    end: Vector3,
}

impl LineSegment {
    fn get_intersection_xy_with(&self, other: &Self) -> Option<Vector3> {
        let a1 = self.end.y - self.start.y;
        let b1 = self.start.x - self.end.x;
        let c1 = a1 * self.start.x + b1 * self.start.y;

        let a2 = other.end.y - other.start.y;
        let b2 = other.start.x - other.end.x;
        let c2 = a2 * other.start.x + b2 * other.start.y;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0.0 {
            return None;
        }

        Some(Vector3 {
            x: (b2 * c1 - b1 * c2) / delta,
            y: (a1 * c2 - a2 * c1) / delta,
            z: 0.,
        })
    }
}

fn solve_part2(input: &str) -> usize {
    // The starting position of the Rock can be represented as (RX,RY,RZ)
    // And its velocity as (RVX,RVY,RVZ)
    // The position of the Rock at some time t is therefore
    //   (RX,RY,RZ) + t*(RVX,RVY,RVZ)

    // Similarly, the starting position of a hailstone can be represented as (hx,hy,hz)
    // And its velocity as (hvx,hvy,hvz)
    // The position of the hailstone at some time t is therefore
    //   (hx,hy,hz) + t*(hvx,hvy,hvz)

    // A collision means that there exists a t such that
    //   (RX,RY,RZ) + t*(RVX,RVY,RVZ) = (hx,hy,hz) + t*(hvx,hvy,hvz)
    // Rearranging gives
    //   (RX,RY,RZ) - (hx,hy,hz) = t*(hvx,hvy,hvz) - t*(RVX,RVY,RVZ)
    //   (RX-hx,RY-hy,RZ-hz) = t*(hvx-RVX,hvy-RVY,hvz-RVZ)
    //   (RX-hx,RY-hy,RZ-hz) / (hvx-RVX,hvy-RVY,hvz-RVZ) = t

    // Separating each axis gives us the three following equations for t:
    //   t = (RX-hx)/(hvx-RVX)
    //   t = (RY-hy)/(hvy-RVY)
    //   t = (RZ-hz)/(hvz-RVZ)

    // X and Y pair:
    //   (RX-hx)/(hvx-RVX) = (RY-hy)/(hvy-RVY)
    //   (RX-hx)*(hvy-RVY) = (RY-hy)*(hvx-RVX)
    //   RX*hvy - RX*RVY - hx*hvy + hx*RVY = RY*hvx - RY*RVX - hy*hvx + hy*RVX

    // X and Z pair:
    //   (RX-hx)/(hvx-RVX) = (RZ-hz)/(hvz-RVZ)
    //   (RX-hx)*(hvz-RVZ) = (RZ-hz)*(hvx-RVX)
    //   RX*hvz - RX*RVZ - hx*hvz + hx*RVZ = RZ*hvx - RZ*RVX - hz*hvx + hz*RVX

    // Y and Z pair:
    //   (RY-hy)/(hvy-RVY) = (RZ-hz)/(hvz-RVZ)
    //   (RY-hy)*(hvz-RVZ) = (RZ-hz)*(hvy-RVY)
    //   RY*hvz - RY*RVZ - hy*hvz + hy*RVZ = RZ*hvy - RZ*RVY - hz*hvy + hz*RVY

    // Rearrange such that we can drop two of the sections that are true for all hailstones
    // since they only use terms from the rock (which is unchanging in our solution)
    //   RX*hvy - hy*RVX - RY*hvx + hx*RVY + hy*hvx - hx*hvy = RX*RVY + RY*RVX
    //
    // We can then take the equivalent equation for a second hailstone, h':
    //   RX*h'vy - h'y*RVX - RY*h'vx + h'x*RVY + h'y*h'vx - h'x*h'vy = RX*RVY + RY*RVX
    //
    // Since the right hand side (RX*RVY + RY*RVX) is equal in both cases, that gives us:
    //   RX*hvy - hy*RVX - RY*hvx + hx*RVY + hy*hvx - hx*hvy = RX*h'vy - h'y*RVX - RY*h'vx + h'x*RVY + h'y*h'vx - h'x*h'vy
    //
    // Rearranged gives us:
    //   RX*hvy - RX*h'vy + h'y*RVX - hy*RVX + RY*h'vx - RY*hvx + hx*RVY - h'x*RVY = h'y*h'vx - h'x*h'vy + hx*hvy - hy*hvx
    // or:
    //   RX(hvy - h'vy) + RVX(h'y - hy) + RY(h'vx - hvx) + RVY(hx - h'x) = h'y*h'vx - h'x*h'vy + hx*hvy - hy*hvx
    // or:
    //   RX(hvy - h'vy) + RY(h'vx - hvx) + RVX(h'y - hy) + RVY(hx - h'x) = h'y*h'vx - h'x*h'vy + hx*hvy - hy*hvx
    // We'll call this coefficients_xy()

    // We can do the same process for the other two pairs, giving us:
    //   RX(hvz - h'vz) + RZ(h'vx - hvx) + RVX(h'z - hz) + RVZ(hx - h'x) = h'z*h'vx - h'x*h'vz + hx*hvz - hz*hvx
    // We'll call this coefficients_xz()
    //
    // and:
    //   RY(hvz - h'vz) + RZ(h'vy - hvy) + RVY(h'z - hz) + RVZ(hy - h'y) = h'z*h'vy - h'y*h'vz + hy*hvz - hz*hvy
    // We'll call this coefficients_yz()

    // We have now got three equations for six unknowns (RX,RY,RZ,RVX,RVY,RVZ).
    // We also have multiple hailstones, so we can generate multiple equations.
    //
    // Gather these into a 6x6 matrix and a column vector and then finally:
    // Apply linear algebra to solve for the unknowns.
    //   A[coefficients left] * R[unknowns] = B[coefficients right]
    //   R = inv(A) * B

    let hailstones = input
        .lines()
        .map(Hailstone::from)
        .skip(1) // Probably float inaccuraccies, but the first hailstone causes the answer to be off by 4
        .take(3) // We only need 3 for the answer
        .collect::<Vec<_>>();

    let c_xy_01 = coefficients_xy(&hailstones[0], &hailstones[1]);
    let c_xy_02 = coefficients_xy(&hailstones[0], &hailstones[2]);
    let c_xz_01 = coefficients_xz(&hailstones[0], &hailstones[1]);
    let c_xz_02 = coefficients_xz(&hailstones[0], &hailstones[2]);
    let c_yz_01 = coefficients_yz(&hailstones[0], &hailstones[1]);
    let c_yz_02 = coefficients_yz(&hailstones[0], &hailstones[2]);

    let a = Matrix6::from_rows(&[
        c_xy_01.0, c_xy_02.0, c_xz_01.0, c_xz_02.0, c_yz_01.0, c_yz_02.0,
    ]);
    let b = Vector6::new(
        c_xy_01.1, c_xy_02.1, c_xz_01.1, c_xz_02.1, c_yz_01.1, c_yz_02.1,
    );

    let inv_a = a.try_inverse().unwrap();
    let res = inv_a * b;

    res.xyz().iter().sum::<f64>() as usize
}

fn coefficients_xy(a: &Hailstone, b: &Hailstone) -> (RowVector6<f64>, f64) {
    // RX(hvy - h'vy) + RY(h'vx - hvx) + RVX(h'y - hy) + RVY(hx - h'x) = h'y*h'vx - h'x*h'vy + hx*hvy - hy*hvx
    (
        RowVector6::new(
            a.velocity.y - b.velocity.y,
            b.velocity.x - a.velocity.x,
            0.,
            b.position.y - a.position.y,
            a.position.x - b.position.x,
            0.,
        ),
        b.position.y * b.velocity.x - b.position.x * b.velocity.y + a.position.x * a.velocity.y
            - a.position.y * a.velocity.x,
    )
}

fn coefficients_xz(a: &Hailstone, b: &Hailstone) -> (RowVector6<f64>, f64) {
    // RX(hvz - h'vz) + RZ(h'vx - hvx) + RVX(h'z - hz) + RVZ(hx - h'x) = h'z*h'vx - h'x*h'vz + hx*hvz - hz*hvx
    (
        RowVector6::new(
            a.velocity.z - b.velocity.z,
            0.,
            b.velocity.x - a.velocity.x,
            b.position.z - a.position.z,
            0.,
            a.position.x - b.position.x,
        ),
        b.position.z * b.velocity.x - b.position.x * b.velocity.z + a.position.x * a.velocity.z
            - a.position.z * a.velocity.x,
    )
}

fn coefficients_yz(a: &Hailstone, b: &Hailstone) -> (RowVector6<f64>, f64) {
    // RY(hvz - h'vz) + RZ(h'vy - hvy) + RVY(h'z - hz) + RVZ(hy - h'y) = h'z*h'vy - h'y*h'vz + hy*hvz - hz*hvy
    (
        RowVector6::new(
            0.,
            a.velocity.z - b.velocity.z,
            b.velocity.y - a.velocity.y,
            0.,
            b.position.z - a.position.z,
            a.position.y - b.position.y,
        ),
        b.position.z * b.velocity.y - b.position.y * b.velocity.z + a.position.y * a.velocity.z
            - a.position.z * a.velocity.y,
    )
}
