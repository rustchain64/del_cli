use structopt::StructOpt; // parses the commandline
use exitfailure:: { ExitFailure };
use serde_derive:: { Serialize, Deserialize };
use reqwest::Url;

use std::time::{Duration, Instant};

mod distance;

/*
Use the twilio api to send an SMS when an order is expected to be delivered late. 

The system should 

STRUCTURE :: 
receive an order id, 
when it’s supposed to be delivered,
 coordinates for pickup 
 
 and delivery and 
 
 the number to send an sms to. 
 
 You can assume a fixed rate of travel over straight line distance.

It can be a command line tool or a web system, 
    just needs to have a simple method for us to pass it the info.

        cli example `mytool -o 123 -p 27,- 80 -d 28,-79  -t "2020-01-01T15:45" -c 5551231234`

    if it is going to be late 
        the customer should receive an SMS containing the order number
    
    if it is on time, 
        don't need to send an sms

*/

#[derive(StructOpt)]
struct Cli {


     // input
     -o 123 
     -p 27,- 80 
     -d 28,-79  
     -t "2020-01-01T15:45" 
     -c 555-123-1234`

    // inputs
    order_id: i32,
    pickup_coord: Pickup_Coord,
    delivery_coord: Delivery_Coord,
    delivery_time: DateTime,
    sms_number: i32,
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    order_id: i32,
    order_time: DateTime, 
    pickup_coord: Pickup_Coord,
    delivery_coord: Delivery_Coord,
    delivery_time: DateTime,
    sms_number: i32,
    
    // dr=t
    distance: f32,
    rate: f32,
    calc_time: f32,
    on_time: bool, // call if false
}

#[derive(Serialize, Deserialize, Debug)]
struct Pickup_Coord {
    long: f64,
    lat: f64,
}

struct Delivery_Coord {
    long: f64,
    lat: f64,
}

fn on_time( delivery_time: DateTime, order_time: DateTime, calc_time: DateTime ) -> bool {
    delivery_time - order_time > calc_time
}


impl Order {
    async fn get( 
        order_id: &i32,
        order_time: &DateTime, 
        pickup_coord: &Pickup_Coord,
        delivery_coord: &Delivery_Coord,
        delivery_time: &DateTime,
        sms_number: &i32,
        distance: &f32,
        rate: &f32,
        calc_time: &f32,
        on_time: &bool,
        
    ) -> Result<Self, ExitFailure> {

        // get a distance from an open source mapping api
        let url = format!( "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid=3b91ff30f4a9f97d3af6637eb51b675e", pickup_coord, delivery_coord );
        let url = Url::parse(&*url)?;

        let resp = reqwest::get( url )
        .await?
        .json::<Order>()
        .await?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response:Order = Order::get( 
        //&args.order_id, 
       // &args.order_time
        &args.pickup_coord,
        &args.delivery_coord,
        //&args.delivery_time,
        //&args.sms_number,
    ).await?;

    println!("Pickup Coord:  {} Delivery Coord: {}, Distance: {}% ", args.pickup_coord, args.delivery_coord, response.main.distance );
    Ok(())

    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
