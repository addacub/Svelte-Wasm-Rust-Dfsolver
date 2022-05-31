use core::fmt;
use std::usize;

use serde::{Deserialize, Serialize};

use crate::array2D;

use super::super::utils::array_2d::{Array2D, Axes};

/// A valid orientation and board position of a puzzle piece.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct PieceBoardPosition {
    name: String,
    board_position: (usize, usize),
    orientation: Array2D,
}

impl PieceBoardPosition {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_board_position(&self) -> (usize, usize) {
        self.board_position
    }

    pub fn get_orienation(&self) -> Array2D {
        self.orientation.clone()
    }
}

/// Model of a puzzle piece that can be placed on board.
/// Contains information required to determine all unique
/// piece orientations.
#[derive(Clone)]
pub struct PieceModel {
    // Arguments
    name: String,
    initial_orientation: Array2D,
    current_orientation: Array2D,
    board_position: Option<(usize, usize)>,

    // Restrictions
    max_rotations: usize,
    is_flippable: bool,

    // Flags
    rotation_count: usize,
    translation_count: usize,
    has_flipped: bool,
    orientation_exhausted: bool,
    is_used: bool,
}

impl PieceModel {
    /// Returns a new instance of `PieceModel`.
    ///
    /// # Arguments
    /// `name`  - The name of the puzzle piece (used only for indentification purposes).
    /// `initial_orientation` - The initial orientation of the puzzle piece, stored in a `Array2D` struct.
    /// `max_rotations` - The number of rotations which provide a unique puzzle piece (takes into account symmetry).
    /// `is_flippable` - Indicates if the puzzle piece has symmetry or asymmerty, which means the piece should be flipped.
    pub fn new(
        name: String,
        initial_orientation: Array2D,
        max_rotations: usize,
        is_flippable: bool,
    ) -> PieceModel {
        PieceModel {
            name,
            current_orientation: initial_orientation.clone(),
            initial_orientation,
            board_position: None,
            max_rotations,
            is_flippable,
            rotation_count: 0,
            translation_count: 0,
            has_flipped: false,
            orientation_exhausted: false,
            is_used: false,
        }
    }

    /// Returns an immutable reference to the current piece orientation
    pub fn current_orientation(&self) -> &Array2D {
        &self.current_orientation
    }

    /// Returns a copy of the current pieces translation count
    pub fn get_translation_count(&self) -> usize {
        self.translation_count
    }

    /// Returns a immutable reference to the piece's name.
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Rotates the puzzle piece model it is called on by 90 degrees anti-clockwise.
    fn rotate(&mut self) {
        self.current_orientation.rotate90(1);
        self.rotation_count += 1;
    }

    /// Flips the puzzle piece model it is called on along vertical axis (i.e. horizontally flips piece)
    fn flip(&mut self) {
        self.current_orientation.flip(Axes::Y);
        self.has_flipped = true;
        self.rotation_count = 0;
    }

    /// Resets the puzzle piece model back to its initial condition.
    pub fn reset(&mut self) {
        self.current_orientation = self.initial_orientation.clone();
        self.rotation_count = 0;
        self.translation_count = 0;
        self.has_flipped = false;
        self.orientation_exhausted = false;
    }

    /// Change the orientation of the puzzle piece model it is called on to return a unique new orientation.
    fn change_orientation(&mut self) {
        // Check if max number of rotations has been made
        if self.rotation_count == self.max_rotations {
            // Check if piece is flippable or has already been flipped
            if self.is_flippable & !self.has_flipped {
                // Shape if flippable and has not been flipped
                self.flip()
            } else {
                // Shape is either not flippable or has already been flipped
                self.orientation_exhausted = true;
            }
        } else {
            // Max number of rotations hasn't been reached yet
            self.rotate();
        }
    }

    /// Translates the puzzle piece model it is called on towards the left of its
    /// board position.
    ///
    /// If there are zeros in the top row of the piece, the piece will be
    /// incrementally translated until a value of 1 is at the board position.
    fn translate(&mut self) {
        if !self.is_translation_exhausted() {
            self.translation_count += 1;
        }
    }

    fn is_translation_exhausted(&self) -> bool {
        self.current_orientation
            .get(0, self.translation_count as usize)
            == 1
    }

    /// Translates and or rotates the puzzle piece model it is called on
    /// to get the next unique orienation of the piece.
    pub fn next_unique_orientation(&mut self) {
        if self.is_translation_exhausted() {
            self.change_orientation();

            // Reset translation information
            self.translation_count = 0;
        } else {
            self.translate();
        }
    }

    /// Sets the is_used field of the piece model it is called on
    /// to the boolean value passed in.
    ///
    /// # Arguments
    /// `is_used` - A boolean indicating if the piece is used or not.
    pub fn set_used(&mut self, is_used: bool) {
        self.is_used = is_used;
    }

    /// Returns a immutable reference to the `is_used` field
    pub fn is_used(&self) -> &bool {
        &self.is_used
    }

    /// Returns an immutable reference to the `is_exhausted` field
    pub fn is_exhausted(&self) -> &bool {
        &self.orientation_exhausted
    }

    /// Sets the board position of puzzle piece.
    pub fn set_board_position(&mut self, board_position: Option<(usize, usize)>) {
        match board_position {
            Some(position) => {
                let (row, col) = position;

                // Adjust column for piece translation
                let col = col - self.translation_count;

                self.board_position = Some((row, col));
            }
            None => self.board_position = None,
        }
    }

    /// Returns an immutable reference to the piece's board position
    pub fn get_board_position(&self) -> &Option<(usize, usize)> {
        &self.board_position
    }

    /// Returns a `PiecePosition` object from the piece model is it called on.
    pub fn get_piece_board_position(&self) -> PieceBoardPosition {
        PieceBoardPosition {
            name: self.name.clone(),
            board_position: self.board_position.unwrap(),
            orientation: self.current_orientation.clone(),
        }
    }
}

impl fmt::Display for PieceModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Piece: {}\n", self.get_name())?;
        match self.get_board_position() {
            Some(positoin) => {
                write!(f, "Board Positon: ({}, {})\n", positoin.0, positoin.1)?;
            }
            None => write!(f, "Board Position: (_, _)\n")?,
        }

        write!(f, "Current orientation:\n")?;

        return write!(f, "{}", self.current_orientation);
    }
}

#[rustfmt::skip::macros(array2D)]
pub fn create_piece_models() -> [PieceModel; 8] {
    [
        PieceModel::new(
            "2x3 No Hole".to_string(),
            array2D!(
                [1, 1, 1],
                [1, 1, 1]
            ),
            1,
            false,
        ),
        PieceModel::new(
            "2x3 Middle Hole".to_string(),
            array2D!(
                [1, 0, 1],
                [1, 1, 1]
            ),
            3,
            false,
        ),
        PieceModel::new(
            "2x3 End Hole".to_string(),
            array2D!(
                [1, 1, 0],
                [1, 1, 1]
            ),
            3,
            true,
        ),
        PieceModel::new(
            "2x4 Zig Zag".to_string(),
            array2D!(
                [0, 0, 1, 1],
                [1, 1, 1, 0]
            ),
            3,
            true,
        ),
        PieceModel::new(
            "2x4 Tee.".to_string(),
            array2D!(
                [0, 0, 1, 0],
                [1, 1, 1, 1]
            ),
            3,
            true,
        ),
        PieceModel::new(
            "2x4 L".to_string(),
            array2D!(
                [0, 0, 0, 1],
                [1, 1, 1, 1]
            ),
            3,
            true,
        ),
        PieceModel::new(
            "3x3 Zig Zag".to_string(),
            array2D!(
                [1, 0, 0],
                [1, 1, 1],
                [0, 0, 1]
            ),
            1,
            true,
        ),
        PieceModel::new(
            "3x3 L".to_string(),
            array2D!(
                [1, 0, 0],
                [1, 0, 0],
                [1, 1, 1]
            ),
            3,
            false,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array2D;

    #[test]
    fn change_piece_orientation_once() {
        // Arrange
        let mut piece = PieceModel::new(
            "2x3 End Hole".to_string(),
            array2D!([1, 1, 0], [1, 1, 1]),
            3,
            true,
        );

        // Act
        piece.change_orientation();

        // Assert
        assert_eq!(piece.rotation_count, 1);
        assert_eq!(piece.has_flipped, false);
    }

    #[test]
    fn change_piece_orientation_max_rotations() {
        // Arrange
        let mut piece = PieceModel::new(
            "2x3 End Hole".to_string(),
            array2D!([1, 1, 0], [1, 1, 1]),
            3,
            true,
        );

        // Act
        let mut i = 0;
        while i < 3 {
            piece.change_orientation();
            i += 1;
        }

        // Assert
        assert_eq!(piece.rotation_count, 3);
        assert_eq!(piece.has_flipped, false);
    }

    #[test]
    fn change_piece_orientation_until_flips() {
        // Arrange
        let mut piece = PieceModel::new(
            "2x3 End Hole".to_string(),
            array2D!([1, 1, 0], [1, 1, 1]),
            3,
            true,
        );

        // Act
        let mut i = 0;
        while i < 4 {
            piece.change_orientation();
            i += 1;
        }

        // Assert
        assert_eq!(piece.rotation_count, 0);
        assert_eq!(piece.has_flipped, true);
    }

    #[test]
    fn change_piece_orientation_until_exhausted() {
        // Arrange
        let mut piece = PieceModel::new(
            "2x3 End Hole".to_string(),
            array2D!([1, 1, 0], [1, 1, 1]),
            3,
            true,
        );

        // ACt
        let mut i = 0;
        while i < 8 {
            piece.change_orientation();
            i += 1;
        }

        // Assert
        assert_eq!(piece.rotation_count, 3);
        assert_eq!(piece.has_flipped, true);
        assert_eq!(piece.orientation_exhausted, true);
    }

    #[test]
    fn translate_piece_once() {
        // Arrange
        let mut piece = PieceModel::new(
            "2x4 Tee".to_string(),
            array2D!([0, 0, 1, 0], [1, 1, 1, 1]),
            3,
            true,
        );

        // Act
        piece.translate();

        // Assert
        assert_eq!(piece.translation_count, 1);
    }

    #[test]
    fn translate_piece_exhausted() {
        // Arrange
        let mut piece = PieceModel::new(
            "2x4 Tee".to_string(),
            array2D!([0, 0, 1, 0], [1, 1, 1, 1]),
            3,
            true,
        );

        // ACt
        let mut i = 0;
        while i < 3 {
            piece.translate();
            i += 1;
        }

        // Assert
        assert_eq!(piece.translation_count, 2);
        assert_eq!(piece.is_translation_exhausted(), true);
    }
}
