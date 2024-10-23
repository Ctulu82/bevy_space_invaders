
use bevy::prelude::*;  // Bevy의 기본 기능을 사용하기 위해 prelude를 가져옵니다.

use crate::resolution;  // 해상도 모듈을 가져옵니다.

// 외계인 관련 로직을 처리하는 AlienPlugin 플러그인을 정의합니다.
pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    // 플러그인이 애플리케이션에 추가될 때 실행되는 빌드 함수입니다.
    fn build(&self, app: &mut App) {
        app
            // 게임이 시작될 때 외계인을 설정하는 시스템을 추가합니다.
            .add_systems(Startup, setup_aliens)
            // 게임이 진행되는 동안 외계인 로직을 처리하는 시스템을 추가합니다.
            .add_systems(Update, (update_aliens, manage_alien_logic));
    }
}

// 외계인 컴포넌트를 정의합니다.
#[derive(Component)]
pub struct Alien {
    pub dead: bool,  // 외계인의 생존 여부를 나타냅니다.
    pub original_position: Vec3,  // 외계인의 원래 위치를 저장합니다.
}

// 죽은 외계인을 표시하는 마커 컴포넌트입니다. 죽은 외계인은 쿼리에서 제외되기 위해 사용됩니다.
#[derive(Component)]
pub struct Dead;

// 외계인의 동작을 관리하는 AlienManager 리소스를 정의합니다.
#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,  // 외계인이 이동하는 방향입니다.
    pub shift_aliens_down: bool,  // 외계인을 아래로 이동시킬지 여부를 나타냅니다.
    pub dist_from_boundary: f32,  // 외계인이 화면 경계에서 얼마나 떨어져 있는지를 나타냅니다.
    pub reset: bool,  // 게임이 재설정되어야 하는지 여부를 나타냅니다.
}

// 외계인을 가로, 세로로 얼마나 배치할지 설정하는 상수입니다.
const WIDTH: i32 = 10;  // 가로 10마리
const HEIGHT: i32 = 5;  // 세로 5마리
const SPACING: f32 = 24.;  // 외계인 간의 간격
const SPEED: f32 = 100.0;  // 외계인이 움직이는 속도
const ALIEN_SHIFT_AMOUNT: f32 = 32.;  // 외계인이 아래로 이동하는 거리

// 외계인을 생성하는 함수입니다.
fn setup_aliens(
    mut commands: Commands,  // 명령 객체로 엔티티 생성 및 리소스 삽입이 가능합니다.
    asset_server: Res<AssetServer>,  // 애셋 서버를 통해 텍스처를 불러옵니다.
    resolution: Res<resolution::Resolution>,  // 해상도 리소스를 불러옵니다.
) {
    // AlienManager 리소스를 초기화하여 게임 상태를 관리합니다.
    commands.insert_resource(AlienManager {
        reset: false,
        dist_from_boundary: 0.,
        shift_aliens_down: false,
        direction: 1.,  // 외계인 이동 방향 (처음엔 오른쪽)
    });

    // 외계인 텍스처를 로드합니다. 기본 루트 디렉토리는 `assets`입니다.
    let alien_texture = asset_server.load("alien.png");

    // 가로 `WIDTH`, 세로 `HEIGHT`만큼 외계인을 배치합니다.
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)  // X축 중앙에 맞추기 위해 이동
                - (Vec3::Y * HEIGHT as f32 * SPACING * 1.0)  // Y축 아래로 이동
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5);  // 화면의 상단으로 이동
            
            // 외계인 엔티티를 생성하고 SpriteBundle과 Alien 컴포넌트를 부여합니다.
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),  // 스프라이트 크기를 해상도 비율에 맞게 설정
                    texture: alien_texture.clone(),
                    ..default()
                },
                Alien {
                    original_position: position,  // 외계인의 원래 위치 저장
                    dead: false,  // 생성된 외계인은 아직 살아있음
                }
            ));
        }
    }
}

// 외계인의 위치와 상태를 업데이트하는 함수입니다.
fn update_aliens(
    mut commands: Commands,
    // 아직 살아 있는 외계인만 쿼리합니다.
    mut alien_query: Query<(Entity, &Alien, &mut Transform, &mut Visibility), Without<Dead>>,
    mut alien_manager: ResMut<AlienManager>,  // 외계인 관리 리소스
    resolution: Res<resolution::Resolution>,  // 해상도 리소스
    time: Res<Time>,  // 시간 리소스
) {
    for (entity, alien, mut transform, mut visibility) in alien_query.iter_mut() {
        // delta_seconds를 사용하여 프레임률에 관계없이 일정한 속도로 외계인이 이동하도록 설정
        transform.translation.x += time.delta_seconds() * alien_manager.direction * SPEED;

        // 외계인이 화면 경계를 넘어가면 아래로 이동
        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            alien_manager.shift_aliens_down = true;
            alien_manager.dist_from_boundary = resolution.screen_dimensions.x * alien_manager.direction * 0.5 - transform.translation.x;
        }

        // 외계인이 죽었을 경우 처리
        if alien.dead {
            commands.entity(entity).insert(Dead{});  // Dead 컴포넌트를 삽입하여 외계인을 죽은 상태로 표시
            *visibility = Visibility::Hidden;  // 외계인을 화면에서 숨김
        } else {
            *visibility = Visibility::Visible;  // 살아 있는 외계인은 계속 표시
        }

        // 외계인이 화면 아래로 내려가면 게임이 리셋됩니다.
        if transform.translation.y < -resolution.screen_dimensions.y * 0.5 {
            alien_manager.reset = true;
        }
    }
}

// 외계인의 로직을 관리하는 함수입니다.
fn manage_alien_logic(
    mut commands: Commands,
    mut alien_query: Query<(Entity, &mut Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,  // 외계인 관리 리소스
) {
    // 외계인을 아래로 이동시키고 방향을 반대로 바꾸는 처리
    if alien_manager.shift_aliens_down {
        alien_manager.shift_aliens_down = false;  // 아래로 이동 완료 후 플래그를 리셋
        alien_manager.direction *= -1.;  // 이동 방향을 반대로 전환

        for (_entity, _alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;  // 외계인을 화면 안으로 이동
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;  // 외계인을 아래로 이동
        }
    }

    // 게임을 리셋해야 할 때 처리
    if alien_manager.reset {
        alien_manager.reset = false;  // 리셋 완료 후 플래그를 리셋
        alien_manager.direction = 1.;  // 이동 방향을 초기화

        for (entity, mut alien, mut transform) in alien_query.iter_mut() {
            transform.translation = alien.original_position;  // 외계인의 위치를 원래 위치로 되돌림

            if alien.dead {
                // 죽은 외계인을 부활시킵니다.
                alien.dead = false;
                commands.entity(entity).remove::<Dead>();  // Dead 컴포넌트를 제거하여 외계인을 다시 살아있는 상태로 만듦
            }
        }
    }
}