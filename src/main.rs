#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // Este será nuestra clave (key)
}

struct Nodo {
    vuelo: Vuelo,
    // El proposito de usar Nodo encapsulado dentro de un box para que Rust no interprete
    // el struct como si tuviese un tamaño infinito; box lo regula vinculandolo al tamaño
    // de memoria en tiempo de compilación
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}


fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    // take extrae el valor dentro del nodo izquierdo y en su lugar deja un valor de tipo None
    // y asigna el valor extraído a la variable especificada (en este caso la variable mutable x)
    let mut x = y.izquierdo.take().expect("Error de radar");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// --- FUNCIÓN DE INSERCIÓN ---

fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let altitudVuelo = vuelo.altitud;
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    if altitudVuelo < nodo.vuelo.altitud {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo));
    } else if altitudVuelo > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo));
    } else {
        return nodo; 
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && altitudVuelo < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && altitudVuelo > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && altitudVuelo > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && altitudVuelo < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }
    
    nodo
}
// ==================== Métodos pedidos por el parcial ================================ //
fn buscar_vuelo(nodo: &Option<Box<Nodo>>, altitud: u32) -> Option<&Vuelo> {
    match nodo.as_ref() {
        None => None,
        
        Some(nodo_interno) => {
            if altitud == nodo_interno.vuelo.altitud {
                Some(&nodo_interno.vuelo)
            } else if altitud < nodo_interno.vuelo.altitud {
                buscar_vuelo(&nodo_interno.izquierdo, altitud)
            } else {
                buscar_vuelo(&nodo_interno.derecho, altitud)
            }
        }
    }
}

fn eliminar_vuelo(nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> {
    let mut nodo = match nodo_opt {
        None => return None,
        Some(n) => n,
    };

    if altitud < nodo.vuelo.altitud {
        nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), altitud);
    } else if altitud > nodo.vuelo.altitud {
        nodo.derecho = eliminar_vuelo(nodo.derecho.take(), altitud);
    } else {
        if nodo.izquierdo.is_none() {
            return nodo.derecho; 
        } else if nodo.derecho.is_none() {
            return nodo.izquierdo; 
        }

        let mut predecesor = nodo.izquierdo.as_ref().unwrap();
        while let Some(ref der) = predecesor.derecho {
            predecesor = der;
        }

        let vuelo_predecesor = predecesor.vuelo.clone();

        nodo.vuelo = vuelo_predecesor;

        nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), nodo.vuelo.altitud);
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);
    if balance > 1 {
        if obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
            return Some(rotar_derecha(nodo));
        } else {
            let hijo_izq = nodo.izquierdo.take().unwrap();
            nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
            return Some(rotar_derecha(nodo));
        }
    }
    if balance < -1 {
        if obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
            return Some(rotar_izquierda(nodo));
        } else {
            let hijo_der = nodo.derecho.take().unwrap();
            nodo.derecho = Some(rotar_derecha(hijo_der));
            return Some(rotar_izquierda(nodo));
        }
    }

    Some(nodo)
}

fn vuelo_menor_altitud(nodo: &Option<Box<Nodo>>) -> Option<&Vuelo> {
    match nodo.as_ref() {
        None => None,
        
        Some(nodo_interno) => {
            if nodo_interno.izquierdo.is_none() {
                Some(&nodo_interno.vuelo)
            } else {
                vuelo_menor_altitud(&nodo_interno.izquierdo)
            }
        }
    }
}

fn main() {
    let mut radar: Option<Box<Nodo>> = None;
    
    // Simulación de entrada de vuelos
    let datos = vec![
        ("AV123", 5000), ("UA456", 3000), ("IB101", 2000),
        ("AF999", 4000), ("TA222", 3500), ("AM777", 6000),
    ];

    for (id, alt) in datos {
        let v = Vuelo { id: id.to_string(), altitud: alt };
        radar = Some(insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");
    // Aquí el estudiante debe invocar sus funciones de búsqueda y eliminación
    println!("\n--- [PRUEBA 1] Vuelo más cercano a tierra ---");
    match vuelo_menor_altitud(&radar) {
        Some(vuelo) => println!("-> Éxito: El avión más bajo es {} a {} pies.", vuelo.id, vuelo.altitud),
        None => println!("-> El radar está vacío."),
    }
    println!("\n--- [PRUEBA 2] Buscando un vuelo específico ---");
    let altitud_busqueda = 3500;
    match buscar_vuelo(&radar, altitud_busqueda) {
        Some(vuelo) => println!("-> Éxito: Encontrado el vuelo {} a {} pies.", vuelo.id, vuelo.altitud),
        None => println!("-> No se encontró ningún avión a {} pies.", altitud_busqueda),
    }
    println!("\n--- [PRUEBA 3] Eliminando un vuelo (Aterrizaje) ---");
    let altitud_eliminar = 3000; 
    println!("Removiendo el vuelo de la altitud {}...", altitud_eliminar);
    radar = eliminar_vuelo(radar.take(), altitud_eliminar);

    match buscar_vuelo(&radar, altitud_eliminar) {
        Some(vuelo) => println!("-> Error: El vuelo {} sigue en el radar.", vuelo.id),
        None => println!("-> Éxito: El avión a {} pies ha sido removido y el árbol se rebalanceó.", altitud_eliminar),
    }

}
