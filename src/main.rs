mod bill;
mod coffee;
mod utils;

use std::io::{BufRead, StdinLock};

use bill::{Bill, Order};
use coffee::Coffee;
use utils::{give_details_of_output, prompt_user};
use crate::utils::{read_numeral_input, read_string_input};

fn main() {
    // create some coffee
    let coffees = Coffee::create_basic_coffees();
    // print the details of each coffee
    for coffee in coffees.iter() {
        println!("{}", coffee.get_formatted_details());
    }
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    // read the id of the coffee
    give_details_of_output(&"Enter the id of the coffee you want to order:".to_string());

    give_details_of_output(&"1 to 10  : To Select coffee".to_string());
    give_details_of_output(&"q        : To quit".to_string());
    give_details_of_output(&"b        : To show bill".to_string());
    give_details_of_output(&"cn       : To cancel an order".to_string());
    give_details_of_output(&"cq       : To change the quantity of an order".to_string());

    let mut ordered_coffees: Vec<Order> = Vec::new();
    let mut bill;
    loop {
        prompt_user("Enter the id of the coffee you want to order:");
        let coffee_selection_input = read_coffee_selection_loop(&mut handle);


        match coffee_selection_input {
            CoffeeIdInput::Quit => {
                break;
            }
            CoffeeIdInput::Bill => {
                bill = Bill::new(&ordered_coffees);
                bill.show_bill();
            }
            CoffeeIdInput::Invalid => {
                continue;
            }
            CoffeeIdInput::Id(id) => {
                let selected_coffee = get_selected_coffee_details(&coffees, &id).unwrap();
                prompt_user("How many do you want to order?");
                let quantity = read_quantity(&mut handle);
                let total_cost = selected_coffee.get_price() as u32 * quantity as u32;
                ordered_coffees.push(Order {
                    coffee: selected_coffee,
                    quantity,
                    total_cost,
                });
            }
            CoffeeIdInput::Cancel => {
                handle_cancel(&mut ordered_coffees, &mut handle);
            }
            CoffeeIdInput::ChangeQuantity => {
                handle_change_quantity(&mut ordered_coffees, &mut handle);
            }
        }

    }
    // final bill
    bill = Bill::new(&ordered_coffees);
    bill.show_bill();
    let cash = bill.get_cash(&mut handle);
    give_details_of_output(&format!("Exchange \t:\t${}", cash.exchange));
}

fn get_selected_coffee_details<'c>(
    coffees: &'c [Coffee; 10],
    coffee_id: &str,
) -> Option<&'c Coffee> {
    let selected_coffee = coffees.iter().find(|coffee| coffee.get_id() == coffee_id);
    // return pointer to the selected coffee
    selected_coffee
}

enum CoffeeIdInput{
    Id(String),
    Quit,
    Bill,
    Cancel,
    ChangeQuantity,
    Invalid,
}
fn read_coffee_selection_loop(handle: &mut StdinLock) -> CoffeeIdInput {
    let mut coffee_id = String::new();
    handle.read_line(&mut coffee_id).unwrap();
    match  coffee_id.trim() {
        "q" => CoffeeIdInput::Quit,
        "b" => CoffeeIdInput::Bill,
        "cn" => CoffeeIdInput::Cancel,
        "cq"=> CoffeeIdInput::ChangeQuantity,
        _ => {
            // if num is in 1 to 10
            if let Ok(num) = coffee_id.trim().parse::<u8>() {
                if num > 0 && num < 11 {
                    CoffeeIdInput::Id(num.to_string())
                } else {
                    CoffeeIdInput::Invalid
                }
            } else {
                CoffeeIdInput::Invalid
            }
        }
    }
}

fn read_quantity(handle: &mut StdinLock) -> u8 {
    let mut quantity = String::new();
    // read the quantity if empty string is entered, default to 1
    handle.read_line(&mut quantity).unwrap();
    let quantity = quantity.trim().parse::<u8>().unwrap_or(1);
    quantity
}

fn handle_cancel(orders: &mut Vec<Order>, handle: &mut StdinLock) {
    prompt_user("Enter the position of the order you want to cancel");
    let pos = read_numeral_input(handle) as usize;
    prompt_user(&format!("Do you want to cancel the order {} ? (y/n)", pos));
    let mut cancel = read_string_input(handle);
    if cancel.trim() == "y" {
        orders.remove(pos);
    }
}

fn handle_change_quantity(orders: &mut Vec<Order>, handle: &mut StdinLock) {
    prompt_user("Enter the position of the order you want to cancel");
    let pos = read_numeral_input(handle) as usize;
    prompt_user(&format!("Enter the new quantity for the order {} ", pos));
    let new_quantity = read_numeral_input(handle);
    orders[pos].quantity = new_quantity;
    orders[pos].total_cost = orders[pos].coffee.get_price() as u32 * new_quantity as u32;
}

