use super::hstring::Hstring;
use super::interfaces::IInspectable;
use super::winapi::ro_activate_instance;
use super::HRESULT;

pub struct Inspectable(IInspectable);

impl Inspectable {
    pub fn activate(class_id: &Hstring) -> Result<Self, HRESULT> {
        Ok(Self(ro_activate_instance(class_id)?))
    }

    /// SAFETY: You are not allowed to release the returned pointer.
    pub unsafe fn as_ptr(&self) -> IInspectable {
        self.0
    }
}

impl Drop for Inspectable {
    fn drop(&mut self) {
        // SAFETY: An `Inspectable` is only ever created with a valid `IInspectable`.
        unsafe { ((**self.0).Release)(self.0.cast()) };
    }
}
