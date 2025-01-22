use array::{da_implementation, dynamic_array, sa_implementation, static_array};
use exceptions::Exceptions;
use hash_table::ht_implementation;
use linked_list::ll_implementation;

fn main() -> Result<(), Exceptions> {
    static_array();
    dynamic_array();
    da_implementation()?;
    sa_implementation()?;
    ll_implementation()?;
    ht_implementation()?;
    Ok(())
}
