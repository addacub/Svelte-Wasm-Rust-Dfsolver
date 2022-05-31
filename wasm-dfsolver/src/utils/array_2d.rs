use core::fmt;
use std::{mem, ops};

use serde::{Deserialize, Serialize};

/// Creates an `Array2D` struct from a passed in array like object.
/// The array data is stored in a 1D `vec`, and the array shape is stored in a `Shape` struct.
///
/// # Arguments
///
/// * `array` - A list like object written as a set of nested arrays used to represent a 2D array.
///
/// # Examples
///
/// Creating a new `Array2D` struct
///
/// ```
/// # use dfsolver::{utils::array_2d::*, array2D};
/// let array: Array2D = array2D![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
///
/// let expected_result: Array2D = Array2D::new(Shape { rows: 3, cols: 3 }, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// assert_eq!(expected_result, array);
/// ```
#[macro_export]
macro_rules! array2D {
    ( $( $row_vector: tt ),* ) => {
        {
            use $crate::utils::array_2d::{ Array2D, Shape };
            let mut data = Vec::new();
            let mut shape = Shape { rows: 0, cols: 0 };

            $(
                let new_row = vec!$row_vector;

                // Set the length of the row (i.e. number of columns) if none has been set.
                // If set, assert that rows all have the same legnth.
                if shape.cols == 0 {
                    shape.cols = new_row.len();
                } else {
                    assert!(new_row.len() == shape.cols, "Supplied matrix had inconsistent row lengths")
                }

                // Counts the number of rows in the matrix
                shape.rows = shape.rows + 1;

                data.append(&mut vec!$row_vector);
            )*

            Array2D::new(shape, data)
        }
    };
}

/// A struct which corresponds to the shape of a M x N array.
///
/// # Arguments
///
/// * `rows` - the number of rows in the matrix (M dimension).
/// * `cols` - the number of columns in the matrix (N dimension).
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub rows: usize,
    pub cols: usize,
}

/// An enumeration of the axes which can be flipped along.
///
/// * Flipping along the X axes corresponds to vertically flipping the matrix.
/// * Flipping along the Y axes corresponds to horizontally flipping the matrix.
pub enum Axes {
    X,
    Y,
}

/// A struct which corresponds to a 2D array.
///
/// # Arguments
///
/// * `shape` - the shape of the M x N array where M is the number of rows, and N is the number of columns
/// * `data` - a 1D `Vec` which holds the data of the array
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Array2D {
    shape: Shape,
    data: Vec<u8>,
}

impl Array2D {
    pub fn new(shape: Shape, data: Vec<u8>) -> Array2D {
        Array2D { shape, data }
    }

    /// Returns the element at the specified index of the `Array2D` it is called on.
    ///
    /// # Arguments
    /// * `row` - The row of the desired element.
    /// * `col` - The column of the desired element.
    ///
    /// # Panics!
    /// Function will panic! if attempting to index outside the bounds of the array.
    ///
    /// # Examples
    /// Array indexing within the bounds of the array.
    ///
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6]);
    /// assert_eq!(matrix.get(0, 1), 2)
    /// ```
    ///
    /// Array indexing outside the bounds of the array
    ///
    /// ```should_panic
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6]);
    /// matrix.get(1, 4);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> u8 {
        if row > self.shape.rows - 1 || col > self.shape.cols - 1 {
            panic!("Indexing outside bounds of array");
        }

        self.data[self.shape.cols * row + col]
    }

    /// Sets the value of the element at the specified index of the `Array2D` it is called on.
    ///
    /// # Arguments
    /// * `row` - The row of the element to be set.
    /// * `col` - The column of the elemnt to be set.
    /// * `new_value` - The value that the element is to be set to.
    ///
    /// # Panics!
    /// Function will panic! if attempting to index outside the bounds of the array.
    ///
    /// # Example
    /// Array indexing within bounds of the array.
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([0, 0, 0], [0, 0, 0], [0, 0, 0]);
    /// matrix.set((1, 1), 5);
    /// assert_eq!(5, matrix.get(1, 1));
    /// ```
    ///
    /// Array indexing outside bounds of the array.
    /// ```should_panic
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([0, 0, 0], [0, 0, 0], [0, 0, 0]);
    /// matrix.set((1, 3), 5);
    /// ```
    pub fn set(&mut self, board_position: (usize, usize), new_value: u8) {
        let (row, col) = board_position;
        if row > self.shape.rows - 1 || col > self.shape.cols - 1 {
            panic!("Indexing outside bounds of array");
        }

        self.data[self.shape.cols * row + col] = new_value;
    }

    /// Returns a immutable reference to the data array of the `Array2D` it is called on.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Returns a immutable reference to the shape of `Array2D` it is called on.
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    /// Returns a mutable reference to the data array of the `Array2D` it is called on.
    pub fn get_mut_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    /// Flips a `Array2D` it is called on along the axes specified.
    /// Only the data field is mutated. The shape field is left untouched.
    ///
    /// # Arguments
    ///
    /// * `axes` - An enum value of either `Axes::X` or `Axes::Y`
    ///
    /// # Examples
    ///
    /// Flipping along the x axis (corresponds to flipping vertically).
    ///
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
    /// matrix.flip(Axes::X);
    ///
    /// let expected_result: Array2D = array2D!([7, 8, 9], [4, 5, 6], [1, 2, 3]);
    /// assert_eq!(expected_result, matrix);
    /// ```
    ///
    /// Flipping along the y axis (corresponds to flipping horizontally).
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
    /// matrix.flip(Axes::Y);
    ///
    /// let expected_result: Array2D = array2D!([3, 2, 1], [6, 5, 4], [9, 8, 7]);
    /// assert_eq!(expected_result, matrix);
    /// ```

    pub fn flip(&mut self, axes: Axes) {
        match axes {
            Axes::X => {
                for row_index in 0..self.shape.cols / 2 {
                    for col_index in 0..self.shape.cols {
                        self.data.swap(
                            row_index * self.shape.cols + col_index,
                            ((self.shape.rows - 1) - row_index) * self.shape.cols + col_index,
                        );
                    }
                }
            }
            Axes::Y => {
                for row_index in 0..self.shape.rows {
                    let row_slice = &mut self.data[row_index * self.shape.cols
                        ..row_index * self.shape.cols + self.shape.cols];
                    row_slice.reverse();
                }
            }
        }
    }

    /// Transposes the `Array2D` it is called on.
    /// Both the data field and the shape field are mutated.
    ///
    /// # Examples
    /// Transposing a 2 x 4 array
    ///
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
    /// matrix.transpose();
    /// let expected_result: Array2D = array2D!([0, 4], [1, 5], [2, 6], [3, 7]);
    /// assert_eq!(expected_result, matrix);
    /// ```
    ///
    /// Calling transpose on an array which has been tranposed.
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
    /// matrix.transpose();
    /// matrix.transpose();
    /// let expected_result: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
    /// assert_eq!(expected_result, matrix);
    ///
    pub fn transpose(&mut self) {
        // The product of M x N which gives the length of the 1D array which represents the data
        let mn = self.shape.rows * self.shape.cols;
        let mut visisted: Vec<bool> = vec![false; mn];

        let mut cycle_start = 0;

        while cycle_start != mn {
            let mut old_index = cycle_start;

            if !visisted[cycle_start] {
                loop {
                    let new_index = if old_index == mn - 1 {
                        mn - 1
                    } else {
                        (self.shape.cols * old_index) % (mn - 1)
                    };

                    if new_index == cycle_start {
                        visisted[old_index] = true;
                        break;
                    }

                    self.data.swap(old_index, new_index);
                    visisted[old_index] = true;
                    old_index = new_index;
                }
            }
            cycle_start += 1;
        }

        mem::swap(&mut self.shape.cols, &mut self.shape.rows);
    }

    /// Rotates the `Array2D` it is called on by k x 90 degrees.
    ///
    /// # Arguments
    /// `k` - how many times the array is rotated 90 degrees
    /// * A postive k rotates the array anti-clockwise.
    /// * A negative k rotates the array clockwise.
    ///
    /// # Examples
    ///
    /// Rotating anti-clockwise
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
    /// matrix.rotate90(2);
    /// let expected_result: Array2D = array2D!([3, 2], [1, 0]);
    /// assert_eq!(expected_result, matrix);
    /// ```
    pub fn rotate90(&mut self, k: isize) {
        let k = k % 4;

        match k {
            0 => return,

            1 | -3 => {
                self.flip(Axes::Y);
                self.transpose();
            }

            2 | -2 => {
                self.flip(Axes::X);
                self.flip(Axes::Y);
            }

            3 | -1 => {
                self.transpose();
                self.flip(Axes::Y)
            }

            _ => {
                panic!("Something went wrong when trying to rotate 2D array.");
            }
        }
    }

    pub fn append_array(&mut self, mut other: Array2D, axes: Axes) {
        match axes {
            Axes::X => {
                if self.shape.cols == other.shape().cols {
                    self.data.append(&mut other.get_mut_data());
                    self.shape = Shape {
                        rows: self.shape.rows + other.shape().rows,
                        cols: self.shape.cols,
                    }
                } else {
                    panic!("Tried to append Array2D with different row dimension.")
                }
            }
            Axes::Y => {
                if self.shape.rows == other.shape().rows {
                    let mut new_array: Vec<u8> = Vec::new();

                    for row_index in 0..self.shape.rows {
                        let mut new_row_self = self.data[row_index * self.shape.cols
                            ..row_index * self.shape.cols + self.shape.cols]
                            .to_vec();

                        let mut new_row_other = other.data[row_index * other.shape.cols
                            ..row_index * other.shape.cols + other.shape.cols]
                            .to_vec();

                        new_row_self.append(&mut new_row_other);
                        new_array.append(&mut new_row_self);
                    }

                    self.data = new_array;
                    self.shape = Shape {
                        rows: self.shape.rows,
                        cols: self.shape.cols + other.shape().cols,
                    }
                } else {
                    panic!("Tried to append Array2D with different column dimension.")
                }
            }
        }
    }
}

impl ops::Add<Array2D> for Array2D {
    type Output = Array2D;

    /// Adds an `Array2D` to the `Array2D` it is called on element wise.
    ///
    /// # Arguments
    /// `other_array` - An Array2D to be added. Both arrays must have the same dimension.
    ///
    /// # Panics!
    /// Will panic if 2D arrays with different dimensions are added together.
    ///
    /// # Examples
    /// Adding 2D arrays of the same dimension together:
    ///
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let matrix: Array2D = array2D!([0, 1, 2], [3, 4, 5]);
    /// let matrix2: Array2D = array2D!([2, 2, 2], [2, 2, 2]);
    /// let expected_result: Array2D = array2D!([2, 3, 4], [5, 6, 7]);
    /// assert_eq!(expected_result, matrix + matrix2);
    /// ```
    ///
    /// Adding together 2D arrays with different dimensions:
    /// ```should_panic
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let matrix: Array2D = array2D!([0, 1, 2], [3, 4, 5]);
    /// let matrix2: Array2D = array2D!([2, 2, 2, 2], [2, 2, 2, 2]);
    /// let _ = matrix + matrix2;
    /// ```
    fn add(self, other_array: Array2D) -> Array2D {
        if self.shape != other_array.shape {
            panic!("Array dimensions must be the same for arrays to be added element wise");
        }

        let new_array: Vec<u8> = self
            .data
            .iter()
            .zip(other_array.data.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        // Struct update syntax - uses values from another instance
        Array2D {
            data: new_array,
            ..self
        }
    }
}

impl fmt::Display for Array2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        for row_index in 0..self.shape.rows {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if row_index != 0 {
                write!(f, ",\n ")?;
            }

            write!(f, "[")?;

            for col_index in 0..self.shape.cols {
                if col_index != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.get(row_index, col_index))?;
            }
            write!(f, "]")?;
        }
        return write!(f, "]");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_value() {
        // Arrange
        let matrix: Array2D = array2D!([0, 0, 0], [0, 5, 0], [0, 0, 0]);

        // Act
        let a = matrix.get(1, 1);

        // Assert
        assert_eq!(5, a);
    }

    #[test]
    #[should_panic]
    fn get_value_panic() {
        // Arrange
        let matrix: Array2D = array2D!([0, 0, 0], [0, 0, 0], [0, 0, 0]);

        // Act
        matrix.get(1, 3);
    }

    #[test]
    fn set_value() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 0, 0], [0, 0, 0], [0, 0, 0]);

        // Act
        matrix.set((1, 1), 5);

        // Assert
        assert_eq!(5, matrix.get(1, 1));
    }

    #[test]
    #[should_panic]
    fn set_value_panic() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 0, 0], [0, 0, 0], [0, 0, 0]);

        // Act
        matrix.set((1, 3), 5);
    }

    #[test]
    fn flip_horizontally() {
        // Arrange
        let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
        let expected_result: Array2D = array2D!([3, 2, 1], [6, 5, 4], [9, 8, 7]);

        // Act
        matrix.flip(Axes::Y);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn flip_vertically() {
        // Arrange
        let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
        let expected_result: Array2D = array2D!([7, 8, 9], [4, 5, 6], [1, 2, 3]);

        // Act
        matrix.flip(Axes::X);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_transpose_square() {
        // Arrange
        let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
        let expected_result: Array2D = array2D!([1, 4, 7], [2, 5, 8], [3, 6, 9]);

        // Act
        matrix.transpose();

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_transpose_rectangle() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
        let expected_result: Array2D = array2D!([0, 4], [1, 5], [2, 6], [3, 7]);

        // Act
        matrix.transpose();

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_transpose_tranpose() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
        let expected_result: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);

        // Act
        matrix.transpose();
        matrix.transpose();

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate0() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([0, 1], [2, 3]);

        // Act
        matrix.rotate90(0);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate90() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([1, 3], [0, 2]);

        // Act
        matrix.rotate90(1);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate180() {
        // Arragne
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([3, 2], [1, 0]);

        // Act
        matrix.rotate90(2);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate270() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([2, 0], [3, 1]);

        // Act
        matrix.rotate90(3);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate360() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([0, 1], [2, 3]);

        // Act
        matrix.rotate90(4);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate_minus90() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([2, 0], [3, 1]);

        // Act
        matrix.rotate90(-1);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate_minus180() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([3, 2], [1, 0]);

        // Act
        matrix.rotate90(-2);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate_minus270() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([1, 3], [0, 2]);

        // Act
        matrix.rotate90(-3);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate_minus360() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([0, 1], [2, 3]);

        // Act
        matrix.rotate90(-4);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate540() {
        //  Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([3, 2], [1, 0]);

        // Act
        matrix.rotate90(6);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_rotate_minus540() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([3, 2], [1, 0]);

        // Act
        matrix.rotate90(-6);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_add_valid_shapes() {
        // Arrange
        let matrix: Array2D = array2D!([0, 1], [2, 3]);
        let matrix2: Array2D = array2D!([0, 1], [2, 3]);
        let expected_result: Array2D = array2D!([0, 2], [4, 6]);

        // Act
        let added_matrix = matrix + matrix2;

        // Assert
        assert_eq!(expected_result, added_matrix);
    }

    #[test]
    #[should_panic]
    fn test_add_invalid_shapes() {
        // Arrange
        let matrix: Array2D = array2D!([0, 1], [2, 3]);
        let matrix2: Array2D = array2D!([0, 1], [2, 3], [4, 5]);

        // Act & Assert
        let _ = matrix + matrix2;
    }

    #[test]
    fn test_append_array_axes_x() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1], [2, 3]);
        let matrix2: Array2D = array2D!([0, 1], [2, 3], [4, 5]);
        let expected_result: Array2D = array2D!([0, 1], [2, 3], [0, 1], [2, 3], [4, 5]);

        // Act
        matrix.append_array(matrix2, Axes::X);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    #[should_panic]
    fn test_append_array_axes_x_invalid() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1, 0], [2, 3, 0]);
        let matrix2: Array2D = array2D!([0, 1], [2, 3], [4, 5]);

        // Act & Assert
        matrix.append_array(matrix2, Axes::X);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_append_array_axes_y() {
        // Arrange
        let mut matrix: Array2D = array2D!(
            [0, 1],
            [2, 3],
            [1, 2]
        );
        let matrix2: Array2D = array2D!(
            [0, 1, 0],
            [2, 3, 0],
            [4, 5, 0]
        );
        let expected_result: Array2D = array2D!(
            [0, 1, 0, 1, 0],
            [2, 3, 2, 3, 0],
            [1, 2, 4, 5, 0]
        );

        // Act
        matrix.append_array(matrix2, Axes::Y);

        // Assert
        assert_eq!(expected_result, matrix);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    #[should_panic]
    fn test_append_array_axes_y_invalid() {
        // Arrange
        let mut matrix: Array2D = array2D!([0, 1, 0], [2, 3, 0]);
        let matrix2: Array2D = array2D!([0, 1], [2, 3], [4, 5]);

        // Act & Assert
        matrix.append_array(matrix2, Axes::X);
    }
}
