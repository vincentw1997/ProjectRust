use std::env;
use std::io::{self, Write};
use std::time::SystemTime;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Date};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table, WithTitle};


struct Student {
    ID : i32, //can be immatrikulationsnummer 
    fullname: String, 
    first_name: String, 
    last_name: String,
}

struct Item{
    item_name: String,
    category_name: String,
    serial_number: i64, //can be the IMEI of the item so 15 numbers
    item_ID: String,
    price: f32, //
    unit_of_measurement: String,
    location: String,
    conditions: String,
    remarks: String,
    item_status: bool, //True means available, false means borrowed
    date_added: DateTime<Utc>,
    borrowed_until: DateTime<Utc>,

}

struct Category{
    category_name: String,
    category_ID: String,

}

struct LendOrder{
    action_ID: String,
    current_date: SystemTime,
    student_name: String,
    item_ID: String,
    lending_date_start: DateTime<Utc>,
    lending_date_end: DateTime<Utc>,

}

struct ReturnOrder{
    action_ID: String,
    current_date: SystemTime,
    student_name: String,  
    item_ID: String,
    lending_date_start: DateTime<Utc>,
    lending_date_end: DateTime<Utc>,
    return_date: SystemTime,
    condition: String,
    remarks: String,

}

struct InventoryList {
    item_ID: String,
    item_name: String,
    item_quantity: i32,
    status_available: i32,
    status_borrowed: i32,
}

fn main() {

    println!("Laboratory Inventory Management");
    println!("-----------------------");
    println!("1. Item");
    println!("2. List of Students");
    println!("3. Lend item");
    println!("4. Return item");
    println!("Press the corresponding numbers for the function:");


    let mut home_input = String::new();
    let stdin = io::stdin();
    let mut counter = 0;
    
        
    while counter < 1 {
        
        stdin.read_line(&mut home_input).unwrap();
        let trimmed = home_input.trim();

        if trimmed.parse::<i8>() == Ok(1) {
            println!("your input {} is equal to 1", trimmed);
            println!("1. Add New Item");
            println!("2. Add New Category");
            println!("3. Add New UoM");
            println!("4. Master of Item");
            let mut counter_item = 0;
            let mut item_input = String::new();
            stdin.read_line(&mut item_input).unwrap();
            let trimmed_item = item_input.trim();
            if trimmed_item.parse::<i8>() == Ok(1){
                println!("your input {} is equal to 1", trimmed_item);
            }
            else if trimmed_item.parse::<i8>() == Ok(2) {
                println!("your input {} is equal to 2", trimmed_item);
            }
            else if trimmed_item.parse::<i8>() == Ok(3) {
                println!("your input {} is equal to 3", trimmed_item);
            }
            else if trimmed_item.parse::<i8>() == Ok(4) {
                println!("your input {} is equal to 4", trimmed_item);
                println!("Master of item");
                println!("----------------");
                println!("1. Search");
                println!("2. Add item");

                let now = Utc::now();

                let mut item_1 = Item {
                    item_name: "Monitor".to_string(),
                    category_name: "Hardware".to_string(),
                    serial_number: 123456789101112, //can be the IMEI of the item so 15 numbers
                    item_ID: "blank".to_string(),
                    price: 200.00, //
                    unit_of_measurement: "pcs".to_string(),
                    location: "A".to_string(),
                    conditions: "Good".to_string(),
                    remarks: "".to_string(),
                    item_status: true, //True means available, false means borrowed
                    date_added: now,
                    borrowed_until: now,
                };

                let table = vec![
                    vec![item_1.item_name.cell()],
                    vec!["empty".cell()],
                ]
                .table()
                .title(vec![
                    "Name of Item".cell().bold(true),
                ])
                .bold(true);

                assert!(print_stdout(table).is_ok());

                let mut moi_input = String::new();
                stdin.read_line(&mut moi_input).unwrap();
                let trimmed_moi = moi_input.trim();

                if trimmed_moi.parse::<i8>() == Ok(1){
                    println!("this is the search engine: {}", trimmed_moi);
                }
                else if trimmed_moi.parse::<i8>() == Ok(2){
                    println!("this is the function to add item: {}", trimmed_moi);

                }
                else {
                    println!("wrong input");
                    continue;
                }





            }



            counter += 1;
        }
        
        else if trimmed.parse::<i8>() == Ok(2) {
            println!("your input {} is equal to 2", trimmed);
            counter += 1;
        } 
    
        else if trimmed.parse::<i8>() == Ok(3) {
            println!("your input {} is equal to 3", trimmed);
            counter += 1;
        }

        else if trimmed.parse::<i8>() == Ok(4) {
            println!("your input {} is equal to 4", trimmed);
            counter += 1;
        }
        
        else {
            println!("wrong input, please reenter number");
            counter = 0;
            home_input.clear(); //this is important to clear the user input
            continue;
        }
        

    
    }


}

fn Addinventory() {




}