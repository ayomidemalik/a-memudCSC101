fn main() {
	let _p:u32 = 520000000;		//the principal or the initial amount of money loaned
	let _n=5;					//the number of years for the compound interest effect
	let _r=10;					//the rate in percentage of the compound interest
	let _compound_interest_formula= 520000000 *1+(10/100)*1+(10/100)*1+(10/100)*1+(10/100)*1+(10/100) - 520000000;		//formula for compound interest
	let compound_interest=317465200;
	println!("The compound interest for 5 years at 10% per annum is = {}",compound_interest);

}