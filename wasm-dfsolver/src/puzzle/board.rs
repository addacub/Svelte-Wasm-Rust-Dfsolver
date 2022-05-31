use std::ops::Range;

use crate::{
    array2D,
    utils::array_2d::{Array2D, Axes, Shape},
};

use crate::utils::memento::*;

use super::piece::PieceModel;

/// Creates an empty calendar (i.e. board with no puzzles placed and no date selected)
fn create_empty_calendar() -> Array2D {
    array2D!(
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1]
    )
}

#[derive(Clone)]
pub struct BoardModel {
    board_layout: Array2D,
    history: RecursiveBoardHistory,
}

impl BoardModel {
    pub fn new(day: usize, month: usize) -> BoardModel {
        BoardModel {
            board_layout: initialise_calendar_layout(day, month, create_empty_calendar()),
            history: RecursiveBoardHistory::new(),
        }
    }

    /// Checks if piece being placed in its current orientation at the board position is valid.
    ///
    /// # Arguments
    /// * `board_pos` - A tuple of the row and column position in which to place the puzzle piece
    /// * `piece_model` - The model of the puzzle piece in the current orientation to be placed onto the board.
    pub fn is_piece_valid(
        &self,
        board_position: (usize, usize),
        piece_model: &mut PieceModel,
    ) -> bool {
        let (row, mut col) = board_position;
        // Check if translated board position (to take into account for spaces in puzzle piece)
        // is within bounds of the board.
        if piece_model.get_translation_count() > col {
            return false;
        } else {
            // Adjust board position to take into account translation
            piece_model.set_board_position(Some(board_position));
            (_, col) = piece_model.get_board_position().unwrap();
        }

        // Check if puzzle piece is within bounds of the board if placed.
        if row + piece_model.current_orientation().shape().rows - 1
            > self.board_layout.shape().rows - 1
            || col + piece_model.current_orientation().shape().cols - 1
                > self.board_layout.shape().cols - 1
        {
            return false;
        }

        // Check if piece will overlap with an existing piece
        let new_board_layout = self.board_layout.clone() + place_piece_on_board(piece_model);
        if new_board_layout.data().contains(&2) {
            return false;
        }

        // Check if piece will leave any holes
        if is_unreachable_holes(&new_board_layout) {
            return false;
        }

        // Valid move
        true
    }

    pub fn add_piece_to_board(&mut self, piece_model: &mut PieceModel) {
        let piece_on_board = place_piece_on_board(piece_model);
        self.board_layout = self.board_layout.clone() + piece_on_board;
    }

    /// Returns a immutable reference to the board layout.
    pub fn get_board_layout(&self) -> &Array2D {
        &self.board_layout
    }

    pub fn generate_memento(&mut self) {
        self.history
            .add_memento(Box::new(BoardMemento::new(self.board_layout.clone())))
    }

    pub fn restore_from_memento(&mut self) {
        let memento = self.history.get_memento();
        self.board_layout = memento.get_state();
    }
}

/// Initialises the calender
fn initialise_calendar_layout(day: usize, month: usize, mut empty_layout: Array2D) -> Array2D {
    // Set day
    let (row, col) = get_calendar_position(day, 2, 6, 7);
    empty_layout.set((row as usize, col as usize), 1);

    // Set month
    let (row, col) = get_calendar_position(month, 0, 5, 6);
    empty_layout.set((row as usize, col as usize), 1);

    empty_layout
}

/// Returns the calendar position (row and column) for a calendar entry (either a day or a month).
///
/// # Arguments
/// `calendar_entry` - The day or month that the calendar position is wanted for.
/// `start_row` - The start row of days or months on the calendar.
/// `end-col` - The last valid column. Note, the length of a month row is shorter due to the board layout
/// `divisor` - The length of the rows.
fn get_calendar_position(
    calendar_entry: usize,
    start_row: usize,
    end_col: usize,
    divisor: usize,
) -> (usize, usize) {
    let quotient = calendar_entry / divisor;
    let remainder = calendar_entry % divisor;

    let (row, col) = if remainder == 0 {
        (start_row + quotient - 1, end_col)
    } else {
        (start_row + quotient, remainder - 1)
    };

    (row, col)
}

/// Returns the next empty board position to place a puzzle piece on.
pub fn next_board_position(board_layout: &Array2D) -> (usize, usize) {
    for (index, &item) in board_layout.data().iter().enumerate() {
        if item == 0 {
            let (row, col) = (
                index / board_layout.shape().cols,
                index % board_layout.shape().cols,
            );
            return (row, col);
        }
    }

    panic!("Unable to find an empty board position.");
}

pub fn get_all_empty_positions(board_layout: &Array2D) -> Vec<(usize, usize)> {
    let mut empty_positions: Vec<(usize, usize)> = Vec::new();

    for (index, &item) in board_layout.data().iter().enumerate() {
        if item == 0 {
            let empty_position = (
                index / board_layout.shape().cols,
                index % board_layout.shape().cols,
            );

            empty_positions.push(empty_position);
        }
    }

    empty_positions
}

/// Checks if board is complete.
/// * If complete, the board layout should contain only values of 1.
/// * An incomplete board will contain values of 0.
pub fn is_board_complete(board_layout: &Array2D) -> bool {
    if board_layout.data().contains(&0) {
        return false;
    } else {
        return true;
    }
}

/// Places a puzzle piece in its current orientation onto a empty board at the position specified.
/// * Piece starting position (0, 0) is placed onto the board at the given position
///
/// # Arguments
/// * `(row, col)` - A tuple corresponding to the row and column of the board at which to place the puzzle piece
/// * `piece_model` - The puzzle piece in its current orientation to be placed onto an empty board.
///
/// # Panics!
/// If the specified board position results in the puzzle piece going outside of the board's bounds.
fn place_piece_on_board(piece_model: &PieceModel) -> Array2D {
    let (row, col) = piece_model.get_board_position().unwrap();

    // Create an empty board
    let mut piece_on_board = Array2D::new(Shape { rows: 7, cols: 7 }, vec![0; 7 * 7]);

    for row_piece in 0..piece_model.current_orientation().shape().rows {
        for col_piece in 0..piece_model.current_orientation().shape().cols {
            piece_on_board.set(
                (row + row_piece, col + col_piece),
                piece_model.current_orientation().get(row_piece, col_piece),
            )
        }
    }

    piece_on_board
}

/// Determines if current layout contains any unreachable holes.
/// * A unreachable hole cannot be filled by a puzzle piece and indicates a dead solution branch.
/// * If there are more than 5 adjacent holes, next board position is reachable.
/// * If there are less than 5 adjacent holes, next board position is unreachable and current board layout is invalid.
fn is_unreachable_holes(board_layout: &Array2D) -> bool {
    // Test if there are any holes first
    if is_board_complete(&board_layout) {
        // Board is complete and piece is valid.
        return false;
    }

    let empty_positions = get_all_empty_positions(board_layout);
    let mut tested_positions: Vec<(usize, usize)> = Vec::new();

    for board_position in empty_positions {
        if !tested_positions.contains(&board_position) {
            let mut tested_holes: Vec<(usize, usize)> = Vec::new();
            let mut other_holes: Vec<(usize, usize)> = Vec::new();
            other_holes.push(board_position.clone());

            loop {
                let mut more_holes: Vec<(usize, usize)> = Vec::new();

                for hole in &other_holes {
                    if !tested_holes.contains(hole) {
                        let neighbours = get_neighbours(hole.clone(), board_layout.clone());
                        more_holes.append(&mut evaluate_neighbours(hole.clone(), neighbours));
                        tested_holes.push(hole.clone());
                    }
                }

                other_holes.append(&mut more_holes);
                other_holes.sort();
                other_holes.dedup();

                if tested_holes.len() > 4 {
                    // Tested more than 4 adjacent holes which means hole isn't unreachable
                    tested_positions.extend(tested_holes);
                    break;
                } else if tested_holes.len() == other_holes.len() {
                    // Current position is unreachable. Piece is invalid - no need to keep testing
                    return true;
                }
            }
        }
    }

    // All empty board positions were tested and not unreachable
    return false;
}

/// Returns a matrix of adjacent neighbours at the specified board position.
///
/// # Arguments
/// * `(row, col)` - A tuple corresponding to the row and column of the specified board position.
/// * `board_layout` - An `Array2D` of the current board layout to return the neighbours from.
fn get_neighbours((mut row, mut col): (usize, usize), mut board_layout: Array2D) -> Array2D {
    // Check if board position is on top or bottom of board
    if row == 0 || row == board_layout.shape().rows - 1 {
        let mut extra_row = Array2D::new(
            Shape {
                rows: 1,
                cols: board_layout.shape().cols,
            },
            vec![1; board_layout.shape().cols],
        );

        // Check if board position is on top row
        if row == 0 {
            extra_row.append_array(board_layout, Axes::X);
            board_layout = extra_row;
            row += 1;
        } else {
            board_layout.append_array(extra_row, Axes::X)
        }
    }

    // Check if board position is on left or right side of board
    if col == 0 || col == board_layout.shape().cols - 1 {
        let mut extra_col = Array2D::new(
            Shape {
                rows: board_layout.shape().rows,
                cols: 1,
            },
            vec![1; board_layout.shape().rows],
        );

        // Check if board position is on left side
        if col == 0 {
            extra_col.append_array(board_layout, Axes::Y);
            board_layout = extra_col;
            col += 1;
        } else {
            board_layout.append_array(extra_col, Axes::Y);
        }
    }

    let mut neighbours: Vec<u8> = Vec::new();
    for row_offset in (Range { start: -1, end: 2 }) {
        // Convert types to be able to add negative offsets
        let (row, col): (i8, i8) = (row.try_into().unwrap(), col.try_into().unwrap());

        let first_index = (row + row_offset) * board_layout.shape().cols as i8 + (col - 1);
        let second_index = (row + (row_offset)) * board_layout.shape().cols as i8 + (col + 1);

        // Convert back to usize to index
        let first_index: usize = first_index.try_into().unwrap();
        let second_index: usize = second_index.try_into().unwrap();

        neighbours.append(&mut board_layout.data()[first_index..second_index + 1].to_vec());
    }

    Array2D::new(Shape { rows: 3, cols: 3 }, neighbours)
}

/// Returns a list of adjacent holes to be tested.
///
/// # Arguments
/// * `(row, col)` - A tuple corresponding to the row and column of the board position being evaulated.
/// * `neighbours` - An `Array2D` of the neighbours around the specified board position being evaulated.
fn evaluate_neighbours(board_position: (usize, usize), neighbours: Array2D) -> Vec<(usize, usize)> {
    let (row, col) = board_position;
    let (row, col): (i8, i8) = (row.try_into().unwrap(), col.try_into().unwrap());
    // let row_offset = if row == 0 { 0 } else { 1 };
    // let col_offset = if col == 0 { 0 } else { 1 };

    let mut other_holes: Vec<(usize, usize)> = Vec::new();

    for row_index in 0..neighbours.shape().rows {
        for col_index in 0..neighbours.shape().cols {
            if (row_index + col_index) % 2 != 0 {
                // If holse at this position, write to other_holes
                if neighbours.get(row_index, col_index) == 0 {
                    let new_row: i8 = row + (row_index as i8 - 1);
                    let new_col: i8 = col + (col_index as i8 - 1);
                    other_holes.push((new_row as usize, new_col as usize))
                }
            }
        }
    }

    return other_holes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_calendar_position_month() {
        // Arrange & Act
        let position = get_calendar_position(5, 0, 5, 6);

        // Assert
        assert_eq!((0, 4), position);
    }

    #[test]
    fn get_calendar_position_day() {
        // Arrange & Act
        let position = get_calendar_position(21, 2, 6, 7);

        // Assert
        assert_eq!((4, 6), position);
    }

    #[test]
    fn get_next_board_position() {
        // Arrange
        let test_board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let (row, col) = next_board_position(&test_board_layout);

        // Assert
        assert_eq!((3, 0), (row, col));
    }

    #[test]
    #[should_panic]
    fn get_next_board_position_panic() {
        // Arrange
        let test_board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1]
        );

        // Act & Assert
        let (_, _) = next_board_position(&test_board_layout);
    }

    #[test]
    fn test_incomplete_board() {
        // Arrange
        let test_board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_board_complete = is_board_complete(&test_board_layout);

        // Assert
        assert_eq!(false, is_board_complete);
    }

    #[test]
    fn test_complete_board() {
        // Arrange
        let test_board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1]
        );

        // Act
        let is_board_complete = is_board_complete(&test_board_layout);

        // Assert
        assert_eq!(true, is_board_complete);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_add_piece_to_board() {
        // Arrange
        let mut piece = PieceModel::new(
            "2x4 Zig Zag".to_string(),
            array2D!(
                [0, 0, 1, 1],
                [1, 1, 1, 0]
            ),
            3,
            true,
        );

        let board_position = (2, 3);
        let expected_result = array2D!(
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0]
        );
        piece.set_board_position(Some(board_position));

        // Act
        let piece_on_board = place_piece_on_board(&mut piece);

        // Assert
        assert_eq!(expected_result, piece_on_board);
    }

    #[test]
    #[should_panic]
    fn test_add_piece_to_board_invalid() {
        // Arrange
        let mut piece = PieceModel::new(
            "2x4 Zig Zag".to_string(),
            array2D!([0, 0, 1, 1], [1, 1, 1, 0]),
            3,
            true,
        );

        let board_position = (2, 4);
        piece.set_board_position(Some(board_position));

        // Act & Assert
        let _ = place_piece_on_board(&mut piece);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_get_neighbours_centre() {
        // Arrange
        let board_position = (3, 2);
        let board_layout = array2D!(
            [0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 1, 1],
            [0, 0, 5, 1, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0]
        );
        let expected_result: Array2D = array2D!(
            [1, 1, 0],
            [0, 5, 1],
            [0, 1, 0]
        );

        // Act
        let neighbours = get_neighbours(board_position, board_layout);

        // Assert
        assert_eq!(expected_result, neighbours);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_get_neighbours_top_lhcorner() {
        // Arrange
        let board_position = (0, 0);
        let board_layout = array2D!(
            [5, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 1, 1],
            [0, 0, 0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0]
        );
        let expected_result: Array2D = array2D!(
            [1, 1, 1],
            [1, 5, 1],
            [1, 0, 0]
        );

        // Act
        let neighbours = get_neighbours(board_position, board_layout);

        // Assert
        assert_eq!(expected_result, neighbours);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_get_neighbours_bottom_rhcorner() {
        // Arrange
        let board_position = (6, 6);
        let board_layout = array2D!(
            [5, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 1, 1],
            [0, 0, 0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 2, 1],
            [0, 0, 0, 0, 0, 1, 5]
        );
        let expected_result: Array2D = array2D!(
            [2, 1, 1],
            [1, 5, 1],
            [1, 1, 1]
        );

        // Act
        let neighbours = get_neighbours(board_position, board_layout);

        // Assert
        assert_eq!(expected_result, neighbours);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_evaluate_neighbours() {
        // Arrange
        let board_position = (3, 2);
        let neighbours = array2D!(
            [0, 0, 0],
            [1, 5, 0],
            [0, 0, 0]
        );
        let mut expected_result: Vec<(usize, usize)> = Vec::new();
        expected_result.push((2, 2));
        expected_result.push((3, 3));
        expected_result.push((4, 2));

        // Act
        let other_holes = evaluate_neighbours(board_position, neighbours);

        // Assert
        assert_eq!(expected_result, other_holes);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_1_hole_on_edge() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 0, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_1_hole_in_middle() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 1, 1, 1, 1, 0],
            [1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_1_hole_in_diagonal_hole() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [0, 1, 1, 1, 0, 1, 1],
            [0, 0, 0, 0, 1, 1, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_2_holes_on_edge() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_2_holes_in_middle() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 1, 0, 1, 0],
            [1, 1, 1, 1, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_3_holes_horizontal_on_edge() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 1, 0, 1, 1],
            [1, 1, 1, 1, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_3_holes_vertical_on_edge() {
        // Arrange
        let board_layout = array2D!(
            [0, 1, 0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_3_holes_horizontal_in_middle() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1, 1, 0],
            [1, 1, 1, 1, 0, 1, 0],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_3_holes_lshape_on_edge() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 0, 1],
            [0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_3_holes_lshape_in_middle() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1, 1, 1],
            [1, 0, 0, 1, 0, 0, 1],
            [1, 1, 1, 1, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_4_holes_square_shape_on_edge() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 0, 1, 1, 1, 1, 0],
            [0, 0, 1, 1, 0, 0, 0],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_4_holes_square_shape_in_corner() {
        // Arrange
        let board_layout = array2D!(
            [0, 0, 1, 0, 0, 0, 1],
            [0, 0, 1, 0, 0, 0, 1],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_4_holes_square_shape_in_middle() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 1, 1, 1, 0],
            [1, 0, 0, 1, 0, 0, 0],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_4_holes_horizontal_in_middle() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 1, 0],
            [1, 1, 1, 1, 1, 1, 0],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(true, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_valid_holes() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 1, 0],
            [1, 0, 0, 1, 1, 1, 0],
            [1, 1, 1, 1, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(false, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_valid_holes_real_example() {
        // Arrange
        let board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 0, 0, 0, 1, 1],
            [1, 1, 1, 1, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1]
        );

        // Act
        let is_unreachable = is_unreachable_holes(&board_layout);

        // Assert
        assert_eq!(false, is_unreachable);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_piece_valid_invalid_translation_count() {
        // Arrange
        let board_layout = array2D!(
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1]
        );
        let board_model = BoardModel {
            board_layout,
            history: RecursiveBoardHistory::new(),
        };
        let board_position = (0, 0);
        let mut puzzle_piece = PieceModel::new(
            "2x4 Tee.".to_string(),
            array2D!(
                [0, 0, 1, 0],
                [1, 1, 1, 1]
            ),
            4,
            true,
        );
        puzzle_piece.next_unique_orientation();

        // Act
        let is_piece_valid = board_model.is_piece_valid(board_position, &mut puzzle_piece);

        // Assert
        assert_eq!(false, is_piece_valid);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_piece_valid_piece_over_side() {
        // Arrange
        let board_layout = array2D!(
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1]
        );
        let board_model = BoardModel {
            board_layout,
            history: RecursiveBoardHistory::new(),
        };
        let board_position = (3, 4);
        let mut puzzle_piece = PieceModel::new(
            "2x4 Tee.".to_string(),
            array2D!(
                [0, 0, 1, 0],
                [1, 1, 1, 1]
            ),
            4,
            true,
        );

        // Act
        let is_piece_valid = board_model.is_piece_valid(board_position, &mut puzzle_piece);

        // Assert
        assert_eq!(false, is_piece_valid);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_piece_valid_piece_over_bottom() {
        // Arrange
        let board_layout = array2D!(
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1]
        );
        let board_model = BoardModel {
            board_layout,
            history: RecursiveBoardHistory::new(),
        };
        let board_position = (6, 0);
        let mut puzzle_piece = PieceModel::new(
            "2x4 Tee.".to_string(),
            array2D!(
                [0, 0, 1, 0],
                [1, 1, 1, 1]
            ),
            4,
            true,
        );

        // Act
        let is_piece_valid = board_model.is_piece_valid(board_position, &mut puzzle_piece);

        // Assert
        assert_eq!(false, is_piece_valid);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_piece_valid_piece_overlaps_piece() {
        // Arrange
        let board_layout = array2D!(
        [1, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1]
        );
        let board_model = BoardModel {
            board_layout,
            history: RecursiveBoardHistory::new(),
        };
        let board_position = (1, 2);
        let mut puzzle_piece = PieceModel::new(
            "2x4 Tee.".to_string(),
            array2D!(
                [0, 0, 1, 0],
                [1, 1, 1, 1]
            ),
            4,
            true,
        );

        // Act
        let is_piece_valid = board_model.is_piece_valid(board_position, &mut puzzle_piece);

        // Assert
        assert_eq!(false, is_piece_valid);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_piece_valid_piece_leaves_unreachable_hole() {
        // Arrange
        let board_layout = array2D!(
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1]
        );
        let board_model = BoardModel {
            board_layout,
            history: RecursiveBoardHistory::new(),
        };
        let board_position = (0, 0);
        let mut puzzle_piece = PieceModel::new(
            "2x3 End Hole".to_string(),
            array2D!(
                [0, 1, 1],
                [1, 1, 1]
            ),
            3,
            true,
        );

        // Act
        let is_piece_valid = board_model.is_piece_valid(board_position, &mut puzzle_piece);

        // Assert
        assert_eq!(false, is_piece_valid);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_piece_valid_piece_valid() {
        // Arrange
        let board_layout = array2D!(
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1]
        );
        let board_model = BoardModel {
            board_layout,
            history: RecursiveBoardHistory::new(),
        };
        let board_position = (0, 1);
        let mut puzzle_piece = PieceModel::new(
            "2x3 End Hole".to_string(),
            array2D!(
                [0, 1, 1],
                [1, 1, 1]
            ),
            3,
            true,
        );

        // Act
        let is_piece_valid = board_model.is_piece_valid(board_position, &mut puzzle_piece);

        // Assert
        assert_eq!(true, is_piece_valid);
    }
}
