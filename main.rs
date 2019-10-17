  
   use std::io;
   fn main(){
      println!("Please enter x =");
            let mut x = String::new();
            io::stdin().read_line(&mut x);
            let x:f32= x.trim().parse().unwrap();

      println!("Please enter y =");
            let mut y = String::new();
            io::stdin().read_line(&mut y);
            let y:f32= y.trim().parse().unwrap();
            
            println!("{}", x+y);
            println!("{}", x-y);
            println!("{}", x/y);
            println!("{}", x*y);
            println!("{}", x*x);
            println!("{}", x*x*x);
            println!("{}",x.powf(y));

   }


