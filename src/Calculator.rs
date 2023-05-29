use std::io;

fn Addition(x: &mut u32, y: &mut u32) {
    let mut input2 = String::new();
let mut inpu3 = String::new();

println!("Enter X: ");

    io::stdin().read_line(&mut input2).unwrap();

    
    
    
  *x = input2.trim().parse().expect("Invalid number");
    println!("Enter Y:");
    io::stdin().read_line(&mut inpu3).unwrap();
    *y = inpu3.trim().parse().expect("Invalid number");

    println!("X is {} And Y is {} and X + Y = {}", x, y, *x + *y);
}
fn Subtraction(x: &mut u32, b: &mut u32) {
    let mut input2 = String::new();
let mut inpu3 = String::new();
println!("Enter x: ");
    io::stdin().read_line(&mut input2).unwrap();

  

    *x = input2.trim().parse().unwrap();
    println!("Enter b: ");
    io::stdin().read_line(&mut inpu3).unwrap();
    *b = inpu3.trim().parse().unwrap();


println!("X is {} and B is {} and X - B is {}",x,b,*x - *b);

}
fn Multiply(x: &mut u32, y: &mut u32) {
    let mut input2 = String::new();
let mut inpu3 = String::new();
    println!("Enter X: ");

    io::stdin().read_line(&mut input2).unwrap();

    

    
    *x = input2.trim().parse().unwrap();
    println!("Enter y");
    io::stdin().read_line(&mut inpu3).unwrap();
    *y = inpu3.trim().parse().unwrap();

    println!("{} Is X and  {} Is Y and X*Y is {}", x, y, *x * *y);
}

fn Division(x: &mut u32, y: &mut u32) {
    let mut input2 = String::new();
let mut inpu3 = String::new();

println!("Enter X:");

    io::stdin().read_line(&mut input2).unwrap();

    *x = input2.trim().parse().unwrap();
    println!("Enter Y:");
    io::stdin().read_line(&mut inpu3).unwrap();
    *y = inpu3.trim().parse().unwrap();

    if x > y {
        println!("X is {} and Y is {} and them divided is {}", x, y, *x / *y);
    } else {
        println!("X is {} and Y is {} and them divided is {}", x, y, *y / *x);
    }
}

fn main() {
    let mut input = String::new();

    println!("Hello This is a Math Program you will See Those Lists to Do your Operation: ");
    println!(
        "'A': Addition \n
            'S': Subtraction \n
            'M': Multiply \n
            'D': Division \n

"
    );

    io::stdin().read_line(&mut input).unwrap();

    let mut x: u32 = 0;
    let mut y: u32 = 0;

       match &*input.trim() {

        "A" => Addition(&mut x, &mut y),
        "S" => Subtraction(&mut x, &mut y),
        "M" => Multiply(&mut x, &mut y),
        "D" => Division(&mut x, &mut y),
    _ => println!("not an option")

    }

  
}

/* To 
 ! [rejected]        master -> master (non-fast-forward)
error: failed to push some refs to 
hint: Updates were rejected because the tip of your current branch is behind
hint: its remote counterpart. Integrate the remote changes (e.g.
hint: 'git pull ...') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
*/