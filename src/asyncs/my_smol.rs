pub fn start() {
    // Call the async function
    let result = get_number();
    let number = smol::block_on(result);
    println!("The number is: {}", number);
}
async fn get_number() -> i32 {
    println!("running get_number async function");
    42
}
