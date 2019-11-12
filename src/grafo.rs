use lista::Lista;
// definición de los grafos.
// se implementan como una lista de adyacencia.lista::Lista;
// Es decir, una lista de nodos, con sus respectivos arcos.

// Cada elemento de la lista es un struct que contiene el elemento y una lista
// de todos los aristas que salen de él.
struct Vertice<T, P> {
    valor: T,
    aristas: Lista<Arista<T, P>>,
}

// Cada elemento de la lista de aristas es un struct que contiene el vértice
// destino y su peso.
struct Arista<T, P> {
    destino: T,
    peso: P,
}

// interfaz del grafo - un puntero al primer vértice.
pub struct Grafo<T, P> {
    grafo: Lista<Vertice<T, P>>,
}

// métodos del grafo.
impl<T, P> Grafo<T, P> {

    // crear un grafo vacío
    pub fn new() -> Self {
        Grafo { Lista::new() }
    }

    // agregar un vértice
    pub fn add_vertex(&mut grafo: Grafo, vertice: T) {
        // simplemente se agrega a la lista.
        Lista::push(&mut grafo.grafo, vertice);
    }

    // eliminar un vértice
    pub fn rm_vertex(&mut grafo: Grafo, vertice: T) {
        // Se eliminan todos los aristas que referencian este vértice.

    }

    // agregar un arista

    // eliminar un arista

    // mostrar los vértices y aristas

    // búsqueda por profundidad

    // búsqueda por anchura

    // árbol recubridor mínimo
}

// -- -- -- --
#[cfg(test)]
mod test {
    use super::Grafo;

    // agregar y eliminar vértices.
    #[test]
}
