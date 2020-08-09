use crate::obj::objbytearray::{PyByteArray, PyByteArrayRef};
use crate::obj::objbytes::{PyBytes, PyBytesRef};
use crate::obj::objstr::{PyString, PyStringRef};
use crate::pyobject::PyObjectRef;
use crate::pyobject::{PyResult, TryFromObject, TypeProtocol};
use crate::stdlib::array::{PyArray, PyArrayRef};
use crate::vm::VirtualMachine;

#[derive(Debug)]
pub enum PyBytesLike {
    Bytes(PyBytesRef),
    Bytearray(PyByteArrayRef),
    Array(PyArrayRef),
}

impl TryFromObject for PyBytesLike {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        match_class!(match obj {
            b @ PyBytes => Ok(PyBytesLike::Bytes(b)),
            b @ PyByteArray => Ok(PyBytesLike::Bytearray(b)),
            array @ PyArray => Ok(PyBytesLike::Array(array)),
            obj => Err(vm.new_type_error(format!(
                "a bytes-like object is required, not {}",
                obj.class()
            ))),
        })
    }
}

impl PyBytesLike {
    pub fn len(&self) -> usize {
        match self {
            PyBytesLike::Bytes(b) => b.len(),
            PyBytesLike::Bytearray(b) => b.borrow_value().len(),
            PyBytesLike::Array(array) => array.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_cow(&self) -> std::borrow::Cow<[u8]> {
        match self {
            PyBytesLike::Bytes(b) => b.get_value().into(),
            PyBytesLike::Bytearray(b) => b.borrow_value().elements.clone().into(),
            PyBytesLike::Array(array) => array.tobytes().into(),
        }
    }

    #[inline]
    pub fn with_ref<R>(&self, f: impl FnOnce(&[u8]) -> R) -> R {
        match self {
            PyBytesLike::Bytes(b) => f(b.get_value()),
            PyBytesLike::Bytearray(b) => f(&b.borrow_value().elements),
            PyBytesLike::Array(array) => f(&*array.get_bytes()),
        }
    }
}

pub enum PyRwBytesLike {
    Bytearray(PyByteArrayRef),
    Array(PyArrayRef),
}

impl TryFromObject for PyRwBytesLike {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        match_class!(match obj {
            b @ PyByteArray => Ok(PyRwBytesLike::Bytearray(b)),
            array @ PyArray => Ok(PyRwBytesLike::Array(array)),
            obj =>
                Err(vm.new_type_error(format!("a buffer object is required, not {}", obj.class()))),
        })
    }
}

impl PyRwBytesLike {
    pub fn len(&self) -> usize {
        match self {
            PyRwBytesLike::Bytearray(b) => b.borrow_value().len(),
            PyRwBytesLike::Array(array) => array.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn with_ref<R>(&self, f: impl FnOnce(&mut [u8]) -> R) -> R {
        match self {
            PyRwBytesLike::Bytearray(b) => f(&mut b.borrow_value_mut().elements),
            PyRwBytesLike::Array(array) => f(&mut array.get_bytes_mut()),
        }
    }
}

#[derive(Debug)]
pub enum PyAsciiBytesLike {
    String(PyStringRef),
    Buffer(PyBytesLike),
}

impl TryFromObject for PyAsciiBytesLike {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        match obj.downcast::<PyString>() {
            Ok(string) => {
                if string.as_str().is_ascii() {
                    Ok(PyAsciiBytesLike::String(string))
                } else {
                    Err(vm.new_value_error(
                        "string argument should contain only ASCII characters".to_owned(),
                    ))
                }
            }
            Err(obj) => PyBytesLike::try_from_object(vm, obj).map(PyAsciiBytesLike::Buffer),
        }
    }
}

impl PyAsciiBytesLike {
    pub fn len(&self) -> usize {
        match self {
            PyAsciiBytesLike::String(s) => s.as_str().len(),
            PyAsciiBytesLike::Buffer(buffer) => buffer.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn with_ref<R>(&self, f: impl FnOnce(&[u8]) -> R) -> R {
        match self {
            PyAsciiBytesLike::String(s) => f(s.as_str().as_bytes()),
            PyAsciiBytesLike::Buffer(buffer) => buffer.with_ref(f),
        }
    }
}
