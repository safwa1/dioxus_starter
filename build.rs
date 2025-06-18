#[cfg(windows)]
fn get_favicon_path() -> &'static str {
	"assets/favicon.ico"
}

#[cfg(windows)]
fn set_windows_metadata() {
	let mut res = winresource::WindowsResource::new();
	res.set_icon(get_favicon_path());
	/* 
	/// uncomment if use want run app as administrator
	if !cfg!(debug_assertions) {
		res.set_manifest(r#"
			<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
			<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
				<security>
					<requestedPrivileges>
						<requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
					</requestedPrivileges>
				</security>
			</trustInfo>
			</assembly>
		"#);
	}
	 */
	res.compile().unwrap();
}

fn main() {
	#[cfg(windows)]
	set_windows_metadata();
}