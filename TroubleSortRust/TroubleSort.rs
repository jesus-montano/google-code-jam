use std::io;
use std::str::FromStr;
fn main() {
    let mut T=0;
    let mut N=0;
    while T<1 ||T>20{
        println!("ingrese cantidad de casos de prueba entre 1 y 20");
         let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let inp: u32 = u32::from_str(&input.trim()).unwrap();
        T=inp;
       }
       while N<3 || N>10u32.pow(9){
           println!("ingrese cantidad de numeros en su arreglo");
           let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let inp: u32 = u32::from_str(&input.trim()).unwrap();
        N=inp;
       }
       let mut V :Vec<u32>= Vec::new();
       for i in 0..N{
          println!("ingrese Numero");
           let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let inp: u32 = u32::from_str(&input.trim()).unwrap();
        V.push(inp); 
       }
        let  v2=sort(V);
        check(v2);
}
fn sort( mut v: Vec<u32>)-> Vec<u32>{
    let mut done= false;
    while !done{
        done=true;
        for i in 0..v.len()-2{
            if v[i]>v[i+2]{
                done = false;
                let aux;
                aux= v[i];
                v[i]=v[i+2];
                v[i+2]=aux;
            }
        }
    }
   v 
}
fn check(v: Vec<u32>){
    let mut flag=false;
        for i in 0.. v.len()-1{
            if v[i]>v[i+1]{
            println!("{}",i);
            flag=true;
            break;        
            }
        }
        if!flag{
            println!("ok");
        }
}