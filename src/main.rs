use std::io;

fn verificar_primo(valor2: &i32) -> i32{
    let mut resultado: i32 = 0;
    for i in 1..valor2+1 {
        let rest: i32 = valor2 % i;
        println!("Resto da divisão: {rest}");
        if rest == 0{
            println!("O resto da divisão é igual a zero");
            resultado = resultado+1;
        }
    }
    resultado
}

fn main() {
    
    let mut  guess = String::new();
    println!("Digite um valor para saber se é um número primo:");
        
    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler o valor digitado");
    
    let valor: i32 = guess
        .trim()
        .parse()
        .expect("Falha ao ler o número digitado");
    
    println!("Número escolhido: {valor}");

    if valor <2 {
        println!("Não é numero primo: {valor}");
    }else {
        let resultado2: i32 = verificar_primo(&valor);
        if resultado2 == 2 {
            println!("{valor} é um número primo"); 
        }else {
            println!("{valor} não é um número primo"); 
        }  
    }

}
