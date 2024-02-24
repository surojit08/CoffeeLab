use std::io::{BufRead, StdinLock};
use crate::coffee::Coffee;
use crate::utils::{give_details_of_output, prompt_user, warn_user};

pub struct Cash {
    pub(crate) exchange: u32,
}
impl Cash {
    pub fn get_cash(handle: &mut StdinLock, final_cost: &u32) -> Self {
        let mut cash = String::new();
        let mut cash_as_number: u32;
        loop {
            cash.clear();
            handle.read_line(&mut cash).unwrap();
            cash_as_number = cash.trim().parse::<u32>().unwrap();
            if cash_as_number < *final_cost {
                warn_user("Cash is less than the cost of the coffee. Please enter a valid amount");
                prompt_user("Please submit cash");
                continue;
            }
            break;
        }
        // this could overflow.
        let exchange: u32 = cash_as_number - *final_cost;
        Cash { exchange }
    }

}


pub struct Order<'a> {
    pub(crate) coffee: &'a Coffee,
    pub(crate) quantity: u8,
    pub(crate) total_cost: u32,
}

// bill is a struct of reference to vector of orders
pub struct Bill<'a> {
    orders: &'a Vec<Order<'a>>,
    total_cost: u32,
}
impl<'a> Bill<'a> {
    pub fn new(orders: &'a Vec<Order>) -> Bill<'a> {
        let mut total_cost: u32 = 0;
        for order in orders.iter() {
            total_cost += order.total_cost;
        }
        Bill { orders, total_cost }
    }
    pub fn get_cash(&self, handle: &mut StdinLock) -> Cash {
        prompt_user("Please submit cash");
        Cash::get_cash(handle, &self.total_cost)
    }
    pub fn show_bill(&self) {
        give_details_of_output(&"   Coffee\t\t----\tQuantity\t----\tCost".to_string());
        for (inx,order) in self.orders.iter().enumerate() {
            let m = format!(
                "({0})\t{1}",
                inx,
                self.format_single_order(order)
            );
            give_details_of_output(&m);
        }
        give_details_of_output(&format!("\t\t\t\tTotal cost :\t${}", self.total_cost));
    }

    pub fn format_single_order(&self, order: &Order) -> String {
        format!(
            "{0}\t----\t{1} coffee(s)\t----\t${2} ",
            order.coffee.get_name(),
            order.quantity,
            order.total_cost,
        )
    }
}
