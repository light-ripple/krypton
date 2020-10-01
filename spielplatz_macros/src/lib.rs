use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};
use quote::quote;

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemStruct);
	let name = &input.ident;
	
	// Massive Cancer
	let id = attr.to_string().parse::<u16>().unwrap();
	
	TokenStream::from(quote! {
		#input
	
		impl Packet for #name {
			fn get_id(&self) -> u16 {
				#id
			}
		}
	})
}