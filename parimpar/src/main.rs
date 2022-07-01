//2. Un programa que solicite 6 números y que cuente cuantos son pares e impares.

fn main() {
    let mut _arr:Vec<i32>=Vec::new();//Declaramos i32 para pasarlo a un numero entero
    
    let mut i=0;
    while i<=7{
        println!("Digita un numero: ");
        let mut numero:String=String::new();
        std::io::stdin().read_line(&mut numero).unwrap();
        let mut _numero:i32=numero.trim().parse().unwrap();
        _arr.push(_numero);//del arreglo lo pasamos a agragar numeros ingresados
        i+=1
    }
    //creamos contadores diferente para cada par e impar
    let mut _count_p:i32=0;
    let mut _count_i:i32=0;
    //Usamos for para recorrer la lista de los nuemeros ingresados
    for j in _arr.iter(){
        let res:i32= j % 2;//Operacion modulo % para conocer residuos.
         
        if res == 0{
            _count_p+=1;//contador de pares
        }else if res ==1 {
            _count_i+=1;//Contador de impares
            }
    }
    println!("Total de Pares: {_count_p}");
    println!("Total de Impares: {_count_i}");
    
}
