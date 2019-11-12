use std::mem;
// Implementación de una lista enlazada.
// "Lista" es la interfaz del tipo.
// "Enlace" representa la posible lista vacía o con elementos.
// "Nodo" representa un elemento completo de la lista.

pub struct Lista<T> {
    head: Enlace<T>,
}

type Enlace<T> = Option<Box<Nodo<T>>>;

struct Nodo<T> {
    valor: T,
    resto: Enlace<T>,
}

// este struct se utiliza para implementar into_iter.
// simplemente contiene una lista del mismo tipo.
pub struct IntoIter<T>(Lista<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// este struct implementa Iter.
// Este iterador recibe una referencia inmutable a la colección.
pub struct Iter<'a, T> {
    next: Option<&'a Nodo<T>> }

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|nodo| {
            self.next = nodo.resto.as_ref().map(|nodo| &**nodo);
            &nodo.valor
        })
    }
}

// este struct se usa para implementar iter_mut.
// contiene una referencia mutable al próximo nodo de la lista.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Nodo<T>> }

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|nodo| {
            self.next = nodo.resto.as_ref().map(|nodo| &mut **nodo);
            &nodo.valor
        })
    }
}

// -- -- -- --
// -- -- -- --
impl<T> Lista<T> {
    // crear una lista vacía
    pub fn new() -> Self {
        Lista {head: None}
    }

    // añadir un elemento al principio de la fila
    pub fn push(&mut self, elem: T) {
        let new_node = Nodo {
            valor: elem,
            resto: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    // remover el primer elemento y retornarlo
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|nodo| {
            self.head = nodo.resto;
            nodo.valor
        })
    }

    // a partir de esta función se implementa into_iter
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_ref().map(|nodo| &**nodo) }
    }

    pub fn iter_mut<'a>(&'a self) -> IterMut<'a, T> {
        IterMut { next: self.head.as_ref().map(|nodo| &mut **nodo) }
    }

    // añadir al final de la lista
    // se crea una referencia inmutable a un nodo
    // mientras el siguiente sea no nulo, cambiar a la siguiente
    // cuando se llegue al fin, reemplazarse a sí misma por una referencia
    // al nuevo nodo
    pub fn append(&mut self, elem: T) {
        // sea crea el nuevo nodo
        let new_node = Nodo {
            valor: elem,
            resto: None,
        };
        // si la lista está vacía, agregar de inmediato
        match &mut self.head {
            None => {
                self.head = Some(Box::new(new_node));
            }

            Some(nodo) => {
                // se crea una referencia al nodo siguiente
                let mut noderef = &mut nodo.resto;
                // mientras no sea nula:
                while let Some(siguiente) = noderef {
                    noderef = &mut siguiente.resto;
                }
                // finalmente, en el lugar del nodo siguiente se coloca
                // el nuevo nodo.
                *noderef = Some(Box::new(new_node))
            }
        }
    }

    // eliminar el elemento N de la lista.
    pub fn remove(&mut self, n: u32) {
        // se crea una referencia a un nodo.
        let mut noderef: &mut Nodo<T>;
        for _ in 0..n-1 {

        }
    }

}

// implementando el método de eliminación de la lista
impl<T> Drop for Lista<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut nodo) = cur_link {
            cur_link = nodo.resto.take();
        }
    }
}


//-- -- -- --
// probando que todo funcione.
#[cfg(test)]
mod test {
    use super::Lista;

    #[test]
    fn push_n_pop() {
        let mut list = Lista::new();

        // llenando la lista
        list.push(1);
        list.push(2);
        list.push(3);

        //extrayendo elementos
        let last = list.pop();
        let snd_to_last = list.pop();

        assert_eq!(last, Some(3));
        assert_eq!(snd_to_last, Some(2));

        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn append() {
        let mut list = Lista::new();

        // poniendo valores
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn iter_inmut() {
        let mut list = Lista::new();
        list.append(1); list.append(2); list.append(3);

        let mut iter = Lista::iter(&list);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
    }
}
