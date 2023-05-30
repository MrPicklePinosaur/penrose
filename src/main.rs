#[macro_use]
extern crate penrose;

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

use std::collections::HashMap;

use anyhow::Result;
use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReflectHorizontal},
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::hooks::{add_ewmh_hooks, WindowSwallowing},
    map,
    x::query::{ClassName, Title},
    x11rb::RustConn,
};
use pino_xrdb::*;
// use tracing_subscriber::{self, prelude::*};

fn raw_keybindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "A-j" => modify_with(|cs| cs.focus_down()),
        "A-k" => modify_with(|cs| cs.focus_up()),
        "A-S-j" => modify_with(|cs| cs.swap_down()),
        "A-S-k" => modify_with(|cs| cs.swap_up()),
        "A-S-c" => modify_with(|cs| cs.kill_focused()),
        "A-Tab" => modify_with(|cs| cs.toggle_tag()),
        "A-S-h" => modify_with(|cs| cs.next_layout()),
        "A-S-l" => modify_with(|cs| cs.previous_layout()),
        "A-i" => send_layout_message(|| IncMain(1)),
        "A-d" => send_layout_message(|| IncMain(-1)),
        "A-l" => send_layout_message(|| ExpandMain),
        "A-h" => send_layout_message(|| ShrinkMain),
        "A-p" => spawn("dmenu_run"),
        "A-S-Return" => spawn("st"),
        "A-S-q" => exit(),

        // screen stuff
        "M-period" => modify_with(|cs| cs.next_screen()),
        "M-comma" => modify_with(|cs| cs.previous_screen()),
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (format!("A-{tag}"), modify_with(move |cs| cs.focus_tag(tag))),
            (
                format!("A-S-{tag}"),
                modify_with(move |cs| cs.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn layouts() -> LayoutStack {
    let max_main = 1;
    let ratio = 0.6;
    let ratio_step = 0.1;
    let outer_px = 5;
    let inner_px = 5;

    stack!(
        MainAndStack::side(max_main, ratio, ratio_step),
        Monocle::boxed(),
        ReflectHorizontal::wrap(MainAndStack::side(max_main, ratio, ratio_step)),
        MainAndStack::bottom(max_main, ratio, ratio_step)
    )
    .map(|layout| Gaps::wrap(layout, outer_px, inner_px))
}

fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt()
    //     .with_env_filter("info")
    //     .finish()
    //     .init();

    let conn = RustConn::new()?;
    let keybindings = parse_keybindings_with_xmodmap(raw_keybindings())?;

    // config from xresources
    let xresources = load_xresources()?;
    let config = Config {
        normal_border: xresources.unfocused_border.try_into().unwrap(),
        focused_border: xresources.focused_border.try_into().unwrap(),
        border_width: xresources.border_px,
        default_layouts: layouts(),
        event_hook: Some(WindowSwallowing::boxed(ClassName("st"))),
        ..Default::default()
    };

    let wm = WindowManager::new(add_ewmh_hooks(config), keybindings, HashMap::new(), conn)?;

    wm.run()?;
    Ok(())
}
