pub mod env;

use tera::Context;

/// Variables stored and accessible during execution, based on [`tera::Context`]
///
/// [`tera::Context`]: ../../tera/struct.Context.html
pub type Facts = Context;

#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
pub fn test_example() -> Facts {
    Context::from_serialize(
        [("foo", "boo"), ("xuu", "zoo")]
            .iter()
            .cloned()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<String, String>>(),
    )
    .unwrap()
}
