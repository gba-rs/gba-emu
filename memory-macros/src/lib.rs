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
}

struct BitField {
    name: Ident,
    start_bit: u8,
    num_bits: u8,
    min_type: proc_macro2::TokenStream,
}

impl Parse for BitField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let start_bit_lit: Lit = input.parse()?;
        input.parse::<Token![,]>()?;
        let num_bits_lit: Lit = input.parse()?;

        let start_bit = lit_to_u8(&start_bit_lit)?;
        let num_bits = lit_to_u8(&num_bits_lit)?;
        let min_type = min_type_for(num_bits, &num_bits_lit)?;

        Ok(BitField {
            name,
            start_bit,
            num_bits,
            min_type,
        })
    }
}

impl ToTokens for BitField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let get_ident = format_ident!("get_{}", self.name);
        let set_ident = format_ident!("set_{}", self.name);

        let start_bit = self.start_bit;
        let num_bits = self.num_bits;
        let min_type = &self.min_type;

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

        let fields = parse_fields(input)?;

        Ok(IORegister {
            segment_address,
            fields
        })
    }
}

struct MultipleAddressIORegister {
    segment_addresses: Expr,
    num_elements: usize,
    fields: Vec<BitField>,
}

impl Parse for MultipleAddressIORegister {
    fn parse(input: ParseStream) -> Result<Self> {
        let segment_addresses: Expr = input.parse()?;
        let num_elements = match &segment_addresses {
            Expr::Array(array) => array.elems.len(),
            other => return Err(syn::Error::new_spanned(other, "expected an array literal of addresses")),
        };
        input.parse::<Token![,]>()?;

        let fields = parse_fields(input)?;

        Ok(MultipleAddressIORegister {
            segment_addresses,
            num_elements,
            fields
        })
    }
}

struct BaseIORegister {
    name: Ident,
    segment_size: Lit,
    segment_type: proc_macro2::TokenStream,
    option: BitFieldOption
}

impl Parse for BaseIORegister {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let segment_size = input.parse::<Lit>()?;
        input.parse::<syn::token::Comma>()?;

        let segment_size_int = lit_to_u8(&segment_size)?;
        let segment_type = segment_type_for(segment_size_int, &segment_size)?;

        let lookahead = input.lookahead1();
        let option = if lookahead.peek(Bracket) {
            BitFieldOption::MultipleAddress(MultipleAddressIORegister::parse(input)?)
        } else if lookahead.peek(Lit) {
            BitFieldOption::SingleAddress(IORegister::parse(input)?)
        } else {
            return Err(lookahead.error());
        };

        Ok(BaseIORegister {
            name,
            segment_size,
            segment_type,
            option
        })
    }
}

fn parse_fields(input: ParseStream) -> Result<Vec<BitField>> {
    let mut fields: Vec<BitField> = vec![];

    loop {
        fields.push(BitField::parse(input)?);

        match input.parse::<Token![,]>() {
            Ok(_) => {
                if input.is_empty() {
                    break
                }
            },
            _ => break
        }
    }

    Ok(fields)
}

fn segment_type_for(segment_size: u8, segment_size_lit: &Lit) -> Result<proc_macro2::TokenStream> {
    match segment_size {
        1 => Ok(quote!{u8}),
        2 => Ok(quote!{u16}),
        4 => Ok(quote!{u32}),
        _ => Err(syn::Error::new_spanned(segment_size_lit, "unsupported segment size (must be 1, 2, or 4)")),
    }
}

fn min_type_for(num_bits: u8, num_bits_lit: &Lit) -> Result<proc_macro2::TokenStream> {
    match num_bits {
        1..=8 => Ok(quote!{u8}),
        9..=16 => Ok(quote!{u16}),
        17..=32 => Ok(quote!{u32}),
        _ => Err(syn::Error::new_spanned(num_bits_lit, "can't do bitfields greater than 32 bits")),
    }
}

fn lit_to_u8(lit: &Lit) -> Result<u8> {
    match lit {
        Lit::Int(int) => int.base10_parse::<u8>(),
        _ => Err(syn::Error::new_spanned(lit, "expected an integer literal")),
    }
}

fn create_bit_field(name: Ident, segment_size: Lit, segment_type: proc_macro2::TokenStream, bit_fields: &IORegister) -> TokenStream {

    let IORegister {
        segment_address,
        fields
    } = bit_fields;

    let expanded = quote! {
        #[derive(Default, Serialize, Deserialize)]
        pub struct #name {
            #[serde(skip)]
            pub memory: Option<Rc<GbaMem>>,
        }

        impl #name {
            pub const SEGMENT_SIZE: usize = #segment_size;
            pub const SEGMENT_INDEX: usize = #segment_address;

            pub fn new() -> #name {
                return #name {
                    memory: None,
                };
            }

            pub fn register(&mut self, mem: &Rc<GbaMem>) {
                self.memory = Some(mem.clone());
            }

            pub fn get_register(&self) -> #segment_type {
                let mut value: #segment_type = 0;
                if let Some(mem) = &self.memory {
                    for i in 0..#name::SEGMENT_SIZE {
                        value |= (mem[#name::SEGMENT_INDEX + (i as usize)].get() as #segment_type) <<  (i * 8);
                    }
                } else {
                    panic!("IO register was accessed without being registered");
                }

                return value;
            }

            pub fn set_register(&self, value: u32) {
                if let Some(mem) = &self.memory {
                    for i in 0..#name::SEGMENT_SIZE {
                        mem[#name::SEGMENT_INDEX + (i as usize)].set(((value & (0xFFu32 << (i * 8))) >> (i * 8)) as u8);
                    }
                } else {
                    panic!("IO register was accessed without being registered");
                }
            }

            #(#fields)*
        }
    };

    return expanded.into();
}

fn create_multiple_bit_field(name: Ident, segment_size: Lit, segment_type: proc_macro2::TokenStream, bit_fields: &MultipleAddressIORegister) -> TokenStream {

    let MultipleAddressIORegister {
        segment_addresses,
        num_elements,
        fields
    } = bit_fields;

    let expanded = quote! {
        #[derive(Default, Serialize, Deserialize)]
        pub struct #name {
            #[serde(skip)]
            pub memory: Option<Rc<GbaMem>>,
            pub index: usize
        }

        impl #name {
            pub const SEGMENT_SIZE: usize = #segment_size;
            pub const SEGMENT_INDICIES: [usize; #num_elements] = #segment_addresses;

            pub fn new(index: usize) -> #name {
                return #name {
                    memory: None,
                    index: index
                };
            }

            pub fn register(&mut self, mem: &Rc<GbaMem>) {
                self.memory = Some(mem.clone());
            }

            pub fn get_register(&self) -> #segment_type {
                let mut value: #segment_type = 0;
                if let Some(mem) = &self.memory {
                    for i in 0..#name::SEGMENT_SIZE {
                        value |= (mem[#name::SEGMENT_INDICIES[self.index] + (i as usize)].get() as #segment_type) <<  (i * 8);
                    }
                }

                return value;
            }

            pub fn set_register(&self, value: u32) {
                if let Some(mem) = &self.memory {
                    for i in 0..#name::SEGMENT_SIZE {
                        mem[#name::SEGMENT_INDICIES[self.index] + (i as usize)].set(((value & (0xFFu32 << (i * 8))) >> (i * 8)) as u8);
                    }
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
        segment_type,
        option
    } = parse_macro_input!(input as BaseIORegister);

    match option {
        BitFieldOption::SingleAddress(register) => {
            return create_bit_field(name, segment_size, segment_type, &register);
        },
        BitFieldOption::MultipleAddress(register) => {
            return create_multiple_bit_field(name, segment_size, segment_type, &register);
        },
    }
}

#[proc_macro]
pub fn gen_obj_array(_: TokenStream) -> TokenStream {

    let mut obj_array_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    for i in 0usize..128usize {
        let token_stream = quote!{
            Object {
                attr0: ObjAttribute0::new(#i),
                attr1: ObjAttribute1::new(#i),
                attr2: ObjAttribute2::new(#i)
            },
        };
        obj_array_tokens.push(token_stream);
    }


    let expanded = quote!{
        macro_rules! obj_array {
            () => {
                [
                    #(#obj_array_tokens)*
                ]
            };
        }
    };

    return expanded.into();
}


#[proc_macro]
pub fn gen_aff_matrix_array(_: TokenStream) -> TokenStream {

    let mut aff_matrix_array_tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    for i in (0usize..128usize).step_by(4) {
        let pa = i;
        let pb = i+1;
        let pc = i+2;
        let pd = i+3;
        let token_stream = quote!{
            AffineMatrix{
                pa: OBJRotScaleParam::new(#pa),
                pb: OBJRotScaleParam::new(#pb),
                pc: OBJRotScaleParam::new(#pc),
                pd: OBJRotScaleParam::new(#pd)
            },
        };
        aff_matrix_array_tokens.push(token_stream);
    }


    let expanded = quote!{
        macro_rules! aff_matrix_array {
            () => {
                [
                    #(#aff_matrix_array_tokens)*
                ]
            };
        }
    };

    return expanded.into();
}
