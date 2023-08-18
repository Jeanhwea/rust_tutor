// When attributes apply to a whole crate, their syntax is
#![crate_attribute]
// and when they apply to a module or item, the syntax is
#[item_attribute]
// (notice the missing bang !).

// Attributes can take arguments with different syntaxes:
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
