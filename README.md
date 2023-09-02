<h1 align="center">
  <br>
  <a><img src="assets/ravelogo.png" alt="Rave" width="300"></a>
  <br>
  RAVE
  <br>
</h1>

<h4 align="center">A Showcase of Rust Memory Analysis Evasion</h4>
<h5 align="center">Using Unsafe Memory Allocation to Prevent Value Leaks during Value Initialization</h5>

<p align="center">

![showcase](assets/showcase.gif)
<br align="center">RAVE deallocates the value and replaces the variable in the heap with a NULL opcode (`0x40`).</br>
<br><br>
<br align="center">How Rust Drops Variables:</br>
![How rust drop variables](assets/vdrop.png)

</p>

## Features
- Allocation
- Deallocation
- Reading
- Variable Spoofing [ `Dealloc::delete()` ]

You can still overwrite values with `*adr = 5`.

## Usage
### 1. Import as src/main.rs module
```rs
mod rave

fn main() {
}
```
### 2. Allocates a &str
```rs
mod rave

fn main(){
  let addr = rave::Alloc::string("my &str");
}
```
### 3. Get the value of the string
```rs
mod rave

fn main() {
  let addr = rave::Alloc::string("my &str");
  println!("{}", rave::StrOut::decode(rave::Read::string(addr)).as_str());
}
```
### 4. Delete allocated &str
```rs
mod rave

fn main(){
  let addr = rave::Alloc::string("my &str");
  println!("{}", rave::StrOut::decode(rave::Read::string(addr)).as_str());
  rave::Dealloc::string(addr);
}
```
## License
[MIT]("LICENSE")

---
