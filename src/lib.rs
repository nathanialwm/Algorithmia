#[path = "quests/1.rs"]
pub mod potions;
pub use potions::{total_potions_1, total_potions_2, total_potions_3};

#[path = "quests/2.rs"]
pub mod runic_words;
pub use runic_words::{runic_words_1, runic_words_2};
