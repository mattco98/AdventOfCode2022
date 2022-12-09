use std::fmt;

/// Represents a 2D grid of elements of type T
/// 
/// This class is a wrapper around a 2D vector, and adds some powerful features.
/// The primary feature is an easy navigation interface between elements that 
/// doesn't involve remembering how the items are laid out in memory. For example,
/// moving from a node down along the y axis can be done with a method named "down".
/// 
/// # Examples
/// ```
/// let grid = Grid::new(my_vector);
/// 
/// match grid.at(1, 2) {
///     None => todo!(),
///     Some(node) => {
///         let left_node = node.left();
/// 
///         // Iterate over all nodes below this one
///         for node in node.below_iter() {
///             // ...
///         }
///     }
/// }
/// ```
pub struct Grid<T> {
    items: Vec<Vec<T>>,
}

/// A specific position on a Grid.
/// 
/// Stores the x and y coordinate of a particular point on the Grid. Can access
/// the underlying data via a Deref implementation. Has various methods for 
/// traversal. 
pub struct GridNode<'a, T> {
    x: usize,
    y: usize,
    grid: &'a Grid<T>,
}

#[allow(dead_code)]
impl<T> Grid<T> {
    pub fn new(items: Vec<Vec<T>>) -> Self {
        Grid { items }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<GridNode<T>> {
        if y >= self.y_len() || x >= self.x_len() {
            None
        } else {
            Some(GridNode { x, y, grid: &self })
        }
    }

    pub fn x_len(&self) -> usize {
        if self.y_len() > 0 {
            self.items[0].len()
        } else {
            0
        }
    }

    pub fn y_len(&self) -> usize {
        self.items.len()
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        assert!(x < self.x_len() && y < self.y_len());

        self.items[y][x] = value;
    }

    pub fn set_node(&mut self, node: GridNode<T>, value: T) {
        self.items[node.y][node.x] = value;
    }

    fn get_value(&self, x: usize, y: usize) -> &T {
        // SAFETY: This method is not public, and can only be called via the Deref
        // implementation in GridNode. A GridNode is guaranteed to have a valid
        // coordinate pair.
        unsafe {
            self.items.get_unchecked(y).get_unchecked(x)
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Vec<T>> {
        self.items.iter()
    }
}

#[allow(dead_code)]
impl<'a, T> GridNode<'a, T> {
    pub fn moved(&self, dx: isize, dy: isize) -> Option<GridNode<'a, T>> {
        let new_x = self.x as isize + dx;
        let new_y = self.y as isize + dy;

        if new_x < 0 || new_x > self.grid.x_len() as isize || new_y < 0 || new_y > self.grid.y_len() as isize {
            None
        } else {
            Some(GridNode {
                x: new_x as usize,
                y: new_y as usize,
                grid: self.grid
            })
        }
    }

    pub fn left(&self) -> Option<GridNode<'a, T>> {
        self.moved(-1, 0)
    }

    pub fn right(&self) -> Option<GridNode<'a, T>> {
        self.moved(1, 0)
    }

    pub fn up(&self) -> Option<GridNode<'a, T>> {
        self.moved(0, -1)
    }

    pub fn down(&self) -> Option<GridNode<'a, T>> {
        self.moved(0, 1)
    }

    // TODO: Expose interface for custom deltas/inclusion if necessary
    pub fn left_iter(&self) -> GridNodeIterator<'a, T> {
        GridNodeIterator::new(self.grid, self.x, self.y, -1, 0)
    }

    pub fn right_iter(&self) -> GridNodeIterator<'a, T> {
        GridNodeIterator::new(self.grid, self.x, self.y, 1, 0)
    }

    pub fn up_iter(&self) -> GridNodeIterator<'a, T> {
        GridNodeIterator::new(self.grid, self.x, self.y, 0, -1)
    }

    pub fn down_iter(&self) -> GridNodeIterator<'a, T> {
        GridNodeIterator::new(self.grid, self.x, self.y, 0, 1)
    }
}

impl<'a, T> std::ops::Deref for GridNode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.grid.get_value(self.x, self.y)
    }
}

pub struct GridNodeIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
    delta_x: isize,
    delta_y: isize,
}

impl<'a, T> GridNodeIterator<'a, T> {
    fn new(grid: &'a Grid<T>, x: usize, y: usize, delta_x: isize, delta_y: isize) -> Self {
        Self { grid, x, y, delta_x, delta_y }
    }
}

impl<'a, T> Iterator for GridNodeIterator<'a, T> {
    type Item = GridNode<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_x = self.x as isize + self.delta_x;
        let next_y = self.y as isize + self.delta_y;
        if next_x < 0 || next_y < 0 {
            return None;
        }

        let next_node = self.grid.at(next_x as usize, next_y as usize);
        self.x = next_x as usize;
        self.y = next_y as usize;
        next_node        
    }
}

impl<T: fmt::Debug> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Grid [")?;

        for row in self.iter() {
            write!(f, "  ")?;
            for el in row.iter() {
                write!(f, "{:?} ", el)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "]")
    }
}
