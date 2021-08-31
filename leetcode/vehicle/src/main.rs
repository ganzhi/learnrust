trait Drive {
    fn drive(&self);
}

struct Car {
    weight:i32
}

struct Truck {

}

impl Drive for Car {
    fn drive(&self) {
        println!("You are driving a car!");
    }
}

impl Drive for Truck {
    fn drive(&self) {
        println!("you are driving a truck!")
    }
}

fn main() {
    let mut vehicles = Vec::<Box<dyn Drive>>::new();
    let car = Car{weight:1};
    vehicles.push(Box::new(car));
    let truck = Truck{};
    vehicles.push(Box::new(truck));
    for item in vehicles.iter() {
        item.drive();
    }

    let b = Box::new(5);
    println!("b = {}", b);
}
