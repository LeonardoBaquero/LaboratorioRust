

fn main() {
    
  let mut x = 5;
    
    let r = &x;  // ┐ r nace
    println!("Valor de r: {}", r);  // ┘ r muere (último uso)
    
    // r ya NO existe
    
    let r2 = &mut x;  // ✅ OK (r ya murió)
    *r2 += 1;

    println!("Valor de r2: {}", r2); 
 
}

//println!("{}",r); 



/*      */


  