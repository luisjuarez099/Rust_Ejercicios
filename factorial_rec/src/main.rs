fn factorial(num: u32)->u32{
    if num == 0{
        return 1;
    }
    else{
        return num * (factorial(num - 1));
    }
}

fn main() {
    println!("Resultado: {:?}",factorial(6));
}
