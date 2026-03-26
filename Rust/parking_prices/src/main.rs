/* Step 1: Define the CarType enum - SUV, Sedan, Coupe */
enum CarType {
    SUV,
    Sedan,
    Coupe,
}

/*
    Step 2: Define the Vehicle enum
        Car -> Car types
        Truck -> Cargo Capacity
        Motorcycle
*/
enum Vehicle {
    Car(CarType),
    Truck(i32),
    Motorcycle,
}

/* Step 3: Write a function to calculate parking rates */
impl Vehicle {
    fn get_parking_rate(&self) -> i32 {
        match self {
            Vehicle::Car(CarType::SUV) => return 20,
            Vehicle::Car(CarType::Sedan) => return 15,
            Vehicle::Car(CarType::Coupe) => return 10,
            Vehicle::Motorcycle => return 10,
            Vehicle::Truck(tons) => {
                if *tons <= 10 {
                    return 20;
                } else {
                    return 25;
                }
            }
        }
    }
}

fn main() {
    /* Step 4: Create a vehicle and pass it to the function */
    let vehicles = [
        Vehicle::Car(CarType::SUV),
        Vehicle::Car(CarType::SUV),
        Vehicle::Car(CarType::Sedan),
        Vehicle::Car(CarType::Coupe),
        Vehicle::Car(CarType::Coupe),
        Vehicle::Car(CarType::Coupe),
        Vehicle::Truck(67),
        Vehicle::Truck(20),
        Vehicle::Truck(5),
        Vehicle::Motorcycle,
        Vehicle::Motorcycle,
    ];

    for veh in vehicles {
        let rate = veh.get_parking_rate();
        print!("{}, ", rate);
    }
}
