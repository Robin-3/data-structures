mod separate_chaining_hash_table;

use exceptions::Exceptions;
pub use separate_chaining_hash_table::SeparateChainingHashTable;

pub fn ht_implementation() -> Result<(), Exceptions> {
    println!("Tabla hash con encadenamiento separado");
    let mut table = SeparateChainingHashTable::new(4);
    println!("  1. Creación en blanco:\n    {table:?}");
    table.insert("00", "Cien")?;
    println!("  2. Ingresar datos:\n    {table:?}");
    table.set("00", "Cero")?;
    println!("  3. Modificar datos:\n    {table:?}");
    table.remove("00")?;
    let is_empty = table.is_empty();
    let buckets_len = table.buckets_len();
    println!("  4. Eliminar datos (está vacío: {is_empty}, buckets en uso: {buckets_len})\n    {table:?}");
    table.insert("01", "Uno")?;
    table.insert("10", "Diez")?;
    println!("  5. Colisiones:\n    {table:?}");
    Ok(())
}
