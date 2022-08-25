#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::MouseEvent,
        config::Config,
        helpers::index_selectors,
        layout::{side_stack, Layout, LayoutConf},
        manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More,
};

fn main() -> penrose::Result<()> {
    let hooks = vec![];

    let key_bindings = gen_keybindings! {
        // basics
        "A-j" => run_internal!(cycle_client, Forward);
        "A-k" => run_internal!(cycle_client, Backward);
        "A-S-j" => run_internal!(drag_client, Forward);
        "A-S-k" => run_internal!(drag_client, Backward);
        "A-S-c" => run_internal!(kill_client);
        "A-H" => run_internal!(update_main_ratio, Less);
        "A-L" => run_internal!(update_main_ratio, More);
        "A-P" => run_internal!(cycle_layout, Forward);
        "A-I" => run_internal!(update_max_main, More);
        "A-D" => run_internal!(update_max_main, Less);
        "A-S-q" => run_internal!(exit);
        "A-S-Return" => run_external!("st");

        // screens
        "M-period" => run_internal!(cycle_screen, Forward);
        "M-comma" => run_internal!(cycle_screen, Backward);
        "M-S-period" => run_internal!(drag_workspace, Forward);
        "M-S-comma" => run_internal!(drag_workspace, Backward);

        // workspace
        "A-Tab" => run_internal!(toggle_workspace);
        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "A-{}" => focus_workspace (REF);
            "A-S-{}" => client_to_workspace (REF);
        };
    };

    let mut config_builder = Config::default().builder();
    config_builder.workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]);

    config_builder.layouts(vec![
        Layout::new("[side]", LayoutConf::default(), side_stack, 1, 0.5),
        Layout::floating("[-]"),
    ]);

    // colors
    config_builder
        .focused_border("#cc241d")?
        .unfocused_border("#3c3836")?;

    let config = config_builder.build().unwrap();
    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})?;

    Ok(())
}
