use serde::Deserialize;

pub fn start() {
    println!("================== CSV Example =================");
    csv_example();

    println!("\n");
    println!("================== CSV with Serde Example =================");
    csv_with_serde();
}

fn csv_with_serde() {
    #[derive(Deserialize, Debug)]
    struct Vehicle {
        #[serde(rename = "Manufacture")]
        manufacture: String,
        #[serde(rename = "Model")]
        model: String,
        #[serde(rename = "Year")]
        year: u16,
        #[serde(rename = "Color")]
        color: String,
        #[serde(rename = "Price")]
        price: f64,
    }

    let file_path = "src/csv/vehicle.csv";

    let mut builder = csv::ReaderBuilder::new();
    builder
        .has_headers(false) // Set to false if the CSV has headers
        .double_quote(false); // Set to false if you want to disable double quotes handling
    let res = builder.from_path(file_path); // Customizable reader with options

    match res {
        Ok(mut reader) => {
            for result in reader.deserialize::<Vehicle>() {
                match result {
                    Ok(record) => {
                        // println!("{:?}", record); // Printing the record directly
                        // println!("{:?}", record.get(0)); // Printing the record item at index 0
                        println!("{:?}", record); // Collecting the record into a vector of strings for better readability
                    }
                    Err(e) => {
                        eprintln!("Error reading record: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
}

fn csv_example() {
    let file_path = "src/csv/vehicle.csv";
    let res = csv::Reader::from_path(file_path); // not customizable, but can be used to read CSV files

    let mut builder = csv::ReaderBuilder::new();
    builder
        .has_headers(false) // Set to false if the CSV has headers
        .double_quote(false); // Set to false if you want to disable double quotes handling
    let res = builder.from_path(file_path); // Customizable reader with options

    match res {
        Ok(mut reader) => {
            for result in reader.records() {
                match result {
                    Ok(record) => {
                        // println!("{:?}", record); // Printing the record directly
                        // println!("{:?}", record.get(0)); // Printing the record item at index 0
                        println!("{:?}", record.iter().collect::<Vec<&str>>()); // Collecting the record into a vector of strings for better readability
                    }
                    Err(e) => {
                        eprintln!("Error reading record: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
}
