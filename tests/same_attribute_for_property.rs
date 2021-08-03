use rusty_german_entity_macro::RustyEntity;
use rusty_german_types::Exercise;

#[derive(RustyEntity)]
pub struct Entity {
    #[entity(sort)]
    #[entity(description)]
    pub phrase: String,
    #[entity(expected_result)]
    pub preposition: String,
}

fn main() {
    let entity = Entity {
        phrase: "test".to_owned(),
        preposition: "prep".to_owned(),
    };

    assert_eq!(entity.get_description(), "test");
    assert_eq!(entity.get_sort_property(), "test");
    assert_eq!(entity.get_expected_result(), "prep");
}

