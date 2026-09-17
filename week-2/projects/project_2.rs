fn main() {
	let q1:f64 = 2.0;
	let q2:f64 = 1.0;
	let q3:f64 = 3.0;
	let q4:f64 = 3.0;
	let q5:f64 = 1.0;
	let a1:f64 = 450_000.00;
	let a2:f64 = 1_500_00.00;
	let a3:f64 = 750_000.00;
	let a4:f64 = 2_850_000.00;
	let a5:f64 = 250_000.00;

	//Total sum spent 
	let sum:f64 = (q1 * a1) + (q2 * a2) + (q3 * a3) + (q4 * a4) + (q5 * a5);
	println!("The total sum spent is {}",sum);
	//total quantity
	let qty:f64 = q1 + q2 +q3 + q4 + q5;
	//The average dpent per item
	let average:f64 =sum/qty;
	println!("The average spent per item is {}",average);
	
}