/// The compiler generates code only in the `generator` module.
/// Therefore, all transformation of ASN.1 identifiers are specific
/// to a particular `Backend` implementation that the compiler uses.
/// For example, the `Rasn` backend unnests inner types and names
/// them with a specific prefix. 
/// `INTERNAL_NESTED_TYPE_NAME_PREFIX` is a prefix that is prepended
/// to stringified type names of nested types (with the exception of
/// item types of array-like types) internally, so that they can be properly
/// identified by the individual compiler backends.
pub const INTERNAL_NESTED_TYPE_NAME_PREFIX: &str = "INNER$";
/// `INTERNAL_ITEM_TYPE_NAME_PREFIX` is a prefix that is prepended
/// to stringified type names of array-like types' item types, so that they 
/// can be properly identified by the individual compiler backends.
pub const INTERNAL_ITEM_TYPE_NAME_PREFIX: &str = "ITEM$";