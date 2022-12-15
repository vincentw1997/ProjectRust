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
    location: String,
    conditions: String,
    remarks: String,
    item_status: String, //new, used but Ok, broken
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


fn item_generator(
    item_name: String,
    category_name: String,
    serial_number: i64, //can be the IMEI of the item so 15 numbers
    item_ID: String,
    price: f32, //
    location: String,
    conditions: String,
    remarks: String,
    item_status: String, //new, used but Ok, broken
    date_added: DateTime<Utc>,
    borrowed_until: DateTime<Utc>,
) -> Item {
    Item {item_name: item_name, category_name: category_name, serial_number: serial_number, item_ID: item_ID, price: price, location: location,conditions: conditions,remarks:remarks, item_status: item_status, date_added: date_added,borrowed_until: borrowed_until}
}


fn main() {
    
    loop {
    println!("Laboratory Inventory Management");
    println!("-----------------------");
    println!("1. Item");
    println!("2. List of Students");
    println!("3. Lend item");
    println!("4. Return item");
    println!("5. Exit Program");
    println!("Choose the corresponding numbers:");
    println!("\n");

    let mut home_input = String::new();
    let stdin = io::stdin();
    let mut counter = 0;
    let now = Utc::now();
    let mut loop_counter = 0;
    let mut item_list: Vec<Item> = Vec::new();
    

         
    while counter < 1 {
        
        stdin.read_line(&mut home_input).unwrap();
        let mut trimmed = home_input.trim();

        if trimmed.parse::<i8>() != Ok(1) && trimmed.parse::<i8>() != Ok(2) && trimmed.parse::<i8>() != Ok(3) && trimmed.parse::<i8>() != Ok(4) && trimmed.parse::<i8>() != Ok(5) {
            println!("wrong input, please re enter the number");
            home_input.clear();
            continue;
        }

        else if trimmed.parse::<i8>() == Ok(1) {
            println!("You choose the {}. Item menu", trimmed);
            println!("\n");
            println!("1. Add New Item");
            println!("2. Add New Category");
            println!("3. Master of Item");
            println!("4. Return to Home Menu"); 
            println!("\n");
            
            let mut item_input = String::new();
            let mut counter_item = 0;

            while counter_item < 1{
                
                stdin.read_line(&mut item_input).unwrap();
                let mut trimmed_item = item_input.trim();
                

                let mut item_1 = Item {
                    item_name: "Monitor".to_string(),
                    category_name: "Hardware".to_string(),
                    serial_number: 123456789101112, //can be the IMEI of the item so 15 numbers
                    item_ID: "blank".to_string(),
                    price: 200.00, //
                    location: "A".to_string(),
                    conditions: "Good".to_string(),
                    remarks: "".to_string(),
                    item_status: "OK".to_string(), //True means available, false means borrowed
                    date_added: now,
                    borrowed_until: now,
                };
                // item_list.push(item_1);
                
                if trimmed_item.parse::<i8>() != Ok(1) && trimmed_item.parse::<i8>() != Ok(2) && trimmed_item.parse::<i8>() != Ok(3) && trimmed_item.parse::<i8>() != Ok(4) {
                    println!("wrong input, please re enter the number");
                    item_input.clear();
                    continue;

                }

                else if trimmed_item.parse::<i8>() == Ok(1){
                    println!("You choose {}. Add New Item.", trimmed_item);
                    println!("\n");
                    let mut new_item = String::new();
                    let mut b = String::new();
                    let mut c_1:i64 = 000000000000;
                    let mut c = String::new();
                    let mut d = String::new();
                    let mut e_1: f32 = 0.00;
                    let mut e = String::new();
                    let mut f = String::new();
                    let mut g = String::new();
                    let mut h = String::new();
                    let mut i = String::new();
                    let mut j = now;
                    let mut k = now;

                    println!("Please enter the name of the item:");
                    let n = stdin.read_line(&mut new_item);
                    println!("\n");

                    println!("Please enter the category name:");
                    let n_1 = stdin.read_line(&mut b);
                    println!("\n");

                    println!("Please enter the serial number:");
                    let n_2 = stdin.read_line(&mut c);
                    println!("\n");

                    println!("Please enter the item ID:");
                    let n_3 = stdin.read_line(&mut d);
                    println!("\n");

                    println!("Please enter the price:");
                    let n_4 = stdin.read_line(&mut e);
                    println!("\n");

                    println!("Please enter the locations:");
                    let n_5 = stdin.read_line(&mut f);
                    println!("\n");

                    println!("Please enter the conditions:");
                    let n_6 = stdin.read_line(&mut g);
                    println!("\n");

                    println!("Please enter the remarks:");
                    let n_7 = stdin.read_line(&mut h);
                    println!("\n");

                    println!("Please enter the item status:");
                    let n_8 = stdin.read_line(&mut i);
                    println!("\n");

                    c_1 = c.trim().parse::<i64>().unwrap();
                    e_1 = e.trim().parse::<f32>().unwrap();
                    let a = item_generator(new_item, b, c_1, d, e_1, f, g, h, i, j, k,);
                    println!("{} {} {} {} {} {} {} {} {} {} {}", a.item_name, a.category_name, a.serial_number, a.item_ID, a.price, a.location, a.conditions, a.remarks, a.item_status, a.date_added, a.borrowed_until);
                    item_list.push(a);
                    // println!("{}", item_list); 
                    //TODO; // still not working well

                    println!("Please press 1 if you want to add another item.");
                    let mut repeat = String::new();
                    stdin.read_line(&mut repeat).unwrap();
                    


                    

                    counter_item +=1;
                
                }

                else if trimmed_item.parse::<i8>() == Ok(2) {
                    println!("You choose {}. Add New Category.", trimmed_item);
                    println!("\n");
                    counter_item += 1;
                }
                else if trimmed_item.parse::<i8>() == Ok(3) {
                    println!("You choose {}. Master of Item.", trimmed_item);
                    println!("Master of item");
                    println!("--------------");
                    println!("1. Search");
                    println!("2. Add item");

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
                        println!("\n");
                        counter_item += 1;
                    }
                    else if trimmed_moi.parse::<i8>() == Ok(2){
                        println!("You choose {}. Add Item", trimmed_moi);
                        println!("\n");
                        // moi_input.clear();
                        item_input = String::from("1");
                        println!("press Enter to continue...");
                        continue;
                    }
                    
                } 
                else if trimmed_item.parse::<i8>() == Ok(4){
                    println!("You choose {}. Return to Home Menu", trimmed_item);
                    counter_item += 1;
                }
            }
            counter += 1;
        }
        
        else if trimmed.parse::<i8>() == Ok(2) {
            println!("You choose {}. List of student menu", trimmed);
            counter += 1;
        } 
    
        else if trimmed.parse::<i8>() == Ok(3) {
            println!("You choose {}. Lend Item menu", trimmed);
            counter += 1;
        }

        else if trimmed.parse::<i8>() == Ok(4) {
            println!("You choose {}. Return Item menu", trimmed);
            counter += 1;
        }
        
        else if trimmed.parse::<i8>() == Ok(5) {
            println!("Exitting program");
            loop_counter += 1;
            break;
        }
        

    
    }
    if loop_counter == 1 {
        break;
    }
    }

}

fn Addinventory() {




}