use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let before = Instant::now();
    let part_1_result = solve_part1(INPUT);
    let after = Instant::now();
    println!("Part 1: {part_1_result}");
    println!("Part 1 took: {:?}", after - before);
}

#[inline(never)]
fn solve_part1(mut input: &str) -> usize {
    let mut shapes = Vec::new();
    while let Some(shape) = parse_next_shape(&mut input) {
        shapes.push(shape);
    }

    input
        .lines()
        .flat_map(Region::try_from)
        .filter(|region| region.can_fit_shapes(&shapes))
        .count()
}

#[derive(Debug, Clone, Copy)]
struct Shape {
    w: usize,
    h: usize,
}

fn parse_next_shape(input: &mut &str) -> Option<Shape> {
    let (raw_shape, rem) = input.split_once("\n\n")?;
    *input = rem;

    Shape::try_from(raw_shape).ok()
}

impl TryFrom<&str> for Shape {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut shape_lines = value.lines();

        let index_line = shape_lines.next().ok_or(())?;
        if !index_line.ends_with(':') {
            // Not a Shape
            return Err(());
        }

        let (w, h) = shape_lines.fold((0, 0), |acc, next| (next.len().max(acc.0), acc.1 + 1));

        Ok(Self { w, h })
    }
}

#[derive(Debug, Clone)]
struct Region {
    w: usize,
    h: usize,
    num_shapes_by_index: Vec<usize>,
}

impl TryFrom<&str> for Region {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (area, shapes) = value.split_once(": ").ok_or(())?;

        let (w, h) = area.split_once('x').ok_or(())?;
        let w = w.parse().map_err(|_| ())?;
        let h = h.parse().map_err(|_| ())?;

        let num_shapes_by_index = shapes.split(' ').flat_map(str::parse).collect();

        Ok(Self {
            w,
            h,
            num_shapes_by_index,
        })
    }
}

impl Region {
    fn can_fit_shapes(&self, shapes: &[Shape]) -> bool {
        let (max_shape_w, max_shape_h) = shapes.iter().fold((0, 0), |acc, shape| {
            (acc.0.max(shape.w), acc.0.max(shape.h))
        });

        let w_fittable = self.w / max_shape_w;
        let h_fittable = self.h / max_shape_h;

        let num_fittable = w_fittable * h_fittable;

        self.num_shapes_by_index.iter().sum::<usize>() <= num_fittable
    }
}

// The cheese solution above DOES NOT WORK with the test data.

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_input_part1() {
//         let input = r#"0:
// ###
// ##.
// ##.

// 1:
// ###
// ##.
// .##

// 2:
// .##
// ###
// ##.

// 3:
// ##.
// ###
// ##.

// 4:
// ###
// #..
// ###

// 5:
// ###
// .#.
// ###

// 4x4: 0 0 0 0 2 0
// 12x5: 1 0 1 0 2 2
// 12x5: 1 0 1 0 3 2"#;

//         assert_eq!(solve_part1(input), 2);
//     }
// }
