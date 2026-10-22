pub mod Windows {
    pub mod Globalization {
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Calendar(windows_core::IUnknown);
        windows_core::imp::interface_hierarchy!(
            Calendar,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl Calendar {
            pub fn new() -> windows_core::Result<Self> {
                Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
            }
            fn IActivationFactory<
                R,
                F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
            >(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<
                    Calendar,
                    windows_core::imp::IGenericFactory,
                > = windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
            pub fn GetTimeZone(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetTimeZone)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            fn ICalendarFactory<R, F: FnOnce(&ICalendarFactory) -> windows_core::Result<R>>(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Calendar, ICalendarFactory> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
            fn ICalendarFactory2<R, F: FnOnce(&ICalendarFactory2) -> windows_core::Result<R>>(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Calendar, ICalendarFactory2> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeType for Calendar {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_class::<Self, ICalendar>();
        }
        unsafe impl windows_core::Interface for Calendar {
            type Vtable = <ICalendar as windows_core::Interface>::Vtable;
            const IID: windows_core::GUID = <ICalendar as windows_core::Interface>::IID;
        }
        impl windows_core::RuntimeName for Calendar {
            const NAME: &'static str = "Windows.Globalization.Calendar";
        }
        unsafe impl Send for Calendar {}
        unsafe impl Sync for Calendar {}
        windows_core::imp::define_interface!(
            ICalendar,
            ICalendar_Vtbl,
            0xca30221d_86d9_40fb_a26b_d44eb7cf08ea
        );
        impl windows_core::RuntimeType for ICalendar {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Globalization.ICalendar");
        }
        #[repr(C)]
        pub struct ICalendar_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
        }
        windows_core::imp::define_interface!(
            ICalendarFactory,
            ICalendarFactory_Vtbl,
            0x83f58412_e56b_4c75_a66e_0f63d57758a6
        );
        impl windows_core::RuntimeType for ICalendarFactory {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Globalization.ICalendarFactory",
            );
        }
        #[repr(C)]
        pub struct ICalendarFactory_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
        }
        windows_core::imp::define_interface!(
            ICalendarFactory2,
            ICalendarFactory2_Vtbl,
            0xb44b378c_ca7e_4590_9e72_ea2bec1a5115
        );
        impl windows_core::RuntimeType for ICalendarFactory2 {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Globalization.ICalendarFactory2",
            );
        }
        #[repr(C)]
        pub struct ICalendarFactory2_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
        }
        windows_core::imp::define_interface!(
            ITimeZoneOnCalendar,
            ITimeZoneOnCalendar_Vtbl,
            0xbb3c25e5_46cf_4317_a3f5_02621ad54478
        );
        impl windows_core::RuntimeType for ITimeZoneOnCalendar {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Globalization.ITimeZoneOnCalendar",
            );
        }
        #[repr(C)]
        pub struct ITimeZoneOnCalendar_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub GetTimeZone: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
    }
}
