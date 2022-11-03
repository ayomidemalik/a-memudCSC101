fn main() {
	let toshiba_amount:f32 = 450000.00;		//the amount of toshiba
	let mac_amount:f32 = 1500000.00;		//the amounf of MAC
	let hp_amount:f32 = 750000.00;			//the amount of HP
	let dell_amount:f32 = 2850000.00;		//the amount of Dell
	let acer_amount:f32 = 250000.00;		//the amount of Acer
	let n_s:f32 = 2.0;						//the quantity of Toshiba
	let n_p:f32 = 1.0;						//the quantity of MAC
	let n_q:f32 = 3.0;						//the quantity of HP
	let n_f:f32 = 3.0;						//the quantity of Dell
	let n_a:f32 = 1.0;						//the quantity of Acer
	let sum = (toshiba_amount * n_s) + (mac_amount * n_p) + (hp_amount * n_q) + (dell_amount * n_f) + (acer_amount * n_a);		//formula for sum
	let average = sum/(n_s + n_p + n_q + n_f + n_a);		//formula for average

	println!("The sum is {}",sum);
	println!("The average is {}",average);
}