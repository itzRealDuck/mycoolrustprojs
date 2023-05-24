
fn ShowBalance(balance: &mut u128) {


    println!("{}", balance);
}
fn Deposit(balance: &mut u128, depositamount: &mut u128) {
    *balance = *balance + *depositamount;

    println!(
        "you Deposited {} and now your balance is {}",
        depositamount, balance
    );

    if balance < depositamount {
        panic!("You are depositing more than you have");
    }
}
fn Withdraw(balance: &mut u128, withdrawamount: &mut u128) {
    *balance = *balance - *withdrawamount;

    println!(
        "You Withdrew {} and the Balance is {} ",
        withdrawamount, balance
    );


if *withdrawamount > 0 { 

panic!("you cant withdraw with negative amount ");


}
}

fn main() {

let mut input = String::new();

    let mut balance: u128 = 1300;
    ShowBalance(&mut balance);

let mut depositamount: u128 = 0;



Deposit(&mut balance, &mut depositamount);

let mut withdrawamount: u128 = 0;

Withdraw(&mut balance, &mut withdrawamount);


Deposit(&mut balance, &mut depositamount);
}
