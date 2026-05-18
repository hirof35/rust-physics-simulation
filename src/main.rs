use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

// 初期位置を記憶する目印コンポーネント
#[derive(Component)]
struct FallingBox {
    initial_x: f32,
    initial_y: f32,
    initial_z: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, reset_simulation_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ライト
    commands.spawn(PointLightBundle {
        point_light: PointLight { shadows_enabled: true, ..default() },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // カメラ
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 地面
    commands
        .spawn(Collider::cuboid(5.0, 0.1, 5.0))
        .insert(RigidBody::Fixed)
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::new(10.0, 0.2, 10.0)),
            material: materials.add(Color::rgb(0.2, 0.7, 0.2)),
            ..default()
        });

    // 100個の立方体を生成
    let mut rng = rand::thread_rng();
    let mesh_handle = meshes.add(Cuboid::new(0.6, 0.6, 0.6));

    for i in 0..100 {
        let x = rng.gen_range(-2.0..2.0);
        let z = rng.gen_range(-2.0..2.0);
        let y = 5.0 + (i as f32) * 0.8;

        let r = rng.gen_range(0.0..1.0);
        let g = rng.gen_range(0.0..1.0);
        let b = rng.gen_range(0.0..1.0);

        commands
            .spawn(RigidBody::Dynamic)
            .insert(Collider::cuboid(0.3, 0.3, 0.3))
            .insert(Restitution::coefficient(0.7))
            // 物理エンジンのスリープ（計算省略モード）を無効化して、常に同期を強制する設定
            .insert(Sleeping::disabled())
            .insert(FallingBox { initial_x: x, initial_y: y, initial_z: z })
            .insert(PbrBundle {
                mesh: mesh_handle.clone(),
                material: materials.add(Color::rgb(r, g, b)),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            });
    }
}

// やり直し（リセット）を行うシステム
fn reset_simulation_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // クエリに対象のEntity、初期位置、位置コンポーネントを指定
    mut query: Query<(Entity, &FallingBox, &mut Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("---- RESET TRIGGERED (FORCE RE-SPAWN) ----"); 
        
        for (entity, box_info, mut transform) in query.iter_mut() {
            // 1. ★ 一度、物理演算の対象（RigidBodyとVelocity）から完全に除外する
            // これにより、物理エンジン側の古い位置データや速度データが完全に消去されます
            commands.entity(entity).remove::<RigidBody>();
            commands.entity(entity).remove::<Velocity>();

            // 2. 見た目の位置と回転を初期値にワープ
            transform.translation = Vec3::new(box_info.initial_x, box_info.initial_y, box_info.initial_z);
            transform.rotation = Quat::default();
            
            // 3. ★ ワープさせた直後に、もう一度「重力で動く物体（Dynamic）」として再登録する
            // 新しく追加されるため、速度（Velocity）も自動的にゼロから再スタートします
            commands.entity(entity).insert(RigidBody::Dynamic);
        }
    }
}