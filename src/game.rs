
use bevy::prelude::*;   // Bevy의 주요 기능들을 사용하기 위해 prelude를 가져옵니다.

use crate::alien;       // 외계인 관련 모듈을 가져옵니다.
use crate::resolution;  // 해상도 관련 모듈을 가져옵니다.
use crate::player;      // 플레이어 관련 모듈을 가져옵니다.
use crate::projectile;  // 투사체(발사체) 관련 모듈을 가져옵니다.

// GamePlugin 구조체를 정의합니다. 이 플러그인은 게임의 주요 시스템과 모듈들을 추가하는 역할을 합니다.
pub struct GamePlugin;

// GamePlugin에 대해 Plugin 트레이트를 구현합니다.
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(   // 여러 커스텀 플러그인들을 애플리케이션에 추가합니다.
                (
                    alien::AlienPlugin,             // 외계인 관련 기능을 처리하는 플러그인을 추가합니다.
                    resolution::ResolutionPlugin,   // 해상도 관련 설정을 담당하는 플러그인을 추가합니다.
                    player::PlayerPlugin,           // 플레이어 관련 로직을 담당하는 플러그인을 추가합니다.
                    projectile::ProjectilePlugin,   // 투사체(발사체) 관련 로직을 처리하는 플러그인을 추가합니다.
                )
            )
            .add_systems(Startup, setup_scene); // 게임 시작 시 실행할 시스템으로 `setup_scene`을 등록합니다.
    }
}

/// 게임 장면(scene)을 설정하는 함수입니다.
/// 주로 카메라를 초기화하는 역할을 합니다.
fn setup_scene(mut commands: Commands) {
    // 2D 카메라를 생성하여 장면에 스폰합니다.
    commands.spawn(Camera2dBundle { ..default() });
}
