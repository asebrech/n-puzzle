pub mod utils; // This makes the puzzle_generator module available

// You can also re-export functions if needed
pub use utils::{make_goal, make_puzzle};
