use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{self, Write},
};

use crate::{Direction, Offset};

#[derive(PartialEq, Eq, Clone)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    pub tiles: Vec<T>,
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
            if i % self.width as usize == 0 {
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
            width: width as isize,
            height: height as isize,
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
    pub fn is_on_grid(&self, Offset { x, y }: Offset) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    #[must_use]
    pub fn get_tile_at(&self, offset: Offset) -> Option<&T> {
        let index = self.to_index(offset)?;
        Some(&self.tiles[index])
    }

    #[must_use]
    pub fn get_tile_at_unchecked(&self, offset: Offset) -> &T {
        debug_assert!(self.is_on_grid(offset));

        let index = self.to_index_unchecked(offset);
        &self.tiles[index]
    }

    #[must_use]
    pub fn get_tile_at_mut(&mut self, offset: Offset) -> Option<&mut T> {
        let index = self.to_index(offset)?;
        Some(&mut self.tiles[index])
    }

    #[must_use]
    pub fn get_tile_at_mut_unchecked(&mut self, offset: Offset) -> &mut T {
        debug_assert!(self.is_on_grid(offset));

        let index = self.to_index_unchecked(offset);
        &mut self.tiles[index]
    }

    #[must_use]
    fn to_index(&self, offset: Offset) -> Option<usize> {
        if self.is_on_grid(offset) {
            Some(self.to_index_unchecked(offset))
        } else {
            None
        }
    }

    #[must_use]
    fn to_index_unchecked(&self, offset: Offset) -> usize {
        debug_assert!(self.is_on_grid(offset));

        (offset.y * self.width + offset.x) as usize
    }

    /// ```
    /// # use advent_of_code_2024_lib::{Grid, Offset};
    ///
    /// let grid = Grid::<char>::from("12\n34\n");
    /// assert_eq!(grid.iter_offsets().collect::<Vec<_>>(), vec![
    ///     Offset { x: 0, y: 0 },
    ///     Offset { x: 1, y: 0 },
    ///     Offset { x: 0, y: 1 },
    ///     Offset { x: 1, y: 1 },
    /// ]);
    ///
    /// let grid = Grid::<char>::from("");
    /// assert_eq!(grid.iter_offsets().collect::<Vec<_>>(), vec![]);
    /// ```
    #[must_use]
    pub fn iter_offsets(&self) -> GridOffsetIter<'_, T> {
        GridOffsetIter {
            grid: self,
            next: Offset { x: 0, y: 0 },
        }
    }
}

impl<'grid, T> IntoIterator for &'grid Grid<T> {
    type Item = Offset;
    type IntoIter = GridOffsetIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_offsets()
    }
}

pub struct GridOffsetIter<'grid, T> {
    grid: &'grid Grid<T>,
    next: Offset,
}

impl<T> Iterator for GridOffsetIter<'_, T> {
    type Item = Offset;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.y >= self.grid.height {
            return None;
        }

        let to_return = self.next;

        self.next.x += 1;

        if self.next.x >= self.grid.width {
            self.next.x = 0;
            self.next.y += 1;
        }

        Some(to_return)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub cost: usize,
    pub offset: Offset,
    pub path_history: Vec<Offset>,
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
    pub fn find_optimal_path(&self, from: Offset, to: Offset) -> Option<Path> {
        let initial = Path {
            cost: 0,
            offset: from,
            path_history: Vec::new(),
        };

        let mut visited = HashSet::<Offset>::new();
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
                if next_offset != to {
                    nexts.push(Reverse(next_path));
                } else {
                    return Some(next_path);
                }
            }
        }

        None
    }

    pub fn calculate_all_costs_from(&self, from: Offset) -> HashMap<Offset, usize> {
        let mut costs = HashMap::new();

        let initial = Path {
            cost: 0,
            offset: from,
            path_history: Vec::new(),
        };

        costs.insert(from, 0);

        let mut visited = HashSet::<Offset>::new();
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
