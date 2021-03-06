// This code was autogenerated with `dbus-codegen-rust -d dev.pustaczek.Vaxtify -p / -m None`, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait DevPustaczekVaxtify {
	fn browser_register(&self, pid: u32) -> Result<(), dbus::Error>;
	fn browser_tab_delete(&self, pid: u32, tab: i32) -> Result<(), dbus::Error>;
	fn browser_tab_update(&self, pid: u32, tab: i32, url: &str) -> Result<(), dbus::Error>;
	fn browser_unregister(&self, pid: u32) -> Result<(), dbus::Error>;
	fn permit_end(&self, permit: &str) -> Result<(), dbus::Error>;
	fn permit_start(&self, permit: &str) -> Result<(), dbus::Error>;
	fn permit_start_with_duration(&self, permit: &str, duration: u64) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> DevPustaczekVaxtify for blocking::Proxy<'a, C> {
	fn browser_register(&self, pid: u32) -> Result<(), dbus::Error> {
		self.method_call("dev.pustaczek.Vaxtify", "BrowserRegister", (pid,))
	}

	fn browser_tab_delete(&self, pid: u32, tab: i32) -> Result<(), dbus::Error> {
		self.method_call("dev.pustaczek.Vaxtify", "BrowserTabDelete", (pid, tab))
	}

	fn browser_tab_update(&self, pid: u32, tab: i32, url: &str) -> Result<(), dbus::Error> {
		self.method_call("dev.pustaczek.Vaxtify", "BrowserTabUpdate", (pid, tab, url))
	}

	fn browser_unregister(&self, pid: u32) -> Result<(), dbus::Error> {
		self.method_call("dev.pustaczek.Vaxtify", "BrowserUnregister", (pid,))
	}

	fn permit_end(&self, permit: &str) -> Result<(), dbus::Error> {
		self.method_call("dev.pustaczek.Vaxtify", "PermitEnd", (permit,))
	}

	fn permit_start(&self, permit: &str) -> Result<(), dbus::Error> {
		self.method_call("dev.pustaczek.Vaxtify", "PermitStart", (permit,))
	}

	fn permit_start_with_duration(&self, permit: &str, duration: u64) -> Result<(), dbus::Error> {
		self.method_call("dev.pustaczek.Vaxtify", "PermitStartWithDuration", (permit, duration))
	}
}

#[derive(Debug)]
pub struct DevPustaczekVaxtifyTabClose {
	pub pid: u32,
	pub tab: i32,
}

impl arg::AppendAll for DevPustaczekVaxtifyTabClose {
	fn append(&self, i: &mut arg::IterAppend) {
		arg::RefArg::append(&self.pid, i);
		arg::RefArg::append(&self.tab, i);
	}
}

impl arg::ReadAll for DevPustaczekVaxtifyTabClose {
	fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
		Ok(DevPustaczekVaxtifyTabClose { pid: i.read()?, tab: i.read()? })
	}
}

impl dbus::message::SignalArgs for DevPustaczekVaxtifyTabClose {
	const NAME: &'static str = "TabClose";
	const INTERFACE: &'static str = "dev.pustaczek.Vaxtify";
}

#[derive(Debug)]
pub struct DevPustaczekVaxtifyTabCreateEmpty {
	pub pid: u32,
}

impl arg::AppendAll for DevPustaczekVaxtifyTabCreateEmpty {
	fn append(&self, i: &mut arg::IterAppend) {
		arg::RefArg::append(&self.pid, i);
	}
}

impl arg::ReadAll for DevPustaczekVaxtifyTabCreateEmpty {
	fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
		Ok(DevPustaczekVaxtifyTabCreateEmpty { pid: i.read()? })
	}
}

impl dbus::message::SignalArgs for DevPustaczekVaxtifyTabCreateEmpty {
	const NAME: &'static str = "TabCreateEmpty";
	const INTERFACE: &'static str = "dev.pustaczek.Vaxtify";
}

pub trait OrgFreedesktopDBusIntrospectable {
	fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopDBusIntrospectable
	for blocking::Proxy<'a, C>
{
	fn introspect(&self) -> Result<String, dbus::Error> {
		self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ()).map(|r: (String,)| r.0)
	}
}
