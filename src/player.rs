
use bevy::prelude::*;  // Bevy의 기본 모듈을 가져옵니다.

use crate::resolution;  // 화면 해상도 관련 모듈을 가져옵니다.
use crate::projectile;  // 발사체(총알) 관련 모듈을 가져옵니다.

/// 플레이어 플러그인 구조체입니다.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// 플러그인 빌드 함수. `setup_player`로 플레이어를 설정하고 `update_player`로 플레이어 상태를 업데이트합니다.
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)  // 게임이 시작될 때 플레이어 설정
            .add_systems(Update, update_player);  // 매 프레임마다 플레이어 상태 업데이트
    }
}

/// 플레이어 컴포넌트. 
#[derive(Component)]
struct Player {
    // 발사 쿨다운 타이머. 연속으로 너무 빨리 발사하지 않도록 제한합니다.
    pub shoot_timer: f32,
}

/// 플레이어 설정 함수입니다. 게임이 시작될 때 호출되어 플레이어 캐릭터를 화면에 배치합니다.
fn setup_player(
    mut commands: Commands,                     // 명령어 큐
    asset_server: Res<AssetServer>,             // 리소스 서버 (스프라이트 같은 리소스 로드에 사용)
    resolution: Res<resolution::Resolution>,    // 해상도 정보
) {
    // 플레이어 이미지(스프라이트)를 로드합니다.
    let player_image = asset_server.load("player.png");
    
    // 플레이어 스프라이트를 화면의 아래쪽 중앙에 배치하고, 적절한 비율로 크기를 조정합니다.
    commands.spawn((
        SpriteBundle {
            texture: player_image,  // 플레이어 이미지
            transform: Transform::from_xyz(
                0.,         // X좌표는 중앙에 위치
                -(resolution.screen_dimensions.y * 0.5) + (resolution.pixel_ratio * 5.0),   // Y좌표는 화면 하단에 약간 위로 띄워서 배치
                0.,         // Z축 좌표
            ).with_scale(Vec3::splat(resolution.pixel_ratio)),  // 스프라이트 크기 조정
            ..Default::default()
        },
        Player { shoot_timer: 0. },  // 발사 쿨다운 타이머를 0으로 초기화하여 처음부터 발사 가능하도록 설정
    ));
}

// 플레이어 이동 속도 및 발사체 속도 상수 정의
const SPEED: f32 = 200.;            // 플레이어의 이동 속도
const BULLET_SPEED: f32 = 400.;     // 발사체의 이동 속도
const SHOOT_COOLDOWN: f32 = 0.5;    // 총알 발사 쿨다운 시간 (0.5초마다 발사 가능)

/// 매 프레임마다 호출되어 플레이어 상태를 업데이트하는 함수입니다.
fn update_player(
    mut commands: Commands,                                 // 명령어 큐
    asset_server: Res<AssetServer>,                         // 리소스 서버 (발사체 이미지 로드에 사용)
    mut player_query: Query<(&mut Player, &mut Transform)>, // 플레이어의 컴포넌트와 위치(Transform) 정보
    time: Res<Time>,                                        // 시간 정보 (프레임 간격 계산에 사용)
    keys: Res<ButtonInput<KeyCode>>,                        // 키보드 입력 정보
    resolution: Res<resolution::Resolution>,                // 화면 해상도 정보
) {
    // 플레이어 컴포넌트를 가져옵니다.
    let (mut player, mut transform) = player_query.single_mut();

    // 키보드 입력에 따라 수평 이동 값을 설정합니다.
    let mut horizontal = 0.;
    
    if keys.pressed(KeyCode::KeyA) {  // A 키를 누르면 왼쪽으로 이동
        horizontal += -1.;
    }
    
    if keys.pressed(KeyCode::KeyD) {  // D 키를 누르면 오른쪽으로 이동
        horizontal += 1.;
    }

    // 플레이어의 위치를 수평 이동 값에 따라 업데이트합니다.
    transform.translation.x += horizontal * time.delta_seconds() * SPEED;

    // 플레이어가 화면 밖으로 나가지 않도록 경계 처리
    let left_bound = -resolution.screen_dimensions.x * 0.5;  // 왼쪽 화면 경계
    let right_bound = resolution.screen_dimensions.x * 0.5;  // 오른쪽 화면 경계

    if transform.translation.x > right_bound {
        transform.translation.x = right_bound;  // 오른쪽 경계를 넘으면 경계에 고정
    }

    if transform.translation.x < left_bound {
        transform.translation.x = left_bound;  // 왼쪽 경계를 넘으면 경계에 고정
    }

    // 발사 쿨다운 타이머를 감소시킵니다.
    player.shoot_timer -= time.delta_seconds();

    // 스페이스바가 눌려 있고, 발사 쿨다운이 끝났을 때 총알을 발사합니다.
    if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0. {
        player.shoot_timer = SHOOT_COOLDOWN;  // 쿨다운을 다시 설정
        let bullet_texture = asset_server.load("bullet.png");  // 총알 이미지 로드
        
        // 총알 스프라이트를 플레이어의 현재 위치에서 생성하고, 위쪽으로 발사되도록 설정
        commands.spawn((
            SpriteBundle {
                texture: bullet_texture,  // 총알 이미지
                transform: Transform::from_translation(transform.translation).with_scale(Vec3::splat(resolution.pixel_ratio)),  // 총알 위치는 플레이어의 현재 위치에서 발사
                ..Default::default()
            },
            projectile::Projectile {
                speed: BULLET_SPEED,  // 총알의 이동 속도 설정
            },
        ));
    }
}
