pub mod puzzle_generator; // This makes the puzzle_generator module available

// You can also re-export functions if needed
pub use puzzle_generator::{make_goal, make_puzzle};
