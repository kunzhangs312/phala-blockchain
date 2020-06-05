// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::utils;
use heck::{
    CamelCase,
    SnakeCase,
};
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use synstructure::Structure;

pub fn call(s: Structure) -> TokenStream {
    let subxt = utils::use_crate("substrate-subxt");
    let codec = utils::use_crate("parity-scale-codec");
    let ident = &s.ast().ident;
    let generics = &s.ast().generics;
    let params = utils::type_params(generics);
    let module = utils::module_name(generics);
    let with_module = format_ident!(
        "with_{}",
        utils::path_to_ident(module).to_string().to_snake_case()
    );
    let call_name = utils::ident_to_name(ident, "Call").to_snake_case();
    let bindings = utils::bindings(&s);
    let fields = utils::fields(&bindings);
    let marker = utils::marker_field(&fields).unwrap_or_else(|| format_ident!("_"));
    let filtered_fields = utils::filter_fields(&fields, &marker);
    let args = utils::fields_to_args(&filtered_fields);
    let build_struct = utils::build_struct(ident, &fields);
    let call_trait = format_ident!("{}CallExt", call_name.to_camel_case());
    let call = format_ident!("{}", call_name);
    let call_and_watch = format_ident!("{}_and_watch", call_name);

    quote! {
        impl#generics #subxt::Call<T> for #ident<#(#params),*> {
            const MODULE: &'static str = MODULE;
            const FUNCTION: &'static str = #call_name;
            fn events_decoder(
                decoder: &mut #subxt::EventsDecoder<T>,
            ) -> Result<(), #subxt::EventsError> {
                decoder.#with_module()?;
                Ok(())
            }
        }

        /// Call extension trait.
        pub trait #call_trait<T: #module, S: #codec::Encode, E: #subxt::SignedExtra<T>> {
            /// Create and submit an extrinsic.
            fn #call<'a>(
                &'a self,
                signer: &'a (dyn #subxt::Signer<T, S, E> + Send + Sync),
                #args
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<T::Hash, #subxt::Error>> + Send + 'a>>;

            /// Create, submit and watch an extrinsic.
            fn #call_and_watch<'a>(
                &'a self,
                signer: &'a (dyn #subxt::Signer<T, S, E> + Send + Sync),
                #args
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<#subxt::ExtrinsicSuccess<T>, #subxt::Error>> + Send + 'a>>;
        }

        impl<T, S, E> #call_trait<T, S, E> for #subxt::Client<T, S, E>
        where
            T: #module + #subxt::system::System + Send + Sync + 'static,
            S: #codec::Encode + Send + Sync + 'static,
            E: #subxt::SignedExtra<T> + #subxt::sp_runtime::traits::SignedExtension + Send + Sync + 'static,
            <<E as #subxt::SignedExtra<T>>::Extra as #subxt::sp_runtime::traits::SignedExtension>::AdditionalSigned: Send + Sync,
        {
            fn #call<'a>(
                &'a self,
                signer: &'a (dyn #subxt::Signer<T, S, E> + Send + Sync),
                #args
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<T::Hash, #subxt::Error>> + Send + 'a>> {
                let #marker = core::marker::PhantomData::<T>;
                Box::pin(self.submit(#build_struct, signer))
            }

            fn #call_and_watch<'a>(
                &'a self,
                signer: &'a (dyn #subxt::Signer<T, S, E> + Send + Sync),
                #args
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<#subxt::ExtrinsicSuccess<T>, #subxt::Error>> + Send + 'a>> {
                let #marker = core::marker::PhantomData::<T>;
                Box::pin(self.watch(#build_struct, signer))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_call() {
        let input = quote! {
            #[derive(Call, Encode)]
            pub struct TransferCall<'a, T: Balances> {
                pub to: &'a <T as System>::Address,
                #[codec(compact)]
                pub amount: T::Balance,
            }
        };
        let expected = quote! {
            impl<'a, T: Balances> substrate_subxt::Call<T> for TransferCall<'a, T> {
                const MODULE: &'static str = MODULE;
                const FUNCTION: &'static str = "transfer";
                fn events_decoder(
                    decoder: &mut substrate_subxt::EventsDecoder<T>,
                ) -> Result<(), substrate_subxt::EventsError> {
                    decoder.with_balances()?;
                    Ok(())
                }
            }

            /// Call extension trait.
            pub trait TransferCallExt<T: Balances, S: codec::Encode, E: substrate_subxt::SignedExtra<T>> {
                /// Create and submit an extrinsic.
                fn transfer<'a>(
                    &'a self,
                    signer: &'a (dyn substrate_subxt::Signer<T, S, E> + Send + Sync),
                    to: &'a <T as System>::Address,
                    amount: T::Balance,
                ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<T::Hash, substrate_subxt::Error>> + Send + 'a>>;

                /// Create, submit and watch an extrinsic.
                fn transfer_and_watch<'a>(
                    &'a self,
                    signer: &'a (dyn substrate_subxt::Signer<T, S, E> + Send + Sync),
                    to: &'a <T as System>::Address,
                    amount: T::Balance,
                ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<substrate_subxt::ExtrinsicSuccess<T>, substrate_subxt::Error>> + Send + 'a>>;
            }

            impl<T, S, E> TransferCallExt<T, S, E> for substrate_subxt::Client<T, S, E>
            where
                T: Balances + substrate_subxt::system::System + Send + Sync + 'static,
                S: codec::Encode + Send + Sync + 'static,
                E: substrate_subxt::SignedExtra<T> + substrate_subxt::sp_runtime::traits::SignedExtension + Send + Sync + 'static,
                <<E as substrate_subxt::SignedExtra<T>>::Extra as substrate_subxt::sp_runtime::traits::SignedExtension>::AdditionalSigned: Send + Sync,
            {
                fn transfer<'a>(
                    &'a self,
                    signer: &'a (dyn substrate_subxt::Signer<T, S, E> + Send + Sync),
                    to: &'a <T as System>::Address,
                    amount: T::Balance,
                ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<T::Hash, substrate_subxt::Error>> + Send + 'a>> {
                    let _ = core::marker::PhantomData::<T>;
                    Box::pin(self.submit(TransferCall { to, amount, }, signer))
                }

                fn transfer_and_watch<'a>(
                    &'a self,
                    signer: &'a (dyn substrate_subxt::Signer<T, S, E> + Send + Sync),
                    to: &'a <T as System>::Address,
                    amount: T::Balance,
                ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<substrate_subxt::ExtrinsicSuccess<T>, substrate_subxt::Error>> + Send + 'a>> {
                    let _ = core::marker::PhantomData::<T>;
                    Box::pin(self.watch(TransferCall { to, amount, }, signer))
                }
            }
        };
        let derive_input = syn::parse2(input).unwrap();
        let s = Structure::new(&derive_input);
        let result = call(s);
        utils::assert_proc_macro(result, expected);
    }
}