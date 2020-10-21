use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};
use quote::quote;

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemStruct);
	let name = &input.ident;
	
	// only proc_macro2 implements quote::ToTokens
	let id = proc_macro2::TokenStream::from(attr);
	
	TokenStream::from(quote! {
		#input
	
		impl Packet for #name {
			fn get_id(&self) -> u16 {
				#id as u16
			}
		}
	})
}