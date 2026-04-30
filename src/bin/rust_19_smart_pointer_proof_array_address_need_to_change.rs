pub fn main() {
    let mut vec = vec![10, 20, 30];

    println!("--- Array (Vec) Address Proof ---");

    let addr_10 = &vec[0] as *const i32;
    let addr_20 = &vec[1] as *const i32;
    let addr_30 = &vec[2] as *const i32;

    println!("Before Delete - '10' Address: {:?}", addr_10);
    println!("Before Delete - '20' Address: {:?}", addr_20);
    println!("Before Delete - '30' Address: {:?}", addr_30);

    vec.remove(1); 

    println!("\n--- After Deleting Middle (20) ---");

    let new_addr_10 = &vec[0] as *const i32;
    let new_addr_30 = &vec[1] as *const i32;

    println!("After Delete - '10' Address: {:?}", new_addr_10);
    println!("After Delete - '30' Address: {:?}", new_addr_30);

    if addr_30 != new_addr_30 {
        println!("\nRESULT: The address of '30' has changed. The data must have been moved to a different location in memory.");
    }
}