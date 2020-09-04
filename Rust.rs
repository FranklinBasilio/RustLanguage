fn main(){
let _cdftv =(235000,10.190,true,'A');
	let _cdftv2:(i32,f64,bool,char) = (235000,10.190,true,'A');
	
	let _array:[i32;5] = [0,1,2,3,4]	;
	let _slice = &_array[0..3];  // elementos 0,1,2 do array
		for x in _slice {
			println!("x is {}",x);
		}


}
