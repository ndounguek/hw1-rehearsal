fn main() {
    println!("Average: {}",average(4,6));
    println!("double: {}" ,double(5)); 	
}
fn average(a: u32, b: u32) -> u32 { 
	(a+b) / 2
}
fn double(x: u32) -> u32 {
    x * 2
}
