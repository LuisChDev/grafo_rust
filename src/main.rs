mod lista;
use lista::Lista;

fn main () {
    let mut new_list: Lista<i32> = Lista::new();
    new_list.push(5);  // Lista::push(new_list, 5);
    let valor = new_list.pop();  // Lista::pop(new_list);
    match valor {
        None => {
            println!("Esta lista está vacía. -_-");
        }
        Some(val) => {
            println!("El primer nodo de esta lista es {}", val);
        }
    }
}
