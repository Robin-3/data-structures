pub mod singly_linked_list;

use exceptions::Exceptions;
pub use singly_linked_list::SinglyLinkedList;

pub fn ll_implementation() -> Result<(), Exceptions> {
    println!("Lista enlazada");
    let slice = &["Venus"];
    let list: SinglyLinkedList<&str> = SinglyLinkedList::from(slice);
    println!(
        "  1.1 Inicialización (está vacío: {0}):\n    {list:?}",
        list.is_empty()
    );
    let mut list: SinglyLinkedList<&str> = SinglyLinkedList::new();
    println!(
        "  1.2 Inicialización (está vacío: {0}):\n    {list:?}",
        list.is_empty()
    );
    list.unshift("Saturno");
    println!("  2.1 Insertar al inicio (en blanco):\n    {list:?}");
    list.unshift("Plutón");
    println!("  2.2 Insertar al inicio:\n    {list:?}");
    let pred_value = "Plutón";
    list.insert_after(pred_value, "Marte")?;
    println!("  3.1 Después de un valor (predecesor: {pred_value}):\n    {list:?}");
    let position = 2;
    list.insert(position, "Jupiter")?;
    println!("  3.2 Después en una posición (index: {position}):\n    {list:?}");
    list.push("Urano");
    println!("  4. Insertar al final:\n    {list:?}");
    let position = 0;
    list.set(position, "Tierra")?;
    let planet = list.get(position)?;
    println!("  5. Obtener y establecer data en una posición (index: {position}, valor: {planet:?}):\n    {list:?}");
    let planet = list.shift()?;
    println!("  6. Eliminar el primer elemento (valor: {planet:?}):\n    {list:?}");
    let planet = list.pop()?;
    println!("  7. Eliminar el ultimo elemento (valor: {planet:?}):\n    {list:?}");
    let position = 1;
    let planet = list.remove(position)?;
    println!("  8. Eliminar en una posición (index: {position}, valor: {planet:?}):\n    {list:?}");
    Ok(())
}
