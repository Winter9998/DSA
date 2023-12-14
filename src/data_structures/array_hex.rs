use rand::Rng;

pub fn generate_number() -> u64 {
    let max = u64::MAX;
    let r#mod = u64::MAX % 2_u64.pow(20);
    let number = rand::thread_rng().gen_range(r#mod..=max);
    number
  
}

pub fn make_array_hex() {

    let mut vec: Vec<*const i32> = Vec::new();
    let mut avec: Vec<u64> = Vec::new();
    let mut i: u64 = 0;
    loop {
        avec.push(generate_number());
        let hex = generate_number() as *const i32;
        vec.push(hex);
        i += 1;
        if i == 8 {
            break
        }

    }  

    for ptr in &vec {
        println!("{:p}", ptr);
    }
    println!("{:?}", avec);

}