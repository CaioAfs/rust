use std::io;

fn convert_to_int(data_input: &str) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn main(){


    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Error ao ler o valor 1 informado");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Error ao ler o valor 2 informado");

    let convert1 = convert_to_int(&number1);
    let convert2 = convert_to_int(&number2);

    if convert1 > convert2 {
        println!("O numero {} é maior", convert1);
    }else{
        println!("O numero {} é maior", convert2);
    }


}