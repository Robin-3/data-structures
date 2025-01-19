pub fn static_array() {
    println!("Array estático");
    // 1. Initialization
    let mut planets: [Option<&str>; 6] = [None; 6];
    println!("  1. Inicialización:\n    {planets:?}");
    // 2. Assigning values
    planets[0] = Some("Venus");
    planets[1] = Some("Plutón");
    planets[2] = Some("Tierra");
    planets[3] = Some("Jupiter");
    println!("  2. Asignación de valores:\n    {planets:?}");
    // 3. Insertion at the beginning
    // Shift elements right
    for i in (1..planets.len()).rev() {
        planets[i] = planets[i - 1];
    }
    // Assing a new value
    planets[0] = Some("Mercurio");
    println!("  3. Insertar al inicio:\n    {planets:?}");
    // 4. Insertion at an arbitrary position
    let position: usize = 4;
    for i in ((position + 1)..planets.len()).rev() {
        planets[i] = planets[i - 1];
    }
    planets[position] = Some("Marte");
    println!("  4. Insertar en una posición arbitraria (indice: {position}):\n    {planets:?}");
    // 5. Deletion at an arbitrary position
    let position: usize = 2;
    let planet: Option<&str> = planets[position];
    // Shift elements left
    for i in position..(planets.len() - 1) {
        planets[i] = planets[i + 1];
    }
    planets[planets.len() - 1] = None;
    println!("  5. Eliminar de una posición arbitraria: (indice: {position}, planeta: {planet:?})\n    {planets:?}");
}

pub fn dynamic_array() {
    println!("Array dinámico");
    // 1. Initialization
    let mut planets: Vec<&str> = Vec::new();
    println!("  1. Inicialización:\n    {planets:?}");
    // 2. Insertion at the end
    planets.push("Venus");
    planets.push("Plutón");
    planets.push("Tierra");
    planets.push("Jupiter");
    println!("  2. Insertar al final:\n    {planets:?}");
    // 3. Insertion at the beginning
    let position: usize = 0;
    planets.insert(position, "Mercurio");
    println!("  3. Insertar al inicio: (indice: {position})\n    {planets:?}");
    // 4. Insertion at an arbitrary position
    let position: usize = 4;
    planets.insert(position, "Marte");
    println!("  4. Insertar en una posición arbitraria  (indice: {position}):\n    {planets:?}");
    // 5. Deletion at an arbitrary position
    let position: usize = 2;
    let planet: &str = planets.remove(position);
    println!("  5. Eliminar de una posición arbitraria (indice: {position}, planeta: {planet:?})\n    {planets:?}");
}

pub mod dynamic_array;
pub mod static_array;

pub use dynamic_array::DynamicArray;
use exceptions::Exceptions;
pub use static_array::StaticArray;

pub fn da_implementation() -> Result<(), Exceptions> {
    println!("Implementación de un array dinámico");
    // 1. Initialization
    let planets: DynamicArray<&str> = DynamicArray::new(5);
    println!("  1. Inicialización:\n    {planets:?}");
    // 2. Initializacion with values
    let mut planets: DynamicArray<&str> =
        DynamicArray::with_values(5, &["Venus", "Plutón", "Tierra", "Marte"]);
    println!("  2.  Inicialización con valores\n    {planets:?}");
    // 3. Insertion at the ending
    planets.push("Jupiter");
    println!("  3. Insertar al final\n    {planets:?}");
    // 4. Insertion at an arbitrary position
    let position: usize = 0;
    planets.insert(position, "Mercurio")?;
    println!("  4.  Insertar en una posición arbitraria: (index: {position})\n    {planets:?}");
    // 5. Deletion at an arbitrary position
    let position: usize = 2;
    let planet = planets.remove(position)?;
    println!("  5. Eliminar de una posición arbitraria: (index: {position}, planeta: {planet:?})\n    {planets:?}");
    let position: usize = 0;
    let planet = planets.remove(position)?;
    println!("  5. Eliminar de una posición arbitraria: (index: {position}, planeta: {planet:?})\n    {planets:?}");
    Ok(())
}

pub fn sa_implementation() -> Result<(), Exceptions> {
    println!("Implementación de un array estático");
    // 1. Initialization
    let planets: StaticArray<&str> = StaticArray::new(5);
    println!("  1. Inicialización:\n    {planets:?}");
    // 2. Initializacion with values
    let mut planets: StaticArray<&str> =
        StaticArray::with_values(5, &["Venus", "Plutón", "Tierra", "Marte"]);
    println!("  2.  Inicialización con valores\n    {planets:?}");
    // 3. Insertion at the ending
    planets.push("Jupiter")?;
    println!("  3. Insertar al final\n    {planets:?}");
    // 4. Insertion at an arbitrary position
    let position: usize = 0;
    planets.insert(position, "Mercurio")?;
    println!("  4.  Insertar en una posición arbitraria: (index: {position})\n    {planets:?}");
    // 5. Deletion at an arbitrary position
    let position: usize = 2;
    let planet = planets.remove(position)?;
    println!("  5. Eliminar de una posición arbitraria: (index: {position}, planeta: {planet:?})\n    {planets:?}");
    let position: usize = 0;
    let planet = planets.remove(position)?;
    println!("  5. Eliminar de una posición arbitraria: (index: {position}, planeta: {planet:?})\n    {planets:?}");
    Ok(())
}
