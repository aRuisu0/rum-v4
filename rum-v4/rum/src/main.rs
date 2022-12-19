use rum::rum::Vm;


fn main() {

	let mut rum = Vm::new();
	rum.boot();
	rum.run();


	// // Take argument from cmd
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let mut groups: Vec<[u8; 4]> = vec![];

	// // Unwrap file (in this case will be a .um)
    // let file = File::open(filename).unwrap();
	// // Map out our files bytes 
	// 	let bytes = file.bytes().map(|wrapped_byte| wrapped_byte.unwrap());

	// // An iterator over a slice in (non-overlapping) chunks (chunk_size elements at a time), starting at the beginning of the slice.
	// //When the slice len is not evenly divided by the chunk size, the last slice of the iteration will be the remainder.
	// 	for chunk in bytes.collect::<Vec<u8>>().chunks(4) {
	// 		groups.push([
	// 			chunk[0], chunk[1], chunk[2], chunk[3],
	// 		]);
	// 	}

	// 	// By implementing IntoIterator for a type, you define how it will be converted to an iterator. 
	// 	// This is common for types which describe a collection of some kind.In our case, bytes collected from word.
	// 	let words: Vec<u32> = groups.into_iter().map(|byte_group| u32::from_be_bytes(byte_group)).collect();

	// 	// Call our Virtual Machine and our "registers" vector to only allow Vecs to be defined with the same syntax as array expressions
	// let mut rum = Vm {
	// 		registers: vec![0_u32; 8],
	// 		memory: vec![words],
	// 		unmapped_segs: vec![],
	// 		max_mapped_seg: 0,
	// 		prog_count: 0,
	// 		running: true
	// };
    
	// // Excecute the instructions while our virtual machine is running for the world input
    // let now = Instant::now();
	// // Use a while loop to run the instructions in the rum.
	// let mut instruction: u32;
	// while rum.running {
    // 	// Get the next instruction from the rum.
    // 	instruction = rum.boot();
    // 	// Execute the instruction.
    // 	rum.execute(instruction);
    // 	// Calculate the elapsed time using the elapsed() method on the Instant object.
    // 	// let elapsed = now.elapsed();
    // 	// Print the elapsed time.
    // 	// println!("{:.2?}", elapsed);
	// }

}
