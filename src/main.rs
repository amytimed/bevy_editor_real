//! Demonstrates how CSS Grid layout can be used to lay items out in a 2D grid
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

#[derive(Component)]
struct Viewport;

use bevy::render::camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Editor Mockup".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_layout)
        .run();
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Inter-Regular.ttf");
    let window_background = Color::hex("39393e").unwrap();
    let panel_background = Color::hex("232326").unwrap();
    commands.spawn(Camera2dBundle::default());

    // Top-level flex (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                // Use the CSS Flex algorithm for laying out this node
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                // Make node fill the entirety it's parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(6.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            background_color: BackgroundColor(window_background),
            ..default()
        })
        .with_children(|builder| {
            // App
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        // 324 px 1fr 324px
                        grid_template_columns: vec![
                            GridTrack::px(324.0),
                            GridTrack::fr(1.0),
                            GridTrack::px(324.0),
                        ],
                        // 42px 1fr 240px 240px
                        grid_template_rows: vec![
                            GridTrack::px(42.0),
                            GridTrack::fr(1.0),
                            GridTrack::px(240.0),
                            GridTrack::px(240.0),
                        ],
                        row_gap: Val::Px(6.0),
                        column_gap: Val::Px(6.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    // header panel, empty
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                padding: UiRect {
                                    left: Val::Px(9.0),
                                    right: Val::Px(9.0),
                                    top: Val::Px(6.0),
                                    bottom: Val::Px(6.0),
                                },
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                position_type: PositionType::Relative,
                                // span 3
                                grid_column: GridPlacement::span(3),
                                ..default()
                            },
                            background_color: BackgroundColor(panel_background),
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(30.0),
                                        height: Val::Px(30.0),
                                        margin: UiRect::right(Val::Px(9.6)),
                                        // no object-fit: contain :(
                                        ..default()
                                    },
                                    // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                },
                                UiImage::new(asset_server.load("icon.png")),
                            ));
                            // File Edit View Window Help, all inline-nlock with margin-left/right 6px and opacity 0.5
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::FlexStart,
                                        row_gap: Val::Px(12.0),
                                        column_gap: Val::Px(12.0),
                                        margin: UiRect::left(Val::Px(6.0)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    spawn_nested_text_bundle(builder, font.clone(), "File");
                                    spawn_nested_text_bundle(builder, font.clone(), "Edit");
                                    spawn_nested_text_bundle(builder, font.clone(), "View");
                                    spawn_nested_text_bundle(builder, font.clone(), "Window");
                                    spawn_nested_text_bundle(builder, font.clone(), "Help");
                                });
                        });
                    // hierarchy, left upper
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                padding: UiRect::all(Val::Px(6.0)),
                                flex_direction: FlexDirection::Column,
                                position_type: PositionType::Relative,
                                margin: UiRect::top(Val::Px(26.4)),
                                ..default()
                            },
                            background_color: BackgroundColor(panel_background),
                            ..default()
                        })
                        .with_children(|builder| {
                            // tab list
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(-26.4),
                                        left: Val::Px(0.0),
                                        height: Val::Px(30.),
                                        display: Display::Flex,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::FlexStart,
                                        row_gap: Val::Px(6.0),
                                        column_gap: Val::Px(6.0),
                                        padding: UiRect {
                                            left: Val::Px(6.0),
                                            right: Val::Px(6.0),
                                            top: Val::Px(0.0),
                                            bottom: Val::Px(0.0),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    // tab
                                    builder
                                        .spawn(NodeBundle {
                                            style: Style {
                                                padding: UiRect {
                                                    left: Val::Px(9.6),
                                                    right: Val::Px(9.6),
                                                    top: Val::Px(0.0),
                                                    bottom: Val::Px(2.4),
                                                },
                                                height: Val::Percent(100.0),
                                                display: Display::Flex,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::FlexStart,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(panel_background),
                                            ..default()
                                        })
                                        .with_children(|builder| {
                                            spawn_nested_text_bundle(
                                                builder,
                                                font.clone(),
                                                "Hierarchy  ×",
                                            );
                                        });
                                });
                        });
                    // viewport, center
                    builder
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    padding: UiRect::all(Val::Px(12.0)),
                                    flex_direction: FlexDirection::Column,
                                    position_type: PositionType::Relative,
                                    grid_row: GridPlacement::span(2),
                                    margin: UiRect::top(Val::Px(26.4)),
                                    ..default()
                                },
                                background_color: BackgroundColor(panel_background),
                                ..default()
                            },
                            Viewport,
                        ))
                        .with_children(|builder| {
                            // tab list
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(-26.4),
                                        left: Val::Px(0.0),
                                        height: Val::Px(30.),
                                        display: Display::Flex,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::FlexStart,
                                        row_gap: Val::Px(6.0),
                                        column_gap: Val::Px(6.0),
                                        padding: UiRect {
                                            left: Val::Px(6.0),
                                            right: Val::Px(6.0),
                                            top: Val::Px(0.0),
                                            bottom: Val::Px(0.0),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    // tab
                                    builder
                                        .spawn(NodeBundle {
                                            style: Style {
                                                padding: UiRect {
                                                    left: Val::Px(9.6),
                                                    right: Val::Px(9.6),
                                                    top: Val::Px(0.0),
                                                    bottom: Val::Px(2.4),
                                                },
                                                height: Val::Percent(100.0),
                                                display: Display::Flex,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::FlexStart,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(panel_background),
                                            ..default()
                                        })
                                        .with_children(|builder| {
                                            spawn_nested_text_bundle(
                                                builder,
                                                font.clone(),
                                                "Viewport  ×",
                                            );
                                        });
                                });
                        });
                    // inspector, right
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                padding: UiRect::all(Val::Px(12.0)),
                                flex_direction: FlexDirection::Column,
                                position_type: PositionType::Relative,
                                grid_row: GridPlacement::span(2),
                                margin: UiRect::top(Val::Px(26.4)),
                                ..default()
                            },
                            background_color: BackgroundColor(panel_background),
                            ..default()
                        })
                        .with_children(|builder| {
                            // tab list
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(-26.4),
                                        left: Val::Px(0.0),
                                        height: Val::Px(30.),
                                        display: Display::Flex,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::FlexStart,
                                        row_gap: Val::Px(6.0),
                                        column_gap: Val::Px(6.0),
                                        padding: UiRect {
                                            left: Val::Px(6.0),
                                            right: Val::Px(6.0),
                                            top: Val::Px(0.0),
                                            bottom: Val::Px(0.0),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    // tab
                                    builder
                                        .spawn(NodeBundle {
                                            style: Style {
                                                padding: UiRect {
                                                    left: Val::Px(9.6),
                                                    right: Val::Px(9.6),
                                                    top: Val::Px(0.0),
                                                    bottom: Val::Px(2.4),
                                                },
                                                height: Val::Percent(100.0),
                                                display: Display::Flex,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::FlexStart,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(panel_background),
                                            ..default()
                                        })
                                        .with_children(|builder| {
                                            spawn_nested_text_bundle(
                                                builder,
                                                font.clone(),
                                                "Inspector  ×",
                                            );
                                        });
                                });
                        });
                    // features, left lower
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                padding: UiRect::all(Val::Px(6.0)),
                                flex_direction: FlexDirection::Column,
                                position_type: PositionType::Relative,
                                margin: UiRect::top(Val::Px(26.4)),
                                ..default()
                            },
                            background_color: BackgroundColor(panel_background),
                            ..default()
                        })
                        .with_children(|builder| {
                            // tab list
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(-26.4),
                                        left: Val::Px(0.0),
                                        height: Val::Px(30.),
                                        display: Display::Flex,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::FlexStart,
                                        row_gap: Val::Px(6.0),
                                        column_gap: Val::Px(6.0),
                                        padding: UiRect {
                                            left: Val::Px(6.0),
                                            right: Val::Px(6.0),
                                            top: Val::Px(0.0),
                                            bottom: Val::Px(0.0),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    // tab
                                    builder
                                        .spawn(NodeBundle {
                                            style: Style {
                                                padding: UiRect {
                                                    left: Val::Px(9.6),
                                                    right: Val::Px(9.6),
                                                    top: Val::Px(0.0),
                                                    bottom: Val::Px(2.4),
                                                },
                                                height: Val::Percent(100.0),
                                                display: Display::Flex,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::FlexStart,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(panel_background),
                                            ..default()
                                        })
                                        .with_children(|builder| {
                                            spawn_nested_text_bundle(
                                                builder,
                                                font.clone(),
                                                "Features  ×",
                                            );
                                        });
                                });
                        });
                    // asset browser, bottom
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                padding: UiRect::all(Val::Px(12.0)),
                                flex_direction: FlexDirection::Column,
                                position_type: PositionType::Relative,
                                grid_column: GridPlacement::span(3),
                                margin: UiRect::top(Val::Px(26.4)),
                                ..default()
                            },
                            background_color: BackgroundColor(panel_background),
                            ..default()
                        })
                        .with_children(|builder| {
                            // tab list
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(-26.4),
                                        left: Val::Px(0.0),
                                        height: Val::Px(30.),
                                        display: Display::Flex,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::FlexStart,
                                        row_gap: Val::Px(6.0),
                                        column_gap: Val::Px(6.0),
                                        padding: UiRect {
                                            left: Val::Px(6.0),
                                            right: Val::Px(6.0),
                                            top: Val::Px(0.0),
                                            bottom: Val::Px(0.0),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    // tab
                                    builder
                                        .spawn(NodeBundle {
                                            style: Style {
                                                padding: UiRect {
                                                    left: Val::Px(9.6),
                                                    right: Val::Px(9.6),
                                                    top: Val::Px(0.0),
                                                    bottom: Val::Px(2.4),
                                                },
                                                height: Val::Percent(100.0),
                                                display: Display::Flex,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::FlexStart,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(panel_background),
                                            ..default()
                                        })
                                        .with_children(|builder| {
                                            spawn_nested_text_bundle(
                                                builder,
                                                font.clone(),
                                                "Assets  ×",
                                            );
                                        });
                                });
                        });
                });

            // Footer / status bar
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        width: Val::Percent(100.0),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    // left element is just a display:flex align-center flex-start with gap of 6px
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                row_gap: Val::Px(6.0),
                                column_gap: Val::Px(6.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|builder| {
                            spawn_nested_text_bundle(
                                builder,
                                font.clone(),
                                "Bevy Editor Mockup (not real)",
                            );
                        });
                });
        });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10., -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        camera: Camera {
            // renders after / on top of the main camera
            order: 1,
            viewport: Some(camera::Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(2, 2),
                ..default()
            }),
            ..default()
        },
        ..default()
    });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 14.3, // web mockup had 12px, for some reason bevy font size doesnt match web, this lines it up
            color: Color::WHITE,
        },
    ));
}
