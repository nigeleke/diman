use diman_lib::{cfg_any_fastnum_decimal_type, for_each_fastnum_decimal_type};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use super::Codegen;

pub struct VectorType {
    pub name: Type,
    pub float_type: FloatType,
    pub num_dims: usize,
}

#[derive(Clone, Debug)]
pub struct FloatType {
    pub name: Type,
    pub conversion_method: TokenStream,
    #[cfg(feature = "mpi")]
    pub mpi_type: TokenStream,
    #[cfg(feature = "hdf5")]
    pub hdf5_type: TokenStream,
}

pub trait StorageType {
    /// The name of the type
    fn name(&self) -> &Type;

    /// For vector types, this represents the underlying storage of a
    /// single entry in the vector.
    fn base_storage(&self) -> &FloatType;
}

impl StorageType for VectorType {
    fn name(&self) -> &Type {
        &self.name
    }

    fn base_storage(&self) -> &FloatType {
        &self.float_type
    }
}

impl StorageType for FloatType {
    fn name(&self) -> &Type {
        &self.name
    }

    fn base_storage(&self) -> &FloatType {
        self
    }
}

macro_rules! fastnum_float_type {
    ($feature:literal, $float_type:ty, $mod_name:ident) => {
        self.fastnum_type::<_>()
    };
}

impl Codegen {
    pub fn storage_types(&self) -> impl Iterator<Item = Box<dyn StorageType>> {
        println!("float_types2: {:?}", self.float_types().iter());
        self.float_types()
            .into_iter()
            .map(|x| Box::new(x) as Box<dyn StorageType>)
            .chain(
                self.vector_types()
                    .into_iter()
                    .map(|x| Box::new(x) as Box<dyn StorageType>),
            )
    }

    pub fn storage_type_names(&self) -> impl Iterator<Item = Type> {
        self.storage_types().map(|x| x.name().clone())
    }

    pub fn vector_types(&self) -> Vec<VectorType> {
        // I don't know if this is really the way to construct types
        let _vec2: Type = syn::parse2(quote! { ::glam::Vec2 }).unwrap();
        let _dvec2: Type = syn::parse2(quote! { ::glam::DVec2 }).unwrap();
        let _vec3: Type = syn::parse2(quote! { ::glam::Vec3 }).unwrap();
        let _dvec3: Type = syn::parse2(quote! { ::glam::DVec3 }).unwrap();
        vec![
            #[cfg(feature = "glam-vec2")]
            VectorType {
                name: _vec2,
                float_type: self.f32_type(),
                num_dims: 2,
            },
            #[cfg(feature = "glam-dvec2")]
            VectorType {
                name: _dvec2,
                float_type: self.f64_type(),
                num_dims: 2,
            },
            #[cfg(feature = "glam-vec3")]
            VectorType {
                name: _vec3,
                float_type: self.f32_type(),
                num_dims: 3,
            },
            #[cfg(feature = "glam-dvec3")]
            VectorType {
                name: _dvec3,
                float_type: self.f64_type(),
                num_dims: 3,
            },
        ]
    }

    #[cfg(feature = "f32")]
    fn f32_type(&self) -> FloatType {
        let f32_ty: Type = syn::parse2(quote! { f32 }).unwrap();
        FloatType {
            name: f32_ty,
            conversion_method: quote! { into_f32 },
            #[cfg(feature = "mpi")]
            mpi_type: quote! { ::mpi::ffi::RSMPI_FLOAT },
            #[cfg(feature = "hdf5")]
            hdf5_type: quote! { hdf5::types::FloatSize::U4 },
        }
    }

    #[cfg(feature = "f64")]
    fn f64_type(&self) -> FloatType {
        let f64_ty: Type = syn::parse2(quote! { f64 }).unwrap();
        FloatType {
            name: f64_ty,
            conversion_method: quote! { into_f64 },
            #[cfg(feature = "mpi")]
            mpi_type: quote! { ::mpi::ffi::RSMPI_DOUBLE },
            #[cfg(feature = "hdf5")]
            hdf5_type: quote! { hdf5::types::FloatSize::U8 },
        }
    }

    cfg_any_fastnum_decimal_type! {
        fn fastnum_type<D>(&self) -> FloatType {
            let fastnum_ty: Type = syn::parse2(quote! { D }).unwrap();
            FloatType {
                name: fastnum_ty,
                conversion_method: quote! { into_fastnum },
                // TODO: Determine if it makes sense to support mpi for fastnum.
                #[cfg(feature = "mpi")]
                mpi_type: quote! { ::mpi::ffi::RSMPI_DOUBLE },
                // TODO: Determine if it makes sense to support hdf5 for fastnum.
                #[cfg(feature = "hdf5")]
                hdf5_type: quote! { hdf5::types::FloatSize::U64 },
            }
        }
    }

    pub fn float_types(&self) -> Vec<FloatType> {
        vec![
            #[cfg(feature = "f32")]
            self.f32_type(),
            #[cfg(feature = "f64")]
            self.f64_type(),
            #[cfg(feature = "fastnum-d64")]
            self.fastnum_type::<fastnum::D64>(),
            // #[cfg(feature = "fastnum-d128")]
            // self.fastnum_type::<fastnum::D128>(),
            // #[cfg(feature = "fastnum-d256")]
            // self.fastnum_type::<fastnum::D256>(),
            // #[cfg(feature = "fastnum-d512")]
            // self.fastnum_type::<fastnum::D512>(),
            // #[cfg(feature = "fastnum-d1024")]
            // self.fastnum_type::<fastnum::D1024>(),
        ]
    }
}
