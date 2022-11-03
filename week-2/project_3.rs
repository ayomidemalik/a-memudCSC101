fn main() {
	let _p:u32 = 210000; 	//the amount the tv was bought for
	let _n=3;				//the amount of years for the depreciation
	let _r=5;				//the rate in percentage of the depreciation
	let _depreciation_formula = _p * ( (1 - (_r / 100)) ^ _n );		//the formula for depreciation
	let depreciation:f32=180048.75;
	println!("The value of the TV after 3 years at 5% per annum depreciation is = {}",depreciation);

}