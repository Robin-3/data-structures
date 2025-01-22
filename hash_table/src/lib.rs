mod separate_chaining_hash_table;

use exceptions::Exceptions;
pub use separate_chaining_hash_table::SeparateChainingHashTable;

pub fn ht_implementation() -> Result<(), Exceptions> {
    println!("Tabla hash con encadenamiento separado");
    let mut table = SeparateChainingHashTable::new(6);
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
    let buckets_len = table.buckets_len();
    println!("  5. Colisiones (buckets en uso: {buckets_len}):\n    {table:?}");
    table.insert("02", "Dos")?;
    table.insert("03", "Tres")?;
    table.insert("04", "Cuatro")?;
    table.insert("05", "Cinco")?;
    table.insert("06", "Seis")?;
    table.insert("07", "Siete")?;
    table.insert("08", "Ocho")?;
    table.insert("09", "Nueve")?;
    table.insert("11", "Once")?;
    table.insert("12", "Doce")?;
    table.insert("13", "Trece")?;
    table.insert("14", "Catorce")?;
    table.insert("15", "Quince")?;
    table.insert("16", "Dieciséis")?;
    table.insert("17", "Diecisiete")?;
    table.insert("18", "Dieciocho")?;
    table.insert("19", "Diecinueve")?;
    table.insert("20", "Veinte")?;
    table.insert("21", "Veintiuno")?;
    table.insert("22", "Veintidós")?;
    table.insert("23", "Veintitrés")?;
    table.insert("24", "Veinticuatro")?;
    table.insert("25", "Veinticinco")?;
    table.insert("26", "Veintiséis")?;
    table.insert("27", "Veintisiete")?;
    table.insert("28", "Veintiocho")?;
    table.insert("29", "Veintinueve")?;
    table.insert("30", "Treinta")?;
    table.insert("31", "Treinta y uno")?;
    table.insert("32", "Treinta y dos")?;
    table.insert("33", "Treinta y tres")?;
    table.insert("34", "Treinta y cuatro")?;
    table.insert("35", "Treinta y cinco")?;
    table.insert("36", "Treinta y seis")?;
    /*
        [06, 15, 24, 33]
        [01, 10, 07, 16, 25, 34]
        [02, 08, 11, 17, 20, 26, 35]
        [03, 09, 12, 18, 21, 27, 30, 36]
        [04, 13, 19, 22, 28, 31]
        [05, 14, 23, 29, 32]
     */
    println!("  6.1 Rehashing (inicial):\n    {table:?}");
    table.rehashing(3);
    /*
        [06, 15, 24, 33, 03, 09, 12, 18, 21, 27, 30, 36]
        [01, 10, 07, 16, 25, 34, 04, 13, 19, 22, 28, 31]
        [02, 08, 11, 17, 20, 26, 35, 05, 14, 23, 29, 32]
     */
    println!("  6.2 Rehashing (decremento):\n    {table:?}");
    table.rehashing(9);
    /*
        [03, 12, 21, 30]
        [04, 13, 22, 31]
        [05, 14, 23, 32]
        [06, 15, 24, 33]
        [07, 16, 25, 34]
        [08, 17, 26, 35]
        [09, 18, 27, 36]
        [01, 10, 19, 28]
        [02, 11, 20, 29]
     */
    println!("  6.2 Rehashing (incremento):\n    {table:?}");
    Ok(())
}
