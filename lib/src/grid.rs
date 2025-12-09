use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{self, Write},
};

use crate::{Direction, Offset};

type Offset2D = Offset<isize, 2>;

#[derive(PartialEq, Eq, Clone)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    pub tiles: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new_with_dimensions(width: isize, height: isize, default_value: T) -> Self
    where
        T: Copy,
    {
        Grid {
            width: width,
            height: height,
            tiles: vec![default_value; width.cast_unsigned() * height.cast_unsigned()],
        }
    }
}

impl<T> fmt::Debug for Grid<T>
where
    for<'a> &'a T: Into<char>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()?;

        for (i, square) in self.tiles.iter().enumerate() {
            if i % self.width.cast_unsigned() == 0 {
                f.write_char('\n')?;
            }
            f.write_char(square.into())?;
        }

        Ok(())
    }
}

impl<T, S> From<S> for Grid<T>
where
    S: AsRef<str>,
    T: From<char>,
{
    fn from(value: S) -> Self {
        let height = value.as_ref().lines().count();
        let width = value.as_ref().lines().next().map_or(0, str::len);

        let mut grid = Grid {
            width: width.cast_signed(),
            height: height.cast_signed(),
            tiles: Vec::with_capacity(width * height),
        };

        for line in value.as_ref().lines() {
            for c in line.chars() {
                let tile = T::from(c);

                grid.tiles.push(tile);
            }
        }

        grid
    }
}

impl<T> Grid<T> {
    #[must_use]
    pub fn is_on_grid(&self, Offset::<isize, 2>([x, y]): Offset2D) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    #[must_use]
    pub fn get_tile_at(&self, offset: Offset2D) -> Option<&T> {
        let index = self.to_index(offset)?;
        Some(&self.tiles[index])
    }

    #[must_use]
    pub fn get_tile_at_unchecked(&self, offset: Offset2D) -> &T {
        debug_assert!(self.is_on_grid(offset));

        let index = self.to_index_unchecked(offset);
        &self.tiles[index]
    }

    #[must_use]
    pub fn get_tile_at_mut(&mut self, offset: Offset2D) -> Option<&mut T> {
        let index = self.to_index(offset)?;
        Some(&mut self.tiles[index])
    }

    #[must_use]
    pub fn get_tile_at_mut_unchecked(&mut self, offset: Offset2D) -> &mut T {
        debug_assert!(self.is_on_grid(offset));

        let index = self.to_index_unchecked(offset);
        &mut self.tiles[index]
    }

    #[must_use]
    fn to_index(&self, offset: Offset2D) -> Option<usize> {
        if self.is_on_grid(offset) {
            Some(self.to_index_unchecked(offset))
        } else {
            None
        }
    }

    #[must_use]
    fn to_index_unchecked(&self, offset: Offset2D) -> usize {
        debug_assert!(self.is_on_grid(offset));

        (offset[1] * self.width + offset[0]).cast_unsigned()
    }

    /// ```
    /// # use advent_of_code_2025_lib::{Grid, Offset};
    ///
    /// let grid = Grid::<char>::from("12\n34\n");
    /// assert_eq!(grid.iter_offsets().collect::<Vec<_>>(), vec![
    ///     Offset::<isize, 2>([0, 0]),
    ///     Offset::<isize, 2>([1, 0]),
    ///     Offset::<isize, 2>([0, 1]),
    ///     Offset::<isize, 2>([1, 1]),
    /// ]);
    ///
    /// let grid = Grid::<char>::from("");
    /// assert_eq!(grid.iter_offsets().collect::<Vec<_>>(), vec![]);
    /// ```
    #[must_use]
    pub fn iter_offsets(&self) -> GridOffsetIter<'_, T> {
        GridOffsetIter {
            grid: self,
            next: Offset::<isize, 2>([0, 0]),
        }
    }

    /// ```
    /// # use advent_of_code_2025_lib::{Grid, Offset};
    ///
    /// let grid = Grid::<char>::from("123\n456\n789\n");
    /// assert_eq!(grid.iter_surrounding(Offset::<isize, 2>([1, 1])).collect::<Vec<_>>(), vec![
    ///     Offset::<isize, 2>([0, 0]),
    ///     Offset::<isize, 2>([1, 0]),
    ///     Offset::<isize, 2>([2, 0]),
    ///     Offset::<isize, 2>([0, 1]),
    ///     Offset::<isize, 2>([2, 1]),
    ///     Offset::<isize, 2>([0, 2]),
    ///     Offset::<isize, 2>([1, 2]),
    ///     Offset::<isize, 2>([2, 2]),
    /// ]);
    ///
    /// assert_eq!(grid.iter_surrounding(Offset::<isize, 2>([0, 0])).collect::<Vec<_>>(), vec![
    ///     Offset::<isize, 2>([1, 0]),
    ///     Offset::<isize, 2>([0, 1]),
    ///     Offset::<isize, 2>([1, 1]),
    /// ]);
    ///
    /// let grid = Grid::<char>::from("");
    /// assert_eq!(grid.iter_surrounding(Offset::<isize, 2>([1, 1])).collect::<Vec<_>>(), vec![]);
    /// ```
    pub fn iter_surrounding(&self, origin: Offset2D) -> impl Iterator<Item = Offset2D> {
        (origin[1] - 1..=origin[1] + 1)
            .flat_map(move |y| {
                (origin[0] - 1..=origin[0] + 1).map(move |x| Offset::<isize, 2>([x, y]))
            })
            .filter(move |o| self.is_on_grid(*o) && *o != origin)
    }
}

impl<'grid, T> IntoIterator for &'grid Grid<T> {
    type Item = Offset2D;
    type IntoIter = GridOffsetIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'grid, T> Grid<T> {
    fn iter(&'grid self) -> GridOffsetIter<'grid, T> {
        self.iter_offsets()
    }
}

pub struct GridOffsetIter<'grid, T> {
    grid: &'grid Grid<T>,
    next: Offset2D,
}

impl<T> Iterator for GridOffsetIter<'_, T> {
    type Item = Offset2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next[1] >= self.grid.height {
            return None;
        }

        let to_return = self.next;

        self.next[0] += 1;

        if self.next[0] >= self.grid.width {
            self.next[0] = 0;
            self.next[1] += 1;
        }

        Some(to_return)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub cost: usize,
    pub offset: Offset2D,
    pub path_history: Vec<Offset2D>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait Traversability {
    fn is_traversable(&self) -> bool;
}

impl<T> Grid<T>
where
    T: Traversability,
{
    pub fn find_optimal_path(&self, from: Offset2D, to: Offset2D) -> Option<Path> {
        let initial = Path {
            cost: 0,
            offset: from,
            path_history: Vec::new(),
        };

        let mut visited = HashSet::<Offset2D>::new();
        visited.insert(initial.offset);

        let mut nexts = BinaryHeap::<Reverse<Path>>::new();
        nexts.push(Reverse(initial));

        while let Some(Reverse(path)) = nexts.pop() {
            for next_offset in Direction::iter()
                .map(Direction::into_offset)
                .map(|offset| path.offset + offset)
            {
                let Some(tile) = self.get_tile_at(next_offset) else {
                    continue;
                };

                if !tile.is_traversable() {
                    continue;
                }

                // By necessity of the minheap, if we've visited this place then
                // we've already visited it in a more optimal way
                if visited.contains(&next_offset) {
                    continue;
                }

                visited.insert(next_offset);

                let next_cost = path.cost + 1;

                let mut path_history = path.path_history.clone();
                path_history.push(next_offset);

                let next_path = Path {
                    cost: next_cost,
                    offset: next_offset,
                    path_history,
                };

                // Are we there yet?
                if next_offset == to {
                    return Some(next_path);
                }

                nexts.push(Reverse(next_path));
            }
        }

        None
    }

    pub fn calculate_all_costs_from(&self, from: Offset2D) -> HashMap<Offset2D, usize> {
        let mut costs = HashMap::new();

        let initial = Path {
            cost: 0,
            offset: from,
            path_history: Vec::new(),
        };

        costs.insert(from, 0);

        let mut visited = HashSet::<Offset2D>::new();
        visited.insert(initial.offset);

        let mut nexts = BinaryHeap::<Reverse<Path>>::new();
        nexts.push(Reverse(initial));

        while let Some(Reverse(path)) = nexts.pop() {
            for next_offset in Direction::iter()
                .map(Direction::into_offset)
                .map(|offset| path.offset + offset)
            {
                let Some(tile) = self.get_tile_at(next_offset) else {
                    continue;
                };

                if !tile.is_traversable() {
                    continue;
                }

                // By necessity of the minheap, if we've visited this place then
                // we've already visited it in a more optimal way
                if visited.contains(&next_offset) {
                    continue;
                }

                visited.insert(next_offset);

                let next_cost = path.cost + 1;

                let mut path_history = path.path_history.clone();
                path_history.push(next_offset);

                let next_path = Path {
                    cost: next_cost,
                    offset: next_offset,
                    path_history,
                };

                costs.insert(next_offset, next_cost);

                nexts.push(Reverse(next_path));
            }
        }

        costs
    }
}
