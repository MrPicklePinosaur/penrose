
use penrose::core::config::Config;

fn main() -> penrose::Result<()> {

    let config = Config::default();
    let key_bindings = gen_keybindings!{

    }

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})

}
