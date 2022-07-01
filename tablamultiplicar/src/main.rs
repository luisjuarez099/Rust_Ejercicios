//4. Solicite un valor y muestre la tabla de multiplicar de dicho número.

fn main() {
    let mut _arr=[1,2,3,4,5,6,7,8,9,10,11,12];
    let mut _res:i8=0;
    println!("Digita un numero: ");
    let mut num:String=String::new();
    std::io::stdin().read_line(&mut num).unwrap();
    let mut _num:i8=num.trim().parse().unwrap();
    for i in _arr.iter(){
        _res=_num*i;
        println!("{i} x {_num} = {_res}");
    }
}
