use std::time::Duration;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use std::fmt::Write;
use bevy::math::*;


const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Component)]
pub struct ScreenUI;

#[derive(Component)]
pub struct FPSText;

pub struct ScreenUIPlugin;

pub struct TimerUpdateUI {
    pub timer: Timer,
}


impl Plugin for ScreenUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(create_ui)
            .add_system(update)
            .init_resource::<TimerUpdateUI>();
        ;
    }
}


fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    }).insert(ScreenUI)
        .with_children(|commands| {
            commands.spawn_bundle(TextBundle {
                text: Text::from_section("",
                                         TextStyle {
                                             font: asset_server.load("fonts/Roboto-Medium.ttf"),
                                             font_size: 30.0,
                                             color: Color::GREEN,
                                         }),
                style: Style {
                    size: Size {
                        width: Val::Percent(10.0),
                        height: Val::Percent(10.0),
                    },
                    align_self: AlignSelf::FlexStart,
                    margin: UiRect {
                        left: Val::Percent(1.0),
                        right: Val::Percent(1.0),
                        top: Val::Percent(1.0),
                        bottom: Val::Percent(1.0),
                    },
                    ..default()
                },
                ..default()
            }).insert(FPSText);
        });
}

impl Default for TimerUpdateUI {
    fn default() -> Self {
        Self {
            timer: Timer::new(UPDATE_INTERVAL, true)
        }
    }
}

fn update(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    ressource_timer_update_ui: Option<ResMut<TimerUpdateUI>>,
    mut query: Query<&mut Text, With<FPSText>>,
) {
    if let Some(mut timer_update_ui) = ressource_timer_update_ui {
        if timer_update_ui.timer.tick(time.delta()).just_finished() {
            for mut text in &mut query {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(fps_format) = fps.average() {
                        text.sections[0].value = format!("{fps_format:.2}");
                    }
                }
            }
        }
    }
}



