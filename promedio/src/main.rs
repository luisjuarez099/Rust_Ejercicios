//3. Diseñe un programa que solicite 10 números enteros, calcule el promedio de los mismos.

fn main() {
    //Declaraciones
    let mut _arr:Vec<i32>=Vec::new();
    let mut i=0;
    let mut sum:i32=0;
    //ciclo
    while i<=9{
        println!("Digita un numero {i}: ");
        let mut numero:String=String::new();
        std::io::stdin().read_line(&mut numero).unwrap();
        let mut _numero:i32=numero.trim().parse().unwrap();
        _arr.push(_numero);
        i+=1;
    }println!("{:?}",_arr);
    
    for j in _arr.iter(){
       sum+=j;// sumatoria que es igual a : suma=suma + j
    }
    println!("Sumatoria Final: {sum}");
    println!("Promedio Final: {:?}",sum/10);
   

    
}
 