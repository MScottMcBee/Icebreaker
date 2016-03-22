extern crate rand;

use std::net::*;
use std::io::{Error, ErrorKind, Result};
use std::fmt;
#[derive(Debug)]
struct ServerInfo {
    id: String,
    addrs: SocketAddr
}

#[derive(Debug)]
struct State{
    servers: Vec<ServerInfo>,
    socket: UdpSocket
}


impl State{
    fn new()-> Result<State>{ 
        let socket = try!(UdpSocket::bind("0.0.0.0:34254"));

        let state:State = State{
            servers: Vec::new(),
            socket: socket,
        };

        Ok(state)
    }
    fn add_server(&mut self, info:ServerInfo){
        self.servers.push(info);
    }
}


fn main() {
    let mut state:State = match State::new(){
        Ok(string) => string,
        Err(err) => {
            println!("Oh SHIT {}",err);
            return ();
        },
    };

    for _ in 0..13{
    	let _ = responder(&mut state);
    }

    drop(state.socket);
}


fn random_id() -> String{
    let mut s:String = "".to_string();

    s = s + random_letter() + random_letter() + random_letter() + random_letter();

    s
}

fn random_letter<'a>() -> &'a str{
    let n = rand::random::<u32>();
    match n%25 {
        0  => "A",
        1  => "B",
        2  => "C",
        3  => "D",
        4  => "E",
        5  => "F",
        6  => "G",
        7  => "H",
        8  => "I",
        9  => "J",
        10 => "K",
        11 => "L",
        12 => "M",
        13 => "N",
        14 => "O",
        15 => "P",
        16 => "Q",
        17 => "R",
        18 => "S",
        19 => "T",
        20 => "U",
        21 => "V",
        22 => "W",
        23 => "X",
        24 => "Y",
        25 => "Z",
        _  => "?"
    }
}

fn responder(state: &mut State) -> std::io::Result<()>{
	println!("setting up socket for responder");
	//let socket = try!(UdpSocket::bind("0.0.0.0:34254"));

	let mut buf = [0; 256];
	let (amt, src) = try!(state.socket.recv_from(&mut buf));
    println!("{} bytes from {:?}",amt, src);

	//println!("We got a message! It's {}!",c);
	let buf = &mut buf[..amt]; //trim the buffer


    let return_string = match std::str::from_utf8(&buf){
        Ok(string) => string,
        Err(err) => {
            println!("ERROR 3 {}",err);
            return Err(Error::new(ErrorKind::Other, "oh no!"));
        },
    };



    println!("{}!",return_string);

    let split_message: Vec<&str> = return_string.split("|").collect();
    //let everything_else: [&str] = split_message[1..split_message.len()];

    for i in 1..split_message.len(){
        println!("^{}", split_message[i]);
    }

    match split_message[0]{
        "AA" => try!(aa(&split_message,&src, state)),
        "AB" => try!(ab(&split_message,&src, &state)),
        "AC" => try!(ac(&split_message,&src, &state)),
        _ => println!("Oh..."),
    };


    //AA register server
    //AB YYYY, X wants to connect to you
    //AC YYYY sent a packet. ok X, try connectin to YYYY



    //BA you're now server YYYY
    //BB send a packet to X
    //BC connect to server YYYY
    //BD server don't exist

	Ok(())
}

fn aa(v: &Vec<&str>, src: &SocketAddr, state: &mut State) -> std::io::Result<()>{
    let mut result = "BA|".to_string();

    let id = random_id();

    result = result + &id;

    let info:ServerInfo = ServerInfo {
        id: id, 
        addrs: *src,
    };
    state.add_server(info);
    println!("{:?}", result);

    try!(state.socket.send_to(result.as_bytes(), src));

    Ok(())
}

fn ab(v: &Vec<&str>, src: &SocketAddr, state: &State) -> std::io::Result<()>{
    let mut i = 0;
    for x in v {
        println!("{} {}",i, x);
        i += 1;
    }

  //  let mut gameServer = src;

    println!(" looking for server {:?}", v[1]);

    let mut result = "BB|".to_string();

    match *src {
        SocketAddr::V4(v4) => {
            let octets = v4.ip().octets();
            for i in 0..4{
                result = result + &octets[i].to_string();
                if i < 3{
                    result = result + ".";
                }
            }
            result = result + ":" + &v4.port().to_string();
            println!("{}.{}.{}.{}", octets[0],octets[1],octets[2],octets[3]);
        },
        SocketAddr::V6(_) => {
            println!("Ok, I honestly have no idea how IPv6 works...");
        }
    }

    for server in &state.servers{
        println!("We found server {:?}",server);
        println!("is this it? {:?}", server.id == v[1] );
        if (server.id == v[1]){
            try!(state.socket.send_to(result.as_bytes(), &server.addrs));
        }
    }
  //  try!(gameServer.send_to(result.as_bytes(), src));

    Ok(())
}

fn ac(v: &Vec<&str>, src: &SocketAddr, state: &State) -> std::io::Result<()>{
    let mut i = 0;
    for x in v {
        println!("{} {}",i, x);
        i += 1;
    }


    println!("Tell client at {:?} to connect to SRC ", v[1]);


    let splitargs:Vec<&str> = v[1].split(":").collect();
    let ip:&str = splitargs[0];
    let bbb:Vec<&str> = ip.split(".").collect();


    let mut result = "BC|".to_string();

    let newClientIP = Ipv4Addr::new(
            bbb[0].parse::<u8>().unwrap(),
            bbb[1].parse::<u8>().unwrap(),
            bbb[2].parse::<u8>().unwrap(),
            bbb[3].parse::<u8>().unwrap(),
        );

    let newClientSocketAddr = SocketAddrV4::new(newClientIP, splitargs[1].parse::<u16>().unwrap());

    match *src {
        SocketAddr::V4(v4) => {
            let octets = v4.ip().octets();
            for i in 0..4{
                result = result + &octets[i].to_string();
                if i < 3{
                    result = result + ".";
                }
            }
            result = result + ":" + &v4.port().to_string();
           // println!("{}.{}.{}.{}", octets[0],octets[1],octets[2],octets[3]);
        },
        SocketAddr::V6(_) => {
            println!("Ok, I honestly have no idea how IPv6 works...");
        }
    }
    println!("{:?}", result);
    try!(state.socket.send_to(result.as_bytes(), newClientSocketAddr));

    Ok(())
}