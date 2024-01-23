use tokio::sync::Mutex;
use std::sync::{Arc};
use std::sync::mpsc::Sender;
use itertools::Itertools;
use mysql::{params, Pool, PooledConn};
use mysql::prelude::Queryable;
use crate::Model::{MainBody, OfficeEvent};

pub fn establish_connection() -> PooledConn {
    let url = r#"mysql://gen_user:U\3+)5,,bGwcsM@94.241.169.12/default_db"#;
    let pool = Pool::new(url).expect("Couldn't connect to a base");
    println!("Connection with MySQL pool is established!");
    return pool.get_conn().unwrap();
}

pub fn fill_with_data(data_sender : Sender<Vec<OfficeEvent>>, connection : Arc<Mutex<PooledConn>>, month_sender : Sender<Vec<String>>) -> () {
    tokio::spawn(async move {
        let mut locked_connection = connection.lock().await;
        let arrived_data : Vec<OfficeEvent> = locked_connection.query_map("SELECT amount, month, my_comment, is_possitive, year FROM office", |(amount, month, my_comment, is_possitive, year)| {
            OfficeEvent {
                amount: amount,
                month: month,
                my_comment: my_comment,
                is_possitive: is_possitive,
                year: year,
            }
        }).unwrap();
        println!("{:#?}", arrived_data);
        drop(locked_connection);
        _ = data_sender.send(arrived_data.clone());
        _ = month_sender.send(arrived_data.clone().iter().map(|value| value.month.to_string()).collect::<Vec<String>>().into_iter().unique().collect::<Vec<String>>())
    });
}

pub fn filtered_request(typo : String, connection : Arc<Mutex<PooledConn>>, data_sender : Sender<Vec<OfficeEvent>>) -> () {
    tokio::spawn(async move {
        let formated = format!("SELECT * from office where month = '{}'", typo);
        let mut locked_connection = connection.lock().await;
        let returnable : Vec<OfficeEvent> = locked_connection.query_map(formated, |(amount, month, my_comment, is_possitive, year)| {
            OfficeEvent {
                amount: amount,
                month: month,
                my_comment: my_comment,
                is_possitive: is_possitive,
                year: year,
            }
        }).unwrap();
        drop(locked_connection);
        let _ = data_sender.send(returnable);
    });
}

pub fn add_data_sql(connection : Arc<Mutex<PooledConn>>, amount : u64, month : String, comment : String, bool : u8, year : u16) -> () {
    tokio::spawn(async move {
        let mut sended_vec : Vec<OfficeEvent> = Vec::new();
        sended_vec.push(OfficeEvent {
            amount: amount,
            month: month,
            my_comment: comment,
            is_possitive: bool,
            year: year,
        });
        let mut connection_unlocked = connection.lock().await;
        connection_unlocked.exec_batch(r"INSERT INTO office (amount, month, my_comment, is_possitive, year) VALUES (:amount, :month, :my_comment, :is_possitive, :year)", sended_vec.iter().map(|c| params! {
                "amount" => c.amount,
                "month" => &c.month,
                "my_comment" => &c.my_comment,
                "is_possitive" => c.is_possitive,
                "year" => c.year
            })).unwrap();
        drop(connection_unlocked);
    });
}


pub fn string_to_int(text : String) -> u64 {
    return text.trim().parse::<f32>().unwrap().ceil() as u64
}

pub fn bool_to_u8(sample : bool) -> u8 {
    match sample {
        true => {return 1}
        false => {return 0}
    }
}
