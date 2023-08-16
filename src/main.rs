use base64;

// ALLOC
struct Alloc;

#[allow(dead_code)]
impl Alloc {
    fn i8(value: i8) -> Result<*mut i32, Box<dyn std::error::Error>> {
        unsafe {
            let ptr = std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(std::mem::size_of::<i32>(), 1)) as *mut i32;


            if !ptr.is_null() {
                *ptr = value as i32;
                std::mem::drop(value);
                Ok(ptr)
            }
            else {
                Err(Box::new(std::fmt::Error))
            }
        }
    }
    fn i16(value: i16) -> Result<*mut i32, Box<dyn std::error::Error>> {
        unsafe {
            let ptr = std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(std::mem::size_of::<i32>(), 1)) as *mut i32;


            if !ptr.is_null() {
                *ptr = value as i32;
                std::mem::drop(value);
                Ok(ptr)
            }
            else {
                Err(Box::new(std::fmt::Error))
            }
        }
    }

    fn i32(value: i32) -> Result<*mut i32, Box<dyn std::error::Error>> {
        unsafe {
            let ptr = std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(std::mem::size_of::<i32>(), 1)) as *mut i32;


            if !ptr.is_null() {
                *ptr = value as i32;
                std::mem::drop(value);
                Ok(ptr)
            }
            else {
                Err(Box::new(std::fmt::Error))
            }
        }
    }

    #[allow(deprecated)]
    fn string(value: &str) -> Result<*mut u16, Box<dyn std::error::Error>> {
        let value = &(base64::encode(value));
        unsafe {
            let u16_values = sth(value);
            std::mem::drop(value);
            let item_size = std::mem::size_of::<u16>();
            let layout = std::alloc::Layout::from_size_align(u16_values.len() * item_size, item_size).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut u16;
            if !ptr.is_null() {
                for i in 0..u16_values.len() {
                    *ptr.add(i) = u16_values[i];
                }

                Ok(ptr)
            } else {
                Err(Box::new(std::fmt::Error))
            }
        }
    }
}

struct Dealloc;
impl Dealloc {
    fn int(addr: *mut i32){
        unsafe {
            std::alloc::dealloc(addr as *mut u8, std::alloc::Layout::from_size_align_unchecked(std::mem::size_of::<i32>(), 1));
            std::mem::drop(addr);
        }
    }
    fn string(addr: *mut u16) {
        unsafe {
            let mut bytearray: Vec<u16> = vec![];
            let item_size = std::mem::size_of::<u16>();
            let mut i=0;
            while ((*addr.add(i)) as u16).to_string().ends_with("3") {
                bytearray.push(
                    (*addr.add(i)) as u16
                );
                i+=1;
            }

            
            let layout = std::alloc::Layout::from_size_align(bytearray.len() * item_size, item_size).unwrap();
            std::mem::drop(bytearray);
            std::alloc::dealloc(addr as *mut u8, layout);
        }
    }
    fn delete(addr: *mut i32) {
        unsafe {
            *addr = 0x40;
        }
    }
}

fn sth(input: &str) -> Vec<u16> {
    let bytearr: Vec<u16> = input.encode_utf16().collect();

    let mut result: Vec<u16> = vec![];

    for byte in bytearr {
        let mut new = byte.to_string();
        new.push('3');
        result.push(
            u16::from_str_radix(&new, 10).expect("Panic")
        )
    }

    std::mem::drop(input);
    return result;
}




struct Read;
impl Read {
    fn int(addr: *mut u8, var:&mut i32) {
        unsafe {
            *var = *addr as i32;
        }
    }

    #[allow(deprecated)]
    fn string(addr: *mut u16, var: &mut String) {
        unsafe {
            let mut bytearray: Vec<u16> = vec![];
            let mut result: Vec<u16> = vec![];
            let mut i = 0;
            
            while ((*addr.add(i)) as u16).to_string().ends_with("3") {
                bytearray.push(*addr.add(i));
                i += 1;
            }

            for byte in &bytearray {
                let mut bstr = byte.to_string();
                bstr.pop();
                result.push(u16::from_str_radix(&bstr, 10).expect("Panic"));
            }


            std::mem::forget(bytearray);
            let x = &(String::from_utf16(&result).expect("Panic"));
            let y = base64::decode(x).unwrap();
            std::mem::drop(x);

            *var = String::from_utf8(y).unwrap();
        }
    }
}

// =======


fn main() {
    /*let adr = Alloc::i16(2421).expect("Panic");  //allochiamo i16
    let mut val: i32=0;                                        // creiamo una var per tenere il valore nella heap
    Read::int(adr, &mut val);                        // leggiamo il valore nella var
    println!("{:?} => {}   !->   {:?}", adr, val, std::ptr::addr_of!(val));                          // print dell'indirizzo e del valore
    dealloc(adr);                                         // deallochiamo l'i16
    delete(std::ptr::addr_of!(val) as *mut i32);               // eliminiamo la variabile
    */
    let adr = Alloc::string("Hello").expect("Panic");
    let mut val:String="".to_string();
    Read::string(adr, &mut val);
    println!("{:?} => {}    !->   {:?}", adr, val, std::ptr::addr_of!(val));
    Dealloc::string(adr);
    Dealloc::delete(std::ptr::addr_of!(val) as *mut i32);
    loop {}
}