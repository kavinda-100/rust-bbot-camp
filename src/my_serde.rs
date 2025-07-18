use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};

// #[serde(rename_all = "camelCase")] = this causes the errors when Deserialized back to structs
#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")] // This attribute renames the fields in the JSON output to camelCase
struct VehicleOwner {
    owner_name: String,
    owner_age: u8,
}
#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")] // This attribute renames the fields in the JSON output to camelCase
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    price: f64,
    owner: VehicleOwner, // Optional field for owner
}

pub fn start() {
    // ============================== Serialization Example ==============================
    // Create an instance of VehicleOwner
    let owner_01 = VehicleOwner {
        owner_name: String::from("Alice"),
        owner_age: 30,
    };
    // Create an instance of Vehicle
    let vehicle_01 = Vehicle {
        manufacturer: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2020,
        price: 20000.00,
        owner: owner_01, // Assign the owner to the vehicle
    };

    // Serialize the vehicle instance to a JSON string
    let serialized_vehicle = to_string(&vehicle_01); // This will serialize the Vehicle instance to a JSON string
    let serialized_vehicle = to_string_pretty(&vehicle_01); // This will serialize the Vehicle instance to a pretty-printed JSON string
    match serialized_vehicle {
        Ok(json_string) => println!("Serialized Vehicle: {}", json_string),
        Err(e) => println!("Serialization failed: {}", e),
    }

    // ============================== Deserialization Example ==============================

    let json_data = r#"
    {
        "manufacturer": "Honda",
        "model": "Civic",
        "year": 2021,
        "price": 22000.00,
        "owner": {
            "owner_name": "Bob",
            "owner_age": 28
        }
    }"#;

    // Deserialize the JSON string back into a Vehicle instance
    let deserialized_vehicle: Result<Vehicle, _> = from_str::<Vehicle>(json_data); // This will deserialize the JSON string into a Vehicle instance
    match deserialized_vehicle {
        Ok(vehicle) => println!("Deserialized Vehicle: {:#?}", vehicle),
        Err(e) => println!("Deserialization failed: {}", e),
    }
}
