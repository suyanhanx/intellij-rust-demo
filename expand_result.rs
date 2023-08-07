#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate napi_derive;
use napi::bindgen_prelude::*;
pub struct Foo {}
impl napi::bindgen_prelude::TypeName for Foo {
    fn type_name() -> &'static str {
        "Foo"
    }
    fn value_type() -> napi::ValueType {
        napi::ValueType::Function
    }
}
impl napi::bindgen_prelude::TypeName for &Foo {
    fn type_name() -> &'static str {
        "Foo"
    }
    fn value_type() -> napi::ValueType {
        napi::ValueType::Object
    }
}
impl napi::bindgen_prelude::TypeName for &mut Foo {
    fn type_name() -> &'static str {
        "Foo"
    }
    fn value_type() -> napi::ValueType {
        napi::ValueType::Object
    }
}
impl napi::bindgen_prelude::ToNapiValue for Foo {
    unsafe fn to_napi_value(
        env: napi::sys::napi_env,
        val: Foo,
    ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
        if let Some(ctor_ref) = napi::__private::get_class_constructor("Foo\0") {
            let wrapped_value = Box::into_raw(Box::new(val));
            let instance_value = Foo::new_instance(
                env,
                wrapped_value as *mut std::ffi::c_void,
                ctor_ref,
            )?;
            Ok(instance_value)
        } else {
            Err(
                napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}` in `ToNapiValue`",
                                "Foo"
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
}
impl napi::bindgen_prelude::ObjectFinalize for Foo {}
impl Foo {
    pub fn instance_of<V: napi::NapiRaw>(
        env: napi::Env,
        value: V,
    ) -> napi::Result<bool> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Foo\0") {
            let mut ctor = std::ptr::null_mut();
            {
                let c = unsafe {
                    napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor)
                };
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get constructor reference of class `{0}`",
                                            "Foo\0"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            let mut is_instance_of = false;
            {
                let c = unsafe {
                    napi::sys::napi_instanceof(
                        env.raw(),
                        value.raw(),
                        ctor,
                        &mut is_instance_of,
                    )
                };
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to run instanceof for class `{0}`", "Foo\0"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            Ok(is_instance_of)
        } else {
            Err(
                napi::Error::new(
                    napi::Status::GenericFailure,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`", "Foo\0"
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
}
impl Foo {
    pub fn into_reference(
        val: Foo,
        env: napi::Env,
    ) -> napi::Result<napi::bindgen_prelude::Reference<Foo>> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Foo\0") {
            unsafe {
                let wrapped_value = Box::into_raw(Box::new(val));
                let instance_value = Foo::new_instance(
                    env.raw(),
                    wrapped_value as *mut std::ffi::c_void,
                    ctor_ref,
                )?;
                {
                    let env = env.raw();
                }
                napi::bindgen_prelude::Reference::<
                    Foo,
                >::from_value_ptr(wrapped_value as *mut std::ffi::c_void, env.raw())
            }
        } else {
            Err(
                napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`", "Foo"
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
    pub fn into_instance(
        self,
        env: napi::Env,
    ) -> napi::Result<napi::bindgen_prelude::ClassInstance<Foo>> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Foo\0") {
            unsafe {
                let wrapped_value = Box::leak(Box::new(self));
                let instance_value = Foo::new_instance(
                    env.raw(),
                    wrapped_value as *mut _ as *mut std::ffi::c_void,
                    ctor_ref,
                )?;
                Ok(
                    napi::bindgen_prelude::ClassInstance::<
                        Foo,
                    >::new(instance_value, wrapped_value),
                )
            }
        } else {
            Err(
                napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`", "Foo"
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
    unsafe fn new_instance(
        env: napi::sys::napi_env,
        wrapped_value: *mut std::ffi::c_void,
        ctor_ref: napi::sys::napi_ref,
    ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
        let mut ctor = std::ptr::null_mut();
        {
            let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to get constructor reference of class `{0}`", "Foo"
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        let mut result = std::ptr::null_mut();
        napi::__private::___CALL_FROM_FACTORY
            .with(|inner| inner.store(true, std::sync::atomic::Ordering::Relaxed));
        {
            let c = napi::sys::napi_new_instance(
                env,
                ctor,
                0,
                std::ptr::null_mut(),
                &mut result,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("Failed to construct class `{0}`", "Foo"),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        napi::__private::___CALL_FROM_FACTORY
            .with(|inner| inner.store(false, std::sync::atomic::Ordering::Relaxed));
        let mut object_ref = std::ptr::null_mut();
        let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
        let finalize_callbacks_ptr = std::rc::Rc::into_raw(
            std::rc::Rc::new(std::cell::Cell::new(Box::into_raw(initial_finalize))),
        );
        {
            let c = napi::sys::napi_wrap(
                env,
                result,
                wrapped_value,
                Some(napi::bindgen_prelude::raw_finalize_unchecked::<Foo>),
                std::ptr::null_mut(),
                &mut object_ref,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to wrap native object of class `{0}`", "Foo"
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        napi::bindgen_prelude::Reference::<
            Foo,
        >::add_ref(
            env,
            wrapped_value,
            (wrapped_value, object_ref, finalize_callbacks_ptr),
        );
        Ok(result)
    }
}
impl napi::bindgen_prelude::FromNapiRef for Foo {
    unsafe fn from_napi_ref(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<&'static Self> {
        let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_unwrap(
                env,
                napi_val,
                &mut wrapped_val,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to recover `{0}` type from napi value", "Foo"
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        Ok(&*(wrapped_val as *const Foo))
    }
}
impl napi::bindgen_prelude::FromNapiMutRef for Foo {
    unsafe fn from_napi_mut_ref(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<&'static mut Self> {
        let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_unwrap(
                env,
                napi_val,
                &mut wrapped_val,
            );
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to recover `{0}` type from napi value", "Foo"
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        Ok(&mut *(wrapped_val as *mut Foo))
    }
}
impl napi::bindgen_prelude::FromNapiValue for &Foo {
    unsafe fn from_napi_value(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<Self> {
        napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
    }
}
impl napi::bindgen_prelude::FromNapiValue for &mut Foo {
    unsafe fn from_napi_value(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<Self> {
        napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
    }
}
impl napi::bindgen_prelude::ValidateNapiValue for &Foo {
    unsafe fn validate(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> napi::Result<napi::sys::napi_value> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Foo\0") {
            let mut ctor = std::ptr::null_mut();
            {
                let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get constructor reference of class `{0}`", "Foo"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            let mut is_instance_of = false;
            {
                let c = napi::sys::napi_instanceof(
                    env,
                    napi_val,
                    ctor,
                    &mut is_instance_of,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get external value of class `{0}`", "Foo"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            if is_instance_of {
                Ok(std::ptr::null_mut())
            } else {
                Err(
                    napi::Error::new(
                        napi::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("Value is not instanceof class `{0}`", "Foo"),
                            );
                            res
                        },
                    ),
                )
            }
        } else {
            Err(
                napi::Error::new(
                    napi::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`", "Foo"
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
}
impl napi::bindgen_prelude::ValidateNapiValue for &mut Foo {
    unsafe fn validate(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> napi::Result<napi::sys::napi_value> {
        if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("Foo\0") {
            let mut ctor = std::ptr::null_mut();
            {
                let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get constructor reference of class `{0}`", "Foo"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            let mut is_instance_of = false;
            {
                let c = napi::sys::napi_instanceof(
                    env,
                    napi_val,
                    ctor,
                    &mut is_instance_of,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get external value of class `{0}`", "Foo"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            if is_instance_of {
                Ok(std::ptr::null_mut())
            } else {
                Err(
                    napi::Error::new(
                        napi::Status::InvalidArg,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("Value is not instanceof class `{0}`", "Foo"),
                            );
                            res
                        },
                    ),
                )
            }
        } else {
            Err(
                napi::Error::new(
                    napi::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to get constructor of class `{0}`", "Foo"
                            ),
                        );
                        res
                    },
                ),
            )
        }
    }
}
impl napi::NapiRaw for &Foo {
    unsafe fn raw(&self) -> napi::sys::napi_value {
        ::core::panicking::panic("internal error: entered unreachable code")
    }
}
impl napi::NapiRaw for &mut Foo {
    unsafe fn raw(&self) -> napi::sys::napi_value {
        ::core::panicking::panic("internal error: entered unreachable code")
    }
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
mod __napi_helper__Foo {
    use std::ptr;
    use super::*;
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    #[cfg(all(not(test), not(feature = "noop"), not(target_arch = "wasm32")))]
    extern fn __napi_register__Foo_struct_0() {
        napi::__private::register_class("Foo", None, "Foo\0", ::alloc::vec::Vec::new());
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __napi_register__Foo_struct_0___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
        unsafe extern "C" fn __napi_register__Foo_struct_0___rust_ctor___ctor() -> usize {
            __napi_register__Foo_struct_0();
            0
        }
        __napi_register__Foo_struct_0___rust_ctor___ctor
    };
}
impl Foo {
    pub fn new() -> Result<Self> {
        {
            ::std::io::_print(format_args!("new\n"));
        };
        Ok(Self {})
    }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod __napi_impl_helper__Foo__0 {
    use super::*;
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    extern "C" fn __napi__new(
        env: napi::bindgen_prelude::sys::napi_env,
        cb: napi::bindgen_prelude::sys::napi_callback_info,
    ) -> napi::bindgen_prelude::sys::napi_value {
        unsafe {
            if napi::__private::___CALL_FROM_FACTORY
                .with(|inner| inner.load(std::sync::atomic::Ordering::Relaxed))
            {
                return std::ptr::null_mut();
            }
            napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None)
                .and_then(|mut cb| {
                    napi::bindgen_prelude::within_runtime_if_available(move || {
                        let _ret = { Foo::new() };
                        cb.construct("constructor", _ret?)
                    })
                })
                .unwrap_or_else(|e| {
                    napi::bindgen_prelude::JsError::from(e).throw_into(env);
                    std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                })
        }
    }
    #[cfg(all(not(test), not(feature = "noop"), not(target_arch = "wasm32")))]
    extern fn __napi_register__Foo_impl_1() {
        napi::__private::register_class(
            "Foo",
            None,
            "Foo\0",
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    napi::bindgen_prelude::Property::new("constructor")
                        .unwrap()
                        .with_property_attributes(
                            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32)
                                .unwrap(),
                        )
                        .with_ctor(__napi__new),
                ]),
            ),
        );
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __napi_register__Foo_impl_1___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
        unsafe extern "C" fn __napi_register__Foo_impl_1___rust_ctor___ctor() -> usize {
            __napi_register__Foo_impl_1();
            0
        }
        __napi_register__Foo_impl_1___rust_ctor___ctor
    };
}
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    }
}
