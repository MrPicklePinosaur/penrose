#[macro_use]
extern crate penrose;

mod config;
mod layout;

use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use config::WMConfig;
use log::*;
use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReflectHorizontal, ReserveTop},
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
    x::{
        query::{ClassName, Title},
        XConn,
    },
    x11rb::RustConn,
    Color,
};
use penrose_ui::{
    bar::{
        widgets::{
            amixer_volume, battery_summary, current_date_and_time, wifi_network, ActiveWindowName,
            CurrentLayout, Workspaces,
        },
        Position, StatusBar,
    },
    core::TextStyle,
};
use serde::{Deserialize, Serialize};

const BAR_HEIGHT: u32 = 8;
const FONT: &'static str = "Sauce Code Pro Nerd Font";

const PROG_NAME: &str = "penrose";
// use tracing_subscriber::{self, prelude::*};

fn raw_keybindings(config: &WMConfig) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
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

// Mostly the example dwm bar from the main repo but recreated here so it's easier to tinker
// with and add in debug widgets when needed.
pub fn status_bar<X: XConn>(config: &WMConfig) -> penrose_ui::Result<StatusBar<X>> {
    let highlight: Color = config.colors.blue;
    let empty_ws: Color = config.colors.dark_white;

    let style = TextStyle {
        font: FONT.to_string(),
        point_size: 8,
        fg: config.colors.white,
        bg: Some(config.colors.black),
        padding: (2.0, 2.0),
    };

    let padded_style = TextStyle {
        padding: (4.0, 2.0),
        ..style.clone()
    };

    const MAX_ACTIVE_WINDOW_CHARS: usize = 50;

    StatusBar::try_new(
        Position::Top,
        config.status_bar.height,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        &[&style.font],
        vec![
            Box::new(Workspaces::new(&style, highlight, empty_ws)),
            Box::new(CurrentLayout::new(&style)),
            // Box::new(penrose_bar::widgets::debug::StateSummary::new(style)),
            Box::new(ActiveWindowName::new(
                MAX_ACTIVE_WINDOW_CHARS,
                &TextStyle {
                    bg: Some(highlight),
                    padding: (6.0, 4.0),
                    ..style.clone()
                },
                true,
                false,
            )),
            Box::new(wifi_network(&padded_style)),
            Box::new(battery_summary("BAT0", &padded_style)),
            // Box::new(amixer_volume("Master", &padded_style)),
            Box::new(current_date_and_time(&padded_style)),
        ],
    )
}

fn layouts(config: &WMConfig) -> LayoutStack {
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
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, outer_px, inner_px), BAR_HEIGHT))
}

fn load_config() -> WMConfig {
    let config_file = match fs::read_to_string(PathBuf::from("config.ron")) {
        Ok(config_file) => config_file,
        Err(e) => {
            warn!("Could not open config file");
            return WMConfig::default();
        },
    };
    let config = match ron::from_str::<WMConfig>(&config_file) {
        Ok(config) => config,
        Err(e) => {
            warn!("Error parsing config file");
            return WMConfig::default();
        },
    };
    config
}

fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt()
    //     .with_env_filter("info")
    //     .finish()
    //     .init();

    let config = load_config();

    let conn = RustConn::new()?;
    let mykeybindings = parse_keybindings_with_xmodmap(raw_keybindings(&config))?;

    let myconfig = Config {
        // normal_border:
        // focused_border:
        // border_width:
        default_layouts: layouts(&config),
        event_hook: Some(WindowSwallowing::boxed(ClassName("st"))),
        ..Default::default()
    };

    let wm = WindowManager::new(
        add_ewmh_hooks(myconfig),
        mykeybindings,
        HashMap::new(),
        conn,
    )?;
    let bar = status_bar(&config)?;
    let wm = bar.add_to(wm);

    wm.run()?;
    Ok(())
}
