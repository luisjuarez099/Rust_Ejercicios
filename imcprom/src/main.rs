/*5. Solicite el peso de N personas, calcule el peso promedio de las mismas. Calcular
cuantas personas tienen el peso menor al promedio y cuantas tienen el peso mayor al
promedio.*/

fn main() {
    let mut _arr:Vec<i16>=Vec::new();
    let mut _prompeso:i16=0;
    let mut _sum:i16=0;
    let mut i=1;
    while i<=5{
        println!("Digite el IMC {i}: ");
        let mut nump:String=String::new();
        std::io::stdin().read_line(&mut nump).unwrap();
        let mut _nump:i16=nump.trim().parse().unwrap();
        _arr.push(_nump);
        _sum+=_nump;
        i+=1;
    }
    println!("Suma de pesos (Kg): {:?}",_sum);
    _prompeso=_sum/5;
    println!("Promedio de Peso (kg): {:?}",_prompeso);
    if _prompeso >= 5{
        println!("Peso Mayor al promedio");
    }else{
        println!("Peso Menor al promedio");
    }
}
