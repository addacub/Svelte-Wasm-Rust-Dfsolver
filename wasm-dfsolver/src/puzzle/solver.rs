use super::board::{self, BoardModel};
use super::piece::{self, PieceBoardPosition, PieceModel};
use std::mem;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self};

#[derive(Clone)]
pub struct SolverSingleThreaded {
    pieces: [PieceModel; 8],
    board: BoardModel,
    solution_set: Vec<Vec<PieceBoardPosition>>,
}

impl SolverSingleThreaded {
    pub fn new(day: usize, month: usize) -> SolverSingleThreaded {
        SolverSingleThreaded {
            pieces: piece::create_piece_models(),
            board: BoardModel::new(day, month),
            solution_set: Vec::new(),
        }
    }

    /// Returns an immutable reference to pieces field.
    pub fn get_pieces(&self) -> &[PieceModel; 8] {
        &self.pieces
    }

    /// Returns an immutable reference to the solution_set field.
    pub fn get_solution_set(&self) -> &Vec<Vec<PieceBoardPosition>> {
        &self.solution_set
    }

    /// Returns a solution set for solver.
    ///
    /// Iterates through all possible combinations and appends valid
    ///  solutions to the `solution_set` vector.
    pub fn find_solution_set(&mut self) {
        // Create memento to handle state
        let mut start_index = 0;
        let mut solver_history: Vec<usize> = Vec::new();

        loop {
            // Flag to determine when to go back to previous state
            let mut restore_last_state = true;

            // Get next available board position
            let board_position = board::next_board_position(self.board.get_board_layout());

            // Get next eligible piece to be placed
            'piece_loop: for index in start_index..self.get_pieces().len() {
                let piece = &mut self.pieces[index];

                if !piece.is_used() {
                    while !piece.is_exhausted() {
                        if self.board.is_piece_valid(board_position, piece) {
                            // Set flag to indicate piece is used
                            piece.set_used(true);

                            // Save current board state
                            self.board.generate_memento();

                            // Update board state
                            self.board.add_piece_to_board(piece);

                            // Save current state of solver
                            solver_history.push(index);

                            // Set loop flag
                            restore_last_state = false;

                            // Reset start_index
                            start_index = 0;

                            break 'piece_loop;
                        } else {
                            piece.next_unique_orientation();
                        }
                    }

                    // Orientations Exhausted - reset piece
                    piece.reset();
                }
            }

            // Check if board is complete. Add solution to solution set if it is complete.
            if board::is_board_complete(self.board.get_board_layout()) {
                // A solutions has been found. Record solution
                let mut solution: Vec<PieceBoardPosition> = Vec::new();
                for piece in &self.pieces {
                    solution.push(piece.get_piece_board_position());
                }

                // Append solution to solution set
                self.solution_set.push(solution);

                // Change flag to indicate to undo last move
                restore_last_state = true;
            }

            // If hasn't broken out, has exhausted all possibilities for current loop.
            // Return to previous solver state or break from loop if finished searching.
            if restore_last_state {
                match solver_history.pop() {
                    Some(x) => {
                        start_index = x;

                        // return to previous board position
                        self.board.restore_from_memento();

                        // Remove flag indicating piece is used
                        self.pieces[start_index].set_used(false);
                        self.pieces[start_index].set_board_position(None);

                        // Get next unique orientation of piece
                        self.pieces[start_index].next_unique_orientation();
                    }
                    // Completed Search, break from outer loop
                    None => break,
                }
            }
        }
    }

    pub fn remove_duplicates(&mut self) {
        self.solution_set.sort();
        self.solution_set.dedup();
    }
}

pub struct SolverMultiThreaded {
    pieces: [PieceModel; 8],
    board: BoardModel,
    solution_set: Vec<Vec<PieceBoardPosition>>,
}

impl SolverMultiThreaded {
    pub fn new(day: usize, month: usize) -> SolverMultiThreaded {
        SolverMultiThreaded {
            pieces: piece::create_piece_models(),
            board: BoardModel::new(day, month),
            solution_set: Vec::new(),
        }
    }

    /// Creates worker threads to find solutions for a smaller subset of pieces
    pub fn find_solution_set(&mut self) {
        let (sender, receiver): (
            Sender<Vec<PieceBoardPosition>>,
            Receiver<Vec<PieceBoardPosition>>,
        ) = mpsc::channel();

        for index in 0..self.pieces.len() {
            let board_thread = self.board.clone();
            let sender_thread = sender.clone();

            thread::spawn(move || {
                let mut worker = Worker::create_new_worker(board_thread, sender_thread);
                worker.find_solution_set(index, index + 1);
            });
        }

        // It is possible to iterate over the receiver. The receiver will block
        // when the iterator asks for the next value. When the channel is closed,
        // the iterator will return None and end.
        // Issue if the original sender is cloned, as the original sender is never dropped.
        // Need to drop the original sender to prevent an infinite loop.
        // https://nickymeuleman.netlify.app/blog/multithreading-rust
        // drop the original sender, else the channel will remain open, causing the receiver to infinitely wait
        mem::drop(sender);

        for solution in receiver {
            self.solution_set.push(solution)
        }
    }

    /// Returns an immutable reference to pieces field.
    pub fn get_pieces(&self) -> &[PieceModel; 8] {
        &self.pieces
    }

    /// Returns an immutable reference to the solution_set field.
    pub fn get_solution_set(&self) -> &Vec<Vec<PieceBoardPosition>> {
        &self.solution_set
    }

    pub fn remove_duplicates(&mut self) {
        self.solution_set.sort();
        self.solution_set.dedup();
    }
}

struct Worker {
    pieces: [PieceModel; 8],
    board: BoardModel,
    sender: Sender<Vec<PieceBoardPosition>>,
}

impl Worker {
    /// Create a brand new worker
    pub fn create_new_worker(board: BoardModel, sender: Sender<Vec<PieceBoardPosition>>) -> Worker {
        Worker {
            pieces: piece::create_piece_models(),
            board,
            sender,
        }
    }

    /// Returns a solution set for solver.
    ///
    /// Iterates through all possible combinations and appends valid
    ///  solutions to the `solution_set` vector.
    pub fn find_solution_set(&mut self, mut start_index: usize, mut end_index: usize) {
        // Create memento to handle state
        let mut solver_history: Vec<(usize, usize)> = Vec::new();

        loop {
            // Flag to determine when to go back to previous state
            let mut restore_last_state = true;

            // Get next available board position
            let board_position = board::next_board_position(self.board.get_board_layout());

            // Get next eligible piece to be placed
            'piece_loop: for index in start_index..end_index {
                let piece = &mut self.pieces[index];

                if !piece.is_used() {
                    while !piece.is_exhausted() {
                        if self.board.is_piece_valid(board_position, piece) {
                            // Set flag to indicate piece is used
                            piece.set_used(true);

                            // Save current board state
                            self.board.generate_memento();

                            // Update board state
                            self.board.add_piece_to_board(piece);

                            // Save current state of solver
                            solver_history.push((index, end_index));

                            // Set loop flag
                            restore_last_state = false;

                            // Reset start_index & end_index
                            start_index = 0;
                            end_index = self.pieces.len();

                            break 'piece_loop;
                        } else {
                            piece.next_unique_orientation();
                        }
                    }

                    // Orientations Exhausted - reset piece
                    piece.reset();
                }
            }

            // Check if board is complete. Add solution to solution set if it is complete.
            if board::is_board_complete(self.board.get_board_layout()) {
                // A solutions has been found. Record solution
                let mut solution: Vec<PieceBoardPosition> = Vec::new();
                for piece in &self.pieces {
                    solution.push(piece.get_piece_board_position());
                }

                // Send solution to receiver to be appended to solution set
                self.sender.send(solution).unwrap();

                // Change flag to indicate to undo last move
                restore_last_state = true;
            }

            // If hasn't broken out, has exhausted all possibilities for current loop.
            // Return to previous solver state or break from loop if finished searching.
            if restore_last_state {
                match solver_history.pop() {
                    Some(x) => {
                        (start_index, end_index) = x;

                        // return to previous board position
                        self.board.restore_from_memento();

                        // Remove flag indicating piece is used
                        self.pieces[start_index].set_used(false);
                        self.pieces[start_index].set_board_position(None);

                        // Get next unique orientation of piece
                        self.pieces[start_index].next_unique_orientation();
                    }
                    // Completed Search, break from outer loop
                    None => break,
                }
            }
        }
    }
}
