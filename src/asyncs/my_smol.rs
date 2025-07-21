pub fn start() {
    // Call the async function
    let result = get_number();
    let number = smol::block_on(result);
    println!("The number is: {}", number);

    println!(" ");

    // Make a coffee
    smol::block_on(async {
        boil_water().await;
        add_sugar().await;
        make_coffee().await;
    });
}
async fn get_number() -> i32 {
    println!("running get_number async function");
    42
}

async fn boil_water() {
    println!("Boiling water...");
    // Simulate a delay
    smol::Timer::after(std::time::Duration::from_secs(1)).await;
    println!("Water boiled!");
}

async fn add_sugar() {
    println!("Adding sugar to coffee...");
    // Simulate a delay
    smol::Timer::after(std::time::Duration::from_secs(1)).await;
    println!("Sugar added!");
}

async fn make_coffee() {
    println!("Making coffee...");
    // Simulate a delay
    smol::Timer::after(std::time::Duration::from_secs(2)).await;
    println!("Coffee is ready!");
}
