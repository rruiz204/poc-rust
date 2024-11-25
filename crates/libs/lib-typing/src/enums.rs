enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn action(&self) -> &str {
        match self {
            TrafficLight::Red => "Stop",
            TrafficLight::Yellow => "Caution",
            TrafficLight::Green => "Go",
        }
    }
}

enum IpAddress {
    V4(i32, i32, i32, i32),
    V6(String),
}

impl IpAddress {
    fn showcase(&self) {
        match self {
            IpAddress::V4(a, b, c, d) => println!("{}.{}.{}.{}", a ,b , c, d),
            IpAddress::V6(address) => println!("{}", address)
        }
    }
}

pub fn enums_showcase() {
    println!("=== Enums and Matching ===");
    let light: TrafficLight = TrafficLight::Green;
    println!("The light says: {}", light.action());

    let light: TrafficLight = TrafficLight::Red;
    println!("The light says: {}", light.action());

    let light: TrafficLight = TrafficLight::Yellow;
    println!("The light says: {}", light.action());

    println!("=== Enums with Values ===");
    let ipv4: IpAddress = IpAddress::V4(127, 0, 0, 1);
    ipv4.showcase();

    let ipv6: IpAddress = IpAddress::V6(String::from("::1"));
    ipv6.showcase();
}
