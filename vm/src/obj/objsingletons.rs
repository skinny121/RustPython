use super::objtype::PyClassRef;
use crate::pyobject::{
    IntoPyObject, PyClassImpl, PyContext, PyObjectRef, PyRef, PyValue, TypeProtocol,
};
use crate::vm::VirtualMachine;

#[pyclass(module = false, name = "NoneType")]
#[derive(Debug)]
pub struct PyNone;
pub type PyNoneRef = PyRef<PyNone>;

impl PyValue for PyNone {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.none.class()
    }
}

// This allows a built-in function to not return a value, mapping to
// Python's behavior of returning `None` in this situation.
impl IntoPyObject for () {
    fn into_pyobject(self, vm: &VirtualMachine) -> PyObjectRef {
        vm.ctx.none()
    }
}

impl<T: IntoPyObject> IntoPyObject for Option<T> {
    fn into_pyobject(self, vm: &VirtualMachine) -> PyObjectRef {
        match self {
            Some(x) => x.into_pyobject(vm),
            None => vm.ctx.none(),
        }
    }
}

#[pyimpl]
impl PyNone {
    #[pyslot]
    fn tp_new(_: PyClassRef, vm: &VirtualMachine) -> PyNoneRef {
        vm.ctx.none.clone()
    }

    #[pymethod(magic)]
    fn repr(&self) -> String {
        "None".to_owned()
    }

    #[pymethod(magic)]
    fn bool(&self) -> bool {
        false
    }
}

#[pyclass(module = false, name = "NotImplementedType")]
#[derive(Debug)]
pub struct PyNotImplemented;
pub type PyNotImplementedRef = PyRef<PyNotImplemented>;

impl PyValue for PyNotImplemented {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.not_implemented.class()
    }
}

#[pyimpl]
impl PyNotImplemented {
    #[pymethod(magic)]
    fn repr(&self) -> String {
        "NotImplemented".to_owned()
    }
}

pub fn init(context: &PyContext) {
    PyNone::extend_class(context, &context.none.class());
    PyNotImplemented::extend_class(context, &context.not_implemented.class());
}
