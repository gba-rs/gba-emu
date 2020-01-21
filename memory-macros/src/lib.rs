extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn::{parse_macro_input, DeriveInput, ItemFn, ItemStruct};
use proc_macro2::{Ident, Span};

#[proc_macro_attribute]
pub fn memory_segment(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let attr_string = attr.to_string();
    let segment_size = attr_string.trim().parse::<usize>().unwrap();
    let name = &input.ident;

    let expanded = quote! {
        #input

        impl #name {
            const SEGMENT_SIZE: usize = #segment_size;

            pub fn new() -> #name {
                return #name {
                    memory: Rc::new(RefCell::new(vec![0; #segment_size]))
                };
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
        33..=64 => quote!{u64},
        65..=128 => quote!{u128},
        _ => panic!("Can't do bitfields greater than 128 bits")
    };

    let byte_number = start_bit / 8;
    let offset_start_bit = start_bit - (8 * byte_number);
    
    let expanded = quote! {
        #input

        impl #name {
            pub fn #function_get_ident(&self) -> #min_type {
                let byte = self.memory.borrow()[#byte_number as usize];
                return ((1 << #num_bits) - 1) & (byte >> (#offset_start_bit)) as #min_type;
            }

            pub fn #function_set_ident(&mut self, value: #min_type) {
                if value as u32 > #num_bits.pow(2) {
                    panic!("Attempting to set number out of range of bit field");
                }

                let mut current_val = self.memory.borrow()[#byte_number as usize];
                for i in (#offset_start_bit)..(#num_bits + #offset_start_bit) {
                    current_val &= !(1 << i);
                }

                let shifted_val = (value << #offset_start_bit) as u8;
                self.memory.borrow_mut()[#byte_number as usize] = current_val | shifted_val;
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}