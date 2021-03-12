// i8, i16, i32, i64, isize
// u8 u16 u32 u64 usize
// bool
// str

// Macro Magic
// Macro's in Rust
#[derive(Debug, PartialEq)]

// organizing data in to groups
pub struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Ship {
  location: Point,
  status: ShipStatus,
}

#[derive(Debug, PartialEq)]
pub enum ShipStatus { // choice of statuses
  Engaged,
  Waiting,
  Firing(Point),
  Heading(Point),
}

fn main() {
  // println!("Hello World!");
  // println!("hello, to all of you!!!");
  // for x in 0..5 {
  
  // i8 i16 132 i64 isize
  // u8 u16 u32 u54 usize

  // mut means we are going to change it in code
  let mut ship = Ship {
    location:Point{x:10,y:10}, 
    status: ShipStatus::Waiting,
  };

  ship.location.x += 10;

    println!("ship location = ({}, {})", ship.location.x, ship.location.y);
    println!("ship = {:?}", ship); // debug form
}