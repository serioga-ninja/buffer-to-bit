
fn concat_vecs(vecs: Vec<Vec<i32>>) -> Vec<i32> {
    let size = vecs.iter().fold(0, |a, b| a + b.len());
    vecs.into_iter().fold(Vec::with_capacity(size), |mut acc, v| {
        acc.extend(v); acc
    })
}


fn octet(arr: Vec<i32>) -> Vec<i32> {
	let len = arr.len();
	let fill = len + (8 - len % 8);

	if len != 0 && len % 8 == 0 {
	    return arr;
	}
	for i in len..fill {
	    arr[i] = 0;
	}
	return arr;
}

fn parse(x: &u8) -> Vec<i32> {
    let mut bits: Vec<i32> = Vec::new();
    let mut tmp: f64 = x.clone() as f64;

 //    if (x === undefined || x === null) {
	//     return bits;
	// }

	while (tmp > 0_f64) {
		let v = tmp % 2_f64;
        bits.push(v as i32);
        tmp = tmp / 2_f64;
        tmp = tmp.floor();
    }
    bits = octet(bits);
    bits.reverse();
    return bits;
}

#[no_mangle]
pub extern fn do_this(buffer_string: [u8;12]) -> Vec<i32> {
	let mut vec: Vec<Vec<i32>> = Vec::new();
	for row in &buffer_string {
		vec.append(parse(row));
	}
	return vec;
}