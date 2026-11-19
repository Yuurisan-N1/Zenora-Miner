fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Zenora Miner");
        res.set("FileDescription", "Zenora Miner");
        if let Err(err) = res.compile() {
            let msg = format!("cargo:warning=the Windows icon could not be attached: {err}");
            println!("{msg}");
        }
    }
}
