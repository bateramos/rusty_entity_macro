use rusty_entity_macro::RustyEntity;

#[derive(RustyEntity)]
pub struct Entity {
    #[entity(sort)]
    #[entity(description)]
    desc: String,
}

fn main() {}
