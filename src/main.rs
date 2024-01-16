//! Demonstrates how CSS Grid layout can be used to lay items out in a 2D grid
use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

#[derive(Component)]
struct Viewport;

#[derive(Component)]
struct ViewportCamera;

use bevy::render::camera;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Editor".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, spawn_layout)
        .add_systems(Update, update_camera)
        .run();
}

fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let font = asset_server.load("fonts/Inter-Regular.ttf");
    let window_background = Color::hex("39393e").unwrap();
    let panel_background = Color::hex("232326").unwrap();
    let viewport_background = Color::hex("2b2c2f").unwrap();
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10., -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

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
                                    padding: UiRect::all(Val::Px(6.0)),
                                    flex_direction: FlexDirection::Column,
                                    position_type: PositionType::Relative,
                                    grid_row: GridPlacement::span(2),
                                    margin: UiRect::top(Val::Px(26.4)),
                                    ..default()
                                },
                                background_color: BackgroundColor(panel_background),
                                ..default()
                            },
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

                                // viewport content, fills up everything with margin 6px, nothing in it tho its just a background color
                                
                                        builder
                                        .spawn((NodeBundle {
                                            style: Style {
                                                height: Val::Percent(100.0),
                                                width: Val::Percent(100.0),
                                                display: Display::Flex,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::FlexStart,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(viewport_background),
                                            ..default()
                                        }, Viewport));
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
                                "Bevy Editor Super Super Basic Prototype - it isnt really usable yet but it has a viewport and stuff",
                            );
                        });
                });
        });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                // Renders the right camera after the left camera, which has a default priority of 0
                order: 1,
                viewport: Some(camera::Viewport {
                    physical_position: UVec2::new(50, 50),
                    physical_size: UVec2::new(300, 300),
                    ..default()
                }),
                ..default()
            },
            camera_3d: Camera3d {
                // don't clear on the second camera because the first camera already cleared the window
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        UiCameraConfig { show_ui: false },
        ViewportCamera,
    ));

    // cube
    // circular base
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    },PickableBundle::default(),));
    // cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },PickableBundle::default(),));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn update_camera(
    viewport: Query<(&Node, &GlobalTransform, With<Viewport>)>,
    ui_scale: Res<UiScale>,
    mut camera: Query<(&mut Camera, &mut Transform, &GlobalTransform, With<ViewportCamera>)>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut motion_evr: EventReader<MouseMotion>,
        mut gizmos: Gizmos,
) {
    // we want to get the the
    let single = viewport.single();
    let physical_rect = single.0.physical_rect(single.1, 1.0, ui_scale.0);
    let mut camera = camera.single_mut();
    if physical_rect.width() > 0.0 && physical_rect.height() > 0.0 {
        camera.0.viewport = Some(camera::Viewport {
            physical_position: UVec2::new(
                physical_rect.min.x as u32,
                physical_rect.min.y as u32,
            ),
            // if height and width are both >0
            physical_size: UVec2::new(
                physical_rect.width() as u32,
                physical_rect.height() as u32,
            ),
            ..default()
        });
    }

    // camera movement, standard fly cam with WASD and QE and mouse, only does anything while right mouse button is held, we also lock mouse while the real
    let mut primary_window = q_windows.single_mut();

    /*
    let Some(cursor_position) = primary_window.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    if let Some(ray) = camera.0.viewport_to_world(camera.2, cursor_position - Vec2::new(physical_rect.min.x, physical_rect.min.y)) {
        if let Some(distance) = ray.intersect_plane(Vec3::ZERO, Vec3::Y) {
        let point = ray.get_point(distance);
        // Draw a circle just above the ground plane at that position.
    gizmos.circle(point + Vec3::Y * 0.01, Vec3::Y, 0.2, Color::WHITE);
    }
    }*/

    if buttons.pressed(MouseButton::Right) {
        primary_window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
        primary_window.cursor.visible = false;

        let mut delta = Vec3::ZERO;
        let forward = -camera.1.local_z();
        let right = camera.1.local_x();
        let up = camera.1.local_y();

        let sensitivity = 0.00012;

        if keyboard_input.pressed(KeyCode::W) {
            delta += forward;
        }
        if keyboard_input.pressed(KeyCode::S) {
            delta -= forward;
        }
        if keyboard_input.pressed(KeyCode::A) {
            delta -= right;
        }
        if keyboard_input.pressed(KeyCode::D) {
            delta += right;
        }
        if keyboard_input.pressed(KeyCode::Q) {
            delta -= up;
        }
        if keyboard_input.pressed(KeyCode::E) {
            delta += up;
        }

        // KeyCode::ShiftLeft = 2x speed, KeyCode::AltLeft = 0.5x speed. you can use both at once, they cancel out automatically because of math
        let speed =
            1.0 * if keyboard_input.pressed(KeyCode::ShiftLeft) {
                5.0
            } else {
                1.0
            } * if keyboard_input.pressed(KeyCode::AltLeft) {
                0.2
            } else {
                1.0
            };

        let mut transform = camera.1;
        transform.translation += delta * 0.1 * speed;

        for ev in motion_evr.iter() {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

            // Using smallest of height or width ensures equal vertical and horizontal sensitivity
            let window_scale = primary_window.height().min(primary_window.width());
            pitch -= (sensitivity * ev.delta.y * window_scale).to_radians();
            yaw -= (sensitivity * ev.delta.x * window_scale).to_radians();

            pitch = pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }

        camera.1 = transform;
    } else {
        primary_window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
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
