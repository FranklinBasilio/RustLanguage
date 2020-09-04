fn main(){
	//bool
	let _van:bool = true;
	let _dormir = false;
	
	//char
	let _amor = '♥';
	let _arroba:char = '@';
	
	//float
	let _saldo_conta_van = 91599.99;  //f64 padrao
	let _saldo_conta_gab:f32=1.99; //evite usar o f32
	
	//array
	let _pgtos_dias = [5,15,20];


  let prices:[f64; 4] = [5.66, 15.00, 2.22, 1.11];
    let mut total:f64 = 0.0;
    for x in prices.iter() {
        total += x;
    }
    println!("Total a pagar: {}", total);

}
