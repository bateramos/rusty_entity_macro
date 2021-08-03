/// ```compile_fail
/// #[derive(RustyEntity)]
/// pub struct Entity {
///    #[entity(description)]
///    desc: String,
///    #[entity(description)]
///    desc_again: String,
/// }
/// ```


fn main() {}
