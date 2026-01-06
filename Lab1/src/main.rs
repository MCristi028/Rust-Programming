//Problema 1
/*fn main() {
    for n in 0..=100{
        if isprime(n){
            println!("{}", n);
        }
    }
}

fn isprime(n: u32) -> bool{
    if(n < 2){
        return false;
    }
    if(n == 2)
    {
        return true;
    }
    if(n % 2 == 0)
    {
        return false;
    }

    for i in (3..=((n as f64).sqrt() as u32)).step_by(2){
        if n % i == 0{
            return false;
        }
    }
    return true;
}*/

//Problema 2
/*fn main(){
    for a in 0..=100{
        for b in 0..=100{
            if coprime(a,b){
                println!("{} si {} sunt coprime",a,b);
            }
        }
    }
}

fn cmmdc(mut a: u32, mut b: u32) -> u32 {
    while b != 0
    {
        let aux = b;
        b = a % b;
        a = aux;
    }
    return a;
}

fn coprime(a: u32, b: u32) -> bool{
    cmmdc(a,b) == 1
}*/

//Problema 3
fn main() {
    for n in (1..=99).rev() {
        if n > 1 {
            println!("{} bottles of beer on the wall,", n);
            println!("{} bottles of beer.", n);
            println!("Take one down, pass it around,");
            if n - 1 == 1 {
                println!("{} bottle of beer on the wall.\n", n - 1);
            } else {
                println!("{} bottles of beer on the wall.\n", n - 1);
            }
        } else {
            println!("1 bottle of beer on the wall,");
            println!("1 bottle of beer.");
            println!("Take one down, pass it around,");
            println!("No bottles of beer on the wall.\n");
        }
    }
}
