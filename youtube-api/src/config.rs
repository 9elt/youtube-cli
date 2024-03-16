struct Config {
    modified: bool,
    gl: &'static str,
    hl: &'static str,
}

static mut CONFIG: Config = Config {
    gl: "US",
    hl: "en",
    modified: false,
};

pub fn read_config() -> (&'static str, &'static str) {
    unsafe { (CONFIG.gl, CONFIG.hl) }
}

pub fn set_config(gl: &str, hl: &str) {
    if gl.len() != 2 {
        panic!("invalid gl '{gl}', provide a valid country code");
    }

    if hl.len() != 2 {
        panic!("invalid hl '{hl}', provide a valid language code");
    }

    unsafe {
        if CONFIG.modified {
            panic!("config already set, cannot set it again");
        }

        CONFIG.gl = gl.to_uppercase().leak();
        CONFIG.hl = hl.to_lowercase().leak();
        CONFIG.modified = true;
    }
}
