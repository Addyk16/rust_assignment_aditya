
// Fix errors and panics to make it work
fn main() {
  let v1 = 247_u8 + 8;
  let v2 = u8::checked_add(247, 8).unwrap();
  println!("{},{}",v1,v2);
}
