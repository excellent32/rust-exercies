 mod gas_price;
 use gas_price::get_gas_fee;
 
 mod gas_suggestion;
 use gas_suggestion::get_gas_suggestion;
 

#[tokio::main]
async fn main(){
    // match get_gas_fee().await {
    //     Ok(data) => {
    //         for (key, value) in &data{
    //             println!("key: {}, value: {}", key, value)
    //         }
    //     },
    //     Err(e) => println!("Error: {}", e),
    // }

    match get_gas_suggestion().await {
        Ok(data) => {
            println!("suggest: {}", data)
        },
        Err(e) => println!("Error: {}", e),
    }
}