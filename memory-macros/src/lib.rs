extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn multiple_memory_segment(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let attr_string = attr.to_string();
    let mut attrs: Vec<&str> = attr_string.split(",").collect();
    let mut segment_indicies: Vec<usize> = vec![];
    let segment_size = attrs[0].trim().parse::<usize>().unwrap();
    attrs.remove(0);

    for attr in attrs {
        segment_indicies.push(usize::from_str_radix(attr.trim().trim_start_matches("0x"), 16).unwrap());
    }

    let segment_indicies_len = segment_indicies.len();

    let name = &input.ident;

    if segment_size != 1 && segment_size != 2 && segment_size != 4 {
        panic!("Unsupported segment size: {}", segment_size);
    }

    let segment_type = match segment_size {
        1 => quote!{u8},
        2 => quote!{u16},
        4 => quote!{u32},
        _ => panic!("Unsupported segment size: {}", segment_size)
    };

    let expanded = quote! {
        #input

        impl #name {
            pub const SEGMENT_SIZE: usize = #segment_size;
            pub const SEGMENT_INDICIES: [usize; #segment_indicies_len] = [#(#segment_indicies),*];

            pub fn new(index: usize) -> #name {
                return #name {
                    memory: Rc::new(RefCell::new(vec![0; #segment_size])),
                    index: index
                };
            }

            pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
                self.memory = mem.clone();
            }

            pub fn get_register(&self) -> #segment_type {
                let mut value: #segment_type = 0;
                let mem_ref = self.memory.borrow();
                for i in 0..#name::SEGMENT_SIZE {
                    value |= (mem_ref[#name::SEGMENT_INDICIES[self.index] + (i as usize)] as #segment_type) <<  (i * 8);
                }

                return value;
            }

            pub fn set_register(&self, value: u32) {
                let mut mem_ref = self.memory.borrow_mut();
                for i in 0..#name::SEGMENT_SIZE {
                    mem_ref[#name::SEGMENT_INDICIES[self.index] + (i as usize)] = ((value & (0xFFu32 << (i * 8))) >> (i * 8)) as u8;
                }
            }
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_attribute]
pub fn memory_segment(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let attr_string = attr.to_string();
    let attrs: Vec<&str> = attr_string.split(",").collect();
    let segment_size = attrs[0].trim().parse::<usize>().unwrap();
    let segment_index = usize::from_str_radix(attrs[1].trim().trim_start_matches("0x"), 16).unwrap();
    let name = &input.ident;

    if segment_size != 1 && segment_size != 2 && segment_size != 4 {
        panic!("Unsupported segment size: {}", segment_size);
    }

    let segment_type = match segment_size {
        1 => quote!{u8},
        2 => quote!{u16},
        4 => quote!{u32},
        _ => panic!("Unsupported segment size: {}", segment_size)
    };

    let expanded = quote! {
        #input

        impl #name {
            pub const SEGMENT_SIZE: usize = #segment_size;
            pub const SEGMENT_INDEX: usize = #segment_index;

            pub fn new() -> #name {
                return #name {
                    memory: Rc::new(RefCell::new(vec![0; #segment_size]))
                };
            }

            pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
                self.memory = mem.clone();
            }

            pub fn get_register(&self) -> #segment_type {
                let mut value: #segment_type = 0;
                let mem_ref = self.memory.borrow();
                for i in 0..#name::SEGMENT_SIZE {
                    value |= (mem_ref[#name::SEGMENT_INDEX + (i as usize)] as #segment_type) <<  (i * 8);
                }

                return value;
            }

            pub fn set_register(&self, value: u32) {
                let mut mem_ref = self.memory.borrow_mut();
                for i in 0..#name::SEGMENT_SIZE {
                    mem_ref[#name::SEGMENT_INDEX + (i as usize)] = ((value & (0xFFu32 << (i * 8))) >> (i * 8)) as u8;
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn bit_field(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let attr_string = attr.to_string();
    let attrs: Vec<&str> = attr_string.split(",").collect();
    let start_bit = attrs[1].trim().parse::<u32>().unwrap();
    let num_bits = attrs[2].trim().parse::<u32>().unwrap();

    // Create the idents
    let function_get_ident = format_ident!("get_{}", attrs[0]);
    let function_set_ident = format_ident!("set_{}", attrs[0]);
    let name = &input.ident;
    let min_type = match num_bits {
        1..=8 => quote!{u8},
        9..=16 => quote!{u16},
        17..=32 => quote!{u32},
        _ => panic!("Can't do bitfields greater than 32 bits")
    };
    
    let expanded = quote! {
        #input

        impl #name {
            pub fn #function_get_ident(&self) -> #min_type {
                let value: u32 = self.get_register() as u32;
                return (((1u32 << #num_bits) - 1u32) & (value >> (#start_bit))) as #min_type;
            }

            pub fn #function_set_ident(&mut self, value: #min_type) {
                if value as u32 > 2u32.pow(#num_bits) {
                    panic!("Attempting to set number out of range of bit field: {}", value);
                }

                let mut current_val: u32 = self.get_register() as u32;
                let shifted_val: u32 = ((value as u32) << #start_bit) as u32;

                for i in #start_bit..(#num_bits + #start_bit) {
                    current_val &= !(1 << i);
                }

                self.set_register(current_val | shifted_val);
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}