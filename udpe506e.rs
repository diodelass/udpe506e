use std::net::UdpSocket;

fn main() {
	// send to everyone in the ipv4 address range. 
	// takes an absolute minimum of about 6 hours with a gigabit connection and an arbitrarily fast computer.
	let socket = UdpSocket::bind("[::]:0").expect("failed to bind to local address");
	for w in 0..256 {
	for x in 0..256 {
	for y in 0..256 {
	for z in 0..256 {
		let _ = socket.send_to(&[b'e';508],&format!("{}.{}.{}.{}:101",w,x,y,z));
	}
	}
	}
	}
	
	println!("finished ipv4. beginning ipv6...");

	// send to everyone in the ipv6 address range.
	// takes a minimum of about 10 septillion years with a gigabit connection and an arbitrarily fast computer.
	// it is recommended to ensure that your computer's power supply has sufficient endurance for this operation.
	for a in 0..65536 {
	for b in 0..65536 {
	for c in 0..65536 {
	for d in 0..65536 {
	for e in 0..65536 {
	for f in 0..65536 {
	for g in 0..65536 {
	for h in 0..65536 {
		let _ = socket.send_to(&[b'e';508],&format!("[{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}]:101",a,b,c,d,e,f,g,h));
	}
	}
	}
	}
	}
	}
	}
	}

	println!("done.");
}
