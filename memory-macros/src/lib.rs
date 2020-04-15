#![feature(proc_macro_diagnostic)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Token, Lit, Expr, token::Bracket};
use quote::{ToTokens};

enum BitFieldOption {
    SingleAddress(IORegister),
    MultipleAddress(MultipleAddressIORegister),
    ParseError
}

struct BitField {
    name: Ident,
    start_bit: Lit,
    num_bits: Lit
}

impl Parse for BitField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let start_bit: Lit = input.parse()?;
        input.parse::<Token![,]>()?;
        let num_bits: Lit = input.parse()?;

        Ok(BitField {
            name,
            start_bit,
            num_bits
        })
    }
}

impl ToTokens for BitField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let get_ident = format_ident!("get_{}", self.name);
        let set_ident = format_ident!("set_{}", self.name);

        let start_bit = parse_start_bit(&self.start_bit, &self.name.span());
        let (num_bits, min_type) = parse_num_bits(&self.num_bits, &self.name.span());

        let value_mask: u32 = (1 << num_bits) - 1;
        let mask: u32 = value_mask << start_bit;
        let clear_mask: u32 = !mask;

        let result = quote! {
            pub fn #get_ident(&self) -> #min_type{
                return (((self.get_register() as u32) >> #start_bit) & #value_mask) as #min_type;
            }

            pub fn #set_ident(&mut self, value: #min_type) {
                let result = ((self.get_register() as u32) & #clear_mask) | (((value as u32) & #value_mask) << #start_bit);
                self.set_register(result as u32);
            }
        };

        result.to_tokens(tokens);
    }
}

struct IORegister {
    segment_address: Lit,
    fields: Vec<BitField>,

}

impl Parse for IORegister {
    fn parse(input: ParseStream) -> Result<Self> {
        let segment_address = input.parse::<Lit>()?;
        input.parse::<Token![,]>()?;

        let fields = parse_fields(input);

        Ok(IORegister {
            segment_address,
            fields
        })
    }
}

struct MultipleAddressIORegister {
    segment_addresses: Expr,
    fields: Vec<BitField>,
}

impl Parse for MultipleAddressIORegister {
    fn parse(input: ParseStream) -> Result<Self> {
        let segment_addresses: Expr = input.parse()?;
        input.parse::<Token![,]>()?;

        let fields = parse_fields(input);

        Ok(MultipleAddressIORegister {
            segment_addresses,
            fields
        })
    }
}

struct BaseIORegister {
    name: Ident,
    segment_size: Lit,
    option: BitFieldOption
}

impl Parse for BaseIORegister {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let segment_size = input.parse::<Lit>()?;
        input.parse::<syn::token::Comma>()?;
        

        let lookahead = input.lookahead1();
        let option = if lookahead.peek(Bracket) {
            BitFieldOption::MultipleAddress(MultipleAddressIORegister::parse(input)?)
        } else if lookahead.peek(Lit) {
            BitFieldOption::SingleAddress(IORegister::parse(input)?)
        } else {
            BitFieldOption::ParseError
        };

        Ok(BaseIORegister {
            name,
            segment_size,
            option
        })
    }
}

fn parse_fields(input: ParseStream) -> Vec<BitField> {
    let mut fields: Vec<BitField> = vec![];

    loop {
        match BitField::parse(input) {
            Ok(field) => {
                fields.push(field);
            }, 
            _ => {
                input.cursor().span().unwrap().error("Unable to parse field").emit();
            }
        }


        match input.parse::<Token![,]>() {
            Ok(_) => {
                if input.is_empty() {
                    break
                }
            },
            _ => break
        }
    }

    return fields;
}

fn parse_segment_size(segment_size: &syn::Lit, span: &proc_macro2::Span) -> (u8, proc_macro2::TokenStream) {
    let segment_size_int = match &segment_size {
        Lit::Int(seg_size_int) => {
            seg_size_int.base10_parse::<u8>().unwrap()
        },
        _ => {
            span.unwrap().error("Segment size has to be an int").emit();
            0u8
        }
    };

    let segment_type = match segment_size_int {
        1 => quote!{u8},
        2 => quote!{u16},
        4 => quote!{u32},
        _ => {
            if let Lit::Int(ref segment_size) = segment_size {
                segment_size.span().unwrap().error("Unsupported segment size").emit();
            }
            panic!("Memory Macro error");
        }
    };

    return (segment_size_int, segment_type);
}

fn parse_num_bits(num_bits_lit: &syn::Lit, span: &proc_macro2::Span) -> (u8, proc_macro2::TokenStream) {

    let num_bits = match &num_bits_lit {
        Lit::Int(num_bits_int) => {
            num_bits_int.base10_parse::<u8>().unwrap()
        }
        _ => {
            span.unwrap().error("Num bits must be an integer").emit();
            0u8
        }
    };

    let min_type = match num_bits {
        1..=8 => quote!{u8},
        9..=16 => quote!{u16},
        17..=32 => quote!{u32},
        _ => {
            if let Lit::Int(ref num_bits_lit) = num_bits_lit {
                num_bits_lit.span().unwrap().error("Can't do bitfields greater than 32 bits").emit();
            }
            panic!("Can't do bitfields greater than 32 bits")
        }
    };

    return (num_bits, min_type);
}

fn parse_start_bit(start_bit_lit: &syn::Lit, span: &proc_macro2::Span) -> u8 {
    return match &start_bit_lit {
        Lit::Int(start_bit_int) => {
            start_bit_int.base10_parse::<u8>().unwrap()
        }
        _ => {
            span.unwrap().error("Start bit must be an integer").emit();
            0u8
        }
    };
}

fn create_bit_field(name: Ident, segment_size: Lit, bit_fields: &IORegister) -> TokenStream {

    let IORegister {
        segment_address,
        fields
    } = bit_fields;

    let (_, segment_type) = parse_segment_size(&segment_size, &name.span());

    let expanded = quote! {
        pub struct #name {
            pub memory: Rc<RefCell<GbaMem>>,
        }

        impl #name {
            pub const SEGMENT_SIZE: usize = #segment_size; 
            pub const SEGMENT_INDEX: usize = #segment_address;

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

            #(#fields)*
        }
    };

    return expanded.into();
}

fn create_multiple_bit_field(name: Ident, segment_size: Lit, bit_fields: &MultipleAddressIORegister) -> TokenStream {

    let MultipleAddressIORegister {
        segment_addresses,
        fields
    } = bit_fields;

    let (_, segment_type) = parse_segment_size(&segment_size, &name.span());

    let num_elements = match &segment_addresses {
        Expr::Array(array) => {
            array.elems.len()
        },
        _ => {
            name.span().unwrap().error("Num bits must be an integer").emit();
            0usize
        }
    };

    let expanded = quote! {
        pub struct #name {
            pub memory: Rc<RefCell<GbaMem>>,
            pub index: usize
        }

        impl #name {
            pub const SEGMENT_SIZE: usize = #segment_size;
            pub const SEGMENT_INDICIES: [usize; #num_elements] = #segment_addresses;

            pub fn new(index: usize) -> #name {
                return #name {
                    memory: Rc::new(RefCell::new(Vec::new())),
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

            #(#fields)*
        }
    };

    return expanded.into();
}

#[proc_macro]
pub fn io_register(input: TokenStream) -> TokenStream {
    let BaseIORegister {
        name,
        segment_size,
        option
    } = parse_macro_input!(input as BaseIORegister);

    match option {
        BitFieldOption::SingleAddress(register) => {
            return create_bit_field(name, segment_size, &register);
        },
        BitFieldOption::MultipleAddress(register) => {
            return create_multiple_bit_field(name, segment_size, &register);
        },
        BitFieldOption::ParseError => {
            name.span().unwrap().error("Error parsing io register").emit();
            panic!("Error parsing io register");
        }
    }
}