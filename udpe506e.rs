use std::net::UdpSocket;

fn main() {
	let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind to local address");
	for w in 0..256 {
		println!("{}.x.x.x",w);
		for x in 0..256 {
			println!("{}.{}.x.x",w,x);
			for y in 0..256 {
				for z in 0..256 {
					let _ = socket.send_to(&[b'e';508],&format!("{}.{}.{}.{}:101",w,x,y,z));
				}
			}
		}
	}
}
