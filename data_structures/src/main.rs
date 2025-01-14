use array::{
    dynamic_array, dynamic_array_implementation::Exceptions, da_implementation, static_array,
};

fn main() -> Result<(), Exceptions> {
    static_array();
    dynamic_array();
    da_implementation()?;
    Ok(())
}
