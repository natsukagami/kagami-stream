use std::error::Error;

/**
 * The result type of this crate.
 */
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn boxed<'a, T: Error + 'a>(e: T) -> Box<dyn Error + 'a> {
	Box::new(e)
}
