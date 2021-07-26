extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

/// Exposes a `jamsocket_wasm::SimpleJamsocketService`-implementing trait as a WebAssembly module.
#[proc_macro_attribute]
pub fn jamsocket_wasm(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Extract the name of the wrapped entity.
    let name = {
        let parsed: ItemStruct = syn::parse(item.clone())
        .expect("The #[jamsocket_wasm] annotation can only be applied to a struct that implements the `JamsocketServer` trait.");
        parsed.ident
    };

    let item_tokens: proc_macro2::TokenStream = item.into();

    (quote! {
        #item_tokens

        mod _jamsocket_wasm_macro_autogenerated {
            extern crate alloc;

            use super::#name;
            use jamsocket_wasm::prelude::{
                WrappedJamsocketService,
                MessageRecipient,
                JamsocketServiceBuilder,
                JamsocketService,
                JamsocketContext
            };

            // Instance-global jamsocket service.
            static mut SERVER_STATE: Option<WrappedJamsocketService<#name, GlobalJamsocketContext>> = None;

            #[no_mangle]
            pub static JAMSOCKET_API_VERSION: i32 = 1;

            #[no_mangle]
            pub static JAMSOCKET_API_PROTOCOL: i32 = 0;

            struct GlobalJamsocketContext;

            impl JamsocketContext for GlobalJamsocketContext {
                fn set_timer(&self, ms_delay: u32) {
                    unsafe {
                        ffi::set_timer(ms_delay);
                    }
                }

                fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str) {
                    unsafe {
                        ffi::send_message(
                            recipient.into().encode_u32(),
                            &message.as_bytes()[0] as *const u8 as u32,
                            message.len() as u32,
                        );
                    }
                }

                fn send_binary(&self, recipient: impl Into<MessageRecipient>, message: &[u8]) {
                    unsafe {
                        ffi::send_binary(
                            recipient.into().encode_u32(),
                            &message[0] as *const u8 as u32,
                            message.len() as u32,
                        );
                    }
                }
            }

            // Functions implemented by the host.
            mod ffi {
                extern "C" {
                    pub fn send_message(user: u32, message: u32, message_len: u32);

                    pub fn send_binary(user: u32, message: u32, message_len: u32);

                    pub fn set_timer(ms_delay: u32);
                }
            }

            // Functions provided to the host.
            #[no_mangle]
            extern "C" fn initialize() {
                let mut c = JamsocketServiceBuilder::build(#name::default(), "", GlobalJamsocketContext);

                unsafe {
                    SERVER_STATE.replace(c);
                }
            }

            #[no_mangle]
            extern "C" fn connect(user: u32) {
                match unsafe { SERVER_STATE.as_mut() } {
                    Some(st) => JamsocketService::connect(st, user),
                    None => ()
                }
            }

            #[no_mangle]
            extern "C" fn disconnect(user: u32) {
                match unsafe { SERVER_STATE.as_mut() } {
                    Some(st) => JamsocketService::disconnect(st, user),
                    None => ()
                }
            }

            #[no_mangle]
            extern "C" fn timer() {
                match unsafe { SERVER_STATE.as_mut() } {
                    Some(st) => JamsocketService::timer(st),
                    None => ()
                }
            }

            #[no_mangle]
            extern "C" fn message(user: u32, ptr: *const u8, len: usize) {
                unsafe {
                    let string = String::from_utf8(std::slice::from_raw_parts(ptr, len).to_vec()).map_err(|e| format!("Error parsing UTF-8 from host {:?}", e)).unwrap();

                    match SERVER_STATE.as_mut() {
                        Some(st) => JamsocketService::message(st, user, &string),
                        None => ()
                    }
                }
            }

            #[no_mangle]
            extern "C" fn binary(user: u32, ptr: *const u8, len: usize) {
                unsafe {
                    let data = std::slice::from_raw_parts(ptr, len);

                    match SERVER_STATE.as_mut() {
                        Some(st) => JamsocketService::binary(st, user, data),
                        None => ()
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn malloc(size: u32) -> *mut u8 {
                let layout = core::alloc::Layout::from_size_align_unchecked(size as usize, 0);
                alloc::alloc::alloc(layout)
            }

            #[no_mangle]
            pub unsafe extern "C" fn free(ptr: *mut u8, size: u32) {
                let layout = core::alloc::Layout::from_size_align_unchecked(size as usize, 0);
                alloc::alloc::dealloc(ptr, layout);
            }
        }
    }).into()
}
