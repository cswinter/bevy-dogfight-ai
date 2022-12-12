use bevy::prelude::*;
use stylist::yew::styled_component;
use stylist::{css, global_style};
use yew::prelude::*;

use bevy_dogfight_ai::LAUNCHER_TITLE;

fn set_window_title(title: &str) {
    web_sys::window()
        .and_then(|w| w.document())
        .expect("Unable to get DOM")
        .set_title(title);
}

fn set_global_css() {
    global_style! {
        r#"
        html {
            min-height: 100%;
            position: relative;
        }
        body {
            height: 100%;
            padding: 0;
            margin: 0;
        }
        "#
    }
    .expect("Unable to mount global style");
}

#[styled_component(Root)]
fn view() -> Html {
    set_window_title(LAUNCHER_TITLE);
    set_global_css();

    let css = css!(
        r#"
        position: absolute;
        overflow: hidden;
        width: 100%;
        height: 100%;
        "#
    );

    html! {
        <div class={ css }>
            <canvas id="bevy"></canvas>
        </div>
    }
}

fn main() {
    // Mount the DOM
    yew::start_app::<Root>();
    // Start the Bevy App
    let settings = bevy_dogfight_ai::Settings {
        players: 2,
        ai_action_interval: Some(12),
        continuous_collision_detection: true,
        opponent_policy: Some("versus-relpos-obsfix-128m".to_string()),
        ..Default::default()
    };
    let mut app = bevy_dogfight_ai::app(settings, vec![]);
    info!("Starting launcher: WASM");
    app.run();
}