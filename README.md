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


## License
[MIT]("LICENSE")

---
