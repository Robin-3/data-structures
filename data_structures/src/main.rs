use array::{aa_implementation, da_implementation, dynamic_array, static_array};
use exceptions::Exceptions;
use linked_list::ll_implementation;

fn main() -> Result<(), Exceptions> {
    static_array();
    dynamic_array();
    da_implementation()?;
    aa_implementation()?;
    ll_implementation()?;
    Ok(())
}
