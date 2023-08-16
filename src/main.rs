use base64;


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
}
struct Delete;
impl Delete {
    fn int(value: &mut i32) {
        unsafe {
            let ptr =std::ptr::addr_of!(value) as *mut u8;
            ptr.write(0x40);
            *ptr = 0x40;
            *value = 0x40;
            std::mem::forget(value);
        }
    }

    fn string(value: &mut String) {
        unsafe {
            *value = "0x40".to_string();
        }
    }
}


struct Read;
impl Read {
    fn int(addr: *mut i32, var:&mut i32) {
        unsafe {
            *var = *addr;
        }
    }

    #[allow(deprecated)]
    fn string(addr: *mut u16) -> Vec<u16> {
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

            return StrOut::encode(String::from_utf16(&result).expect("Panic"));

        }
    }
}

struct StrOut;
impl StrOut {
    fn encode(plain: String) -> Vec<u16> {
        let mut encoded_data = Vec::new();
    
        for utf16_unit in plain.encode_utf16() {
            let encoded_unit = utf16_unit.wrapping_add(1);
            encoded_data.push(encoded_unit);
        }
    
        encoded_data
    }
    
    fn decode(encoded: Vec<u16>) -> String {
        let mut decoded_data = Vec::new();
    
        for utf16_unit in encoded {
            let decoded_unit = utf16_unit.wrapping_sub(1);
            decoded_data.push(decoded_unit);
        }
    
        return String::from_utf8(base64::decode(String::from_utf16_lossy(&decoded_data)).unwrap()).unwrap();
    }
}

// =======


fn main() {
    /*let adr = Alloc::i16(2421).expect("Panic");
    let mut val: i32=0;                                     
    Read::int(adr, &mut val);                       
    println!("{:?} => {}   !->   {:?}", adr, val, std::ptr::addr_of!(val));                         
    Dealloc::int(adr);                                         
    Delete::int(std::ptr::addr_of!(val) as *mut i32);
    */

    let adr = Alloc::string("Hello").expect("Panic");
    Read::string(adr);
    println!("{:?} => {:?}", adr, Read::string(adr));
    Dealloc::string(adr);
    loop {}
}