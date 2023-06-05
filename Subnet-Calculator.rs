use std::net::Ipv4Addr;
use std::io;

fn main() {
    println!("Enter the IP address (e.g., 192.168.1.0):");
    let mut ip_address = String::new();
    io::stdin().read_line(&mut ip_address).expect("Failed to read input.");
    let ip_address: Ipv4Addr = ip_address.trim().parse().expect("Invalid IP address.");

    println!("Enter the subnet mask (e.g., 24):");
    let mut subnet_mask = String::new();
    io::stdin().read_line(&mut subnet_mask).expect("Failed to read input.");
    let subnet_mask: u8 = subnet_mask.trim().parse().expect("Invalid subnet mask.");

    let subnet_size = 2u32.pow(u32::from(32 - subnet_mask));

    println!("IP Address: {}", ip_address);
    println!("Subnet Mask: /{}", subnet_mask);
    println!("Subnet Size: {}", subnet_size);

    // Calculate and display different subnet divisions
    let mut current_subnet_mask = subnet_mask;
    while current_subnet_mask < 32 {
        let subnet_count = 2u32.pow(u32::from(32 - current_subnet_mask));
        println!("Number of /{} Subnets: {}", current_subnet_mask, subnet_count);
        current_subnet_mask += 1;
    }
}
