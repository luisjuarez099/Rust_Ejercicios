/*1. Solicitar los primeros 5 números enteros, almacene en un array y ordene en forma
ascendentes (abajo hacia arriba, en este caso del ultimo al primero)
*/

fn main() {
    let mut _numero:Vec<String>=Vec::new();
   
    for _j in 0..5{
        println!("Digita un numero: {_j}");
        let mut num:String=String::new();
        std::io::stdin().read_line(&mut num).unwrap();
        let _digito:i32=num.trim().parse().unwrap();
        _numero.push(num);
    }
    println!("Descendente: {:?}",_numero);
    _numero.reverse();//Imprime al reverso
    println!("Ascendente: {:?}",_numero);
   
}
