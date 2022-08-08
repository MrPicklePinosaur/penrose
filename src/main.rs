#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::MouseEvent, config::Config, helpers::index_selectors, manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More,
};

fn main() -> penrose::Result<()> {
    let config = Config::default();
    let hooks = vec![];

    let key_bindings = gen_keybindings! {
        "A-j" => run_internal!(cycle_client, Forward);
        "A-k" => run_internal!(cycle_client, Backward);
        "A-S-j" => run_internal!(drag_client, Forward);
        "A-S-k" => run_internal!(drag_client, Backward);
        "A-S-c" => run_internal!(kill_client);
        "A-Tab" => run_internal!(toggle_workspace);
        // "M-bracketright" => run_internal!(cycle_screen, Forward);
        // "M-bracketleft" => run_internal!(cycle_screen, Backward);
        // "M-S-bracketright" => run_internal!(drag_workspace, Forward);
        // "M-S-bracketleft" => run_internal!(drag_workspace, Backward);
        // "M-grave" => run_internal!(cycle_layout, Forward);
        // "M-S-grave" => run_internal!(cycle_layout, Backward);
        // "M-A-Up" => run_internal!(update_max_main, More);
        // "M-A-Down" => run_internal!(update_max_main, Less);
        // "M-A-Right" => run_internal!(update_main_ratio, More);
        // "M-A-Left" => run_internal!(update_main_ratio, Less);
        "A-S-q" => run_internal!(exit);
        "A-Return" => run_external!("st");

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "A-{}" => focus_workspace (REF);
            "A-S-{}" => client_to_workspace (REF);
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})?;

    Ok(())
}
