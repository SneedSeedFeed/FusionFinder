extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};


// Now unused macro I was using to generate the different closures for the stat sliders
// Honestly I kind of miss it since the code was more static and easier to read but harder/impossible to split off to a generic component nicely
// Couldve created a macro to generate the component itself but eh lazy
#[proc_macro]
pub fn stat_change(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stat_ident = Ident::new(
        &parse_macro_input!(input as LitStr).value(),
        proc_macro2::Span::call_site(),
    );

    let value_getter = if stat_ident == "min_bst" {
        quote! {
            let value = event_target_value(&event).parse::<u16>().unwrap();
        }
    } else {
        quote! {
            let value = event_target_value(&event).parse::<u8>().unwrap();
        }
    };

    quote! {
        move |event| {
            #value_getter
            set_filters.update(|filters| filters.#stat_ident = value);
        }
    }
    .into()
}
