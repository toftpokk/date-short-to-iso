use std::process::exit;
use std::env;

enum DateError {
    FieldNotExist,
    FieldIsInvalid,
}

fn parse_num(raw_string: Option<&str>) -> Result<i32,DateError>{
    let num_ordinal = match raw_string {
        Some(num_or) => num_or,
        None => {
            return Err(DateError::FieldNotExist);
        }
    };
    let mut num_string = String::from("");
    for c in num_ordinal.chars() {
        if c.is_numeric() {
            let c_str = c.to_string();
            num_string.push_str(&c_str)
        }
        else{
            break;
        }
    }
    if num_string == "" {
        return Err(DateError::FieldIsInvalid);
    }

    let day_number = num_string.parse::<i32>().unwrap();
    if day_number < 1 {
        return Err(DateError::FieldIsInvalid);;
    }
    Ok(day_number)
}

fn main() {
    let args : Vec<String> = env::args().collect();
    // if date_string != "" {
    //     println!("{}",date_string)
    // }
    if args.len() < 1 {
        eprintln!("Error: Expected date argument");
        exit(1)
    }

    // Define months
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    // Split MMM dd yyyy
    let mut splits = args[1].split(" ");

    // Match month
    let short_month = match splits.next() {
        Some(sm) => sm,
        None => {
            eprintln!("Error: month does not exist");
            exit(1);
        }
    };
    if !months.contains(&short_month) {
        eprintln!("Error: month does not exist");
        exit(1)
    }
    let month_number = months.iter().position(|&s| s == short_month).unwrap();
    let month_number = month_number + 1;

    // Match Day
    let day_number = match parse_num(splits.next()) {
        Ok(n)=>n,
        Err(e)=>{
            match e {
                DateError::FieldIsInvalid => eprintln!("Error: day is invalid"),
                DateError::FieldNotExist => eprintln!("Error: day does not exist")
            }
            exit(1)
        }
    };

    // Year
    let year_number = match parse_num(splits.next()) {
        Ok(n)=>n,
        Err(e)=>{
            match e {
                DateError::FieldIsInvalid => eprintln!("Error: year is invalid"),
                DateError::FieldNotExist => eprintln!("Error: year does not exist")
            }
            exit(1)
        }
    };
    println!("{}-{:02}-{:02}",year_number,day_number,month_number)
}
