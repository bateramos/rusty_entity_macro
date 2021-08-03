
use rusty_entity_macro::RustyEntity;

#[derive(RustyEntity)]
pub struct Entity {
    #[entity(description)]
    desc: String,
    #[entity(expected_results)]
    res: Vec<String>,
}

fn main() {}
