pub mod linked_list;

use exceptions::Exceptions;
use linked_list::LinkedList;

pub fn ll_implementation() -> Result<(), Exceptions> {
    println!("Lista enlazada");
    let list: LinkedList<&str> = LinkedList::with_data("Venus");
    println!(
        "  1.1 Inicialización (está vacío: {0}):\n    {list:?}",
        list.is_empty()
    );
    let mut list: LinkedList<&str> = LinkedList::new();
    println!(
        "  1.2 Inicialización (está vacío: {0}):\n    {list:?}",
        list.is_empty()
    );
    list.insert_first("Saturno");
    println!("  2.1 Insertar al inicio (en blanco):\n    {list:?}");
    list.insert_first("Plutón");
    println!("  2.2 Insertar al inicio:\n    {list:?}");
    let pred_value = "Plutón";
    list.insert_after(pred_value, "Marte")?;
    println!("  3.1 Después de un valor (predecesor: {pred_value}):\n    {list:?}");
    let position = 2;
    list.insert_index(position, "Jupiter")?;
    println!("  3.2 Después en una posición (index: {position}):\n    {list:?}");
    list.insert_last("Urano");
    println!("  4. Insertar al final:\n    {list:?}");
    let position = 0;
    list.set(position, "Tierra")?;
    let planet = list.get(position)?;
    println!("  5. Obtener y establecer data en una posición (index: {position}, valor: {planet:?}):\n    {list:?}");
    Ok(())
}
