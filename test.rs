


fn types () {
	let tuple: (u32, bool) = (3, true);
	let array: [i8; 5] = [1, 2, 3, 4, 5];
	let value = tuple.1; // true
	let value = array[2]; // 3
	
	let array2: [char; 10] = [' '; 10];	
}

fn pattern_match (x: Option<u64>) -> u64 {
	match x {
		Some(y) => y * 2,
		None => 0,
	};
	
	// same as:
	if let Some(y) = x {
		y * 2
	} else {
		0
	}
}


fn loop_break () {
	let value = 456;
	let mut x = 1;
	let y = loop {
		x *= 10;
		if x > value {
			break x / 10;
		}
	};
	println!("largest power of ten that is smaller than or equal to value: {y}");
	
	let mut up = 1;
	'outer: loop {
		let mut down = 120;
		loop {
			if up > 100 {
				break 'outer;
			}
			
			if down < 4 {
				break;
			}
			
			down /= 2;
			up += 1;
			println!("up: {up}, down: {down}");
		}
		up *= 2;
	}
}

fn loop_for () {
	// Using `for` with range syntax for the same functionality as above
	// The syntax 4..=10 means the range from 4 to 10, up to and including 10.
	for value in 4..=10 {
		println!("value = {value}");
	}
}

fn loop_while () {
	// Iterate over all integers from 4 to 10
	let mut value = 4;
	while value <= 10 {
		println!("value = {value}");
		value += 1;
	}
}

fn branching () {
	let x = 10;
	if x > 5 {
		println!("value is greater than five");
	}
	
	if x % 7 == 0 {
		println!("value is divisible by 7");
	} else if x % 5 == 0 {
		println!("value is divisible by 5");
	} else {
		println!("value is not divisible by 7 or 5");
	}
}

fn block_expression () {
	let x = {
		println!("this is inside the block");
		1 + 2
	};
	println!("1 + 2 = {x}");
}

fn variable () {
	let foo: i32 = 10;
	println!("The value of foo is {foo}");
	
	// Mutable variable
	// This code would not compile without adding "mut".
	let mut foo = 10; 
	println!("The value of foo is {foo}");
	foo = 20;
	println!("The value of foo is {foo}");
	
	// Variable shadowing
	let foo = 10;
	// This will output "The value of foo is 10"
	println!("The value of foo is {foo}");
	let foo = foo * 2;
	// This will output "The value of foo is 20"
	println!("The value of foo is {foo}");
}

fn stdout () {
	println!("Hello, World!");
}


fn main () {
	stdout()
}


