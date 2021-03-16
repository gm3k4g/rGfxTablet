use std::error::Error;
use std::io;
use tokio::net::UdpSocket;
use enigo::MouseControllable;
use enigo::*;

use winit::event_loop::EventLoop;
use std::convert::TryInto;

struct Server {
	socket: UdpSocket,
	buf: Vec<u8>,
	mouse: enigo::Enigo,
}

impl Server {
	async fn run(mut self) -> Result<(), io::Error> {

		let event_loop = EventLoop::new();

		// TODO: now just create each relevant struct and assign the values to the struct/update mouse accordingly.
		loop {
			let (_size, _addr) = self.socket.recv_from(&mut self.buf).await?;
			self.process(&event_loop);
		}
	}

	fn process(&mut self, event_loop: &EventLoop<()>) {
		let xpos: u16 = u16::from_be_bytes([self.buf[12], self.buf[13]]).try_into().unwrap(); // << 8) | &self.buf[13] & 0xFF;
		let ypos: u16 = u16::from_be_bytes([self.buf[14], self.buf[15]]).try_into().unwrap(); // << 8) | &self.buf[15] & 0xFF;
		let pressure: u16 = u16::from_be_bytes([self.buf[16], self.buf[17]]).try_into().unwrap(); // << 8) | buf[17] & 0xFF;
		let press_down: bool = match &self.buf[18] & 0xFF {
				0 => true,
				_ => false,
			};

		let pm = event_loop.primary_monitor().unwrap().size();

		// TODO: is this correct?
		let mulx: i32 = ((xpos as f32/65535f32) * pm.width as f32) as i32;
		let muly: i32 = ((ypos as f32/65535f32) * pm.height as f32) as i32;

		self.mouse.mouse_move_to(mulx, muly);

		// TODO: implement a 'mode change' thing?
		match press_down {
			true => {
				self.mouse.mouse_down(MouseButton::Left);
			},
			false => {
				self.mouse.mouse_up(MouseButton::Left);
			}
		}
		
		print!(" x : {} ", mulx);
		print!("| y: {} ", muly);
		print!("| pressure : {}", pressure);
		print!("| left_click : {}", press_down);
		println!();
	}
}

/*
fn get_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect("8.8.8.8:80").await?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}
*/

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
	//println!("{}",get_ip().await.unwrap());

	let addr = String::from("0.0.0.0:40118");
	let socket = UdpSocket::bind(&addr).await?;

	println!("GfxTablet Input Driver -- Rust edition");
	println!("Make sure you have GfxTablet up and running on your android device!");
	println!("Copy the IP after \"addr:\" and use it as the recipient's IP address in your settings:");

	let myaddress = UdpSocket::bind("0.0.0.0:0").await?;
	myaddress.connect("8.8.8.8:80").await?;
    myaddress.local_addr().ok().map(|addr| addr.ip().to_string());
	println!("{:?}", myaddress);

	println!("Listening on: {}", socket.local_addr()?);

	// Init mouse
	// todo

	// Init server
	let server = Server {
		socket,
		buf: vec![0; 20],
		mouse: enigo::Enigo::new(),
	};

	server.run().await?;

	Ok(())
}