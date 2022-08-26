#[macro_use]
extern crate penrose;

use anyhow::Result;
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
use pino_xrdb::*;

struct ColorConfig {
    focused_border: String,
    unfocused_border: String,
    border_px: u32,
    gap_px: u32,
    show_bar: bool,
    top_bar: bool,
    bar_height: u32,
}

const PROG_NAME: &str = "penrose";
fn load_xresources() -> Result<ColorConfig> {
    let xrdb = Xrdb::new()?;

    Ok(ColorConfig {
        focused_border: xrdb
            .query(PROG_NAME, "focused_border")
            .unwrap_or(String::from("#FFFFFF")),
        unfocused_border: xrdb
            .query(PROG_NAME, "unfocused_border")
            .unwrap_or(String::from("#000000")),
        border_px: xrdb
            .query(PROG_NAME, "border_px")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(2),
        gap_px: xrdb
            .query(PROG_NAME, "gap_px")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(5),
        show_bar: xrdb
            .query(PROG_NAME, "show_bar")
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(true),
        top_bar: xrdb
            .query(PROG_NAME, "top_bar")
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(true),
        bar_height: xrdb
            .query(PROG_NAME, "bar_height")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(5),
    })
}

fn main() -> Result<()> {
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

    // config from xresources
    let config = load_xresources()?;
    config_builder
        .focused_border(config.focused_border)?
        .unfocused_border(config.unfocused_border)?
        .border_px(config.border_px)
        .gap_px(config.gap_px)
        .show_bar(config.show_bar)
        .top_bar(config.top_bar)
        .bar_height(config.bar_height);

    let config = config_builder.build().unwrap();
    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})?;

    Ok(())
}
