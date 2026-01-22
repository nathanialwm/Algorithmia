#[path = "quests/1.rs"]
pub mod potions;
pub use potions::{total_potions_1, total_potions_2, total_potions_3};

#[path = "quests/2.rs"]
pub mod runic_words;
pub use runic_words::{runic_words_1, runic_words_2, runic_words_3};

#[path = "quests/3.rs"]
pub mod mining;
pub use mining::{mining_1};

#[path = "quests/4.rs"]
pub mod smithing;
pub use smithing::{smithing_1};