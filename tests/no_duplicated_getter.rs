/// ```compile_fail
/// use rusty_german_entity_macro::RustyEntity;
///
/// #[derive(RustyEntity)]
/// pub struct Entity {
///    #[entity(description)]
///    desc: String,
///    #[entity(description)]
///    desc_again: String,
/// }
/// ```


fn main() {}
