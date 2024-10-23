
use bevy::prelude::*;   // Bevy의 주요 기능들을 사용하기 위해 prelude를 가져옵니다.

use crate::resolution;  // 해상도 관련 모듈을 가져옵니다.
use crate::alien;       // 외계인 관련 모듈을 가져옵니다.

// 투사체(발사체) 관련 로직을 처리하는 ProjectilePlugin 플러그인을 정의합니다.
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    // 이 플러그인이 애플리케이션에 추가될 때 실행되는 빌드 함수입니다.
    fn build(&self, app: &mut App) {
        // 게임 업데이트 중에 실행될 시스템으로 투사체 이동 및 외계인과의 상호작용을 추가합니다.
        app.add_systems(Update, (update_projectiles, update_alien_interactions));
    }
}

// 투사체를 나타내는 컴포넌트를 정의합니다.
#[derive(Component)]
pub struct Projectile {
    pub speed: f32,  // 투사체의 이동 속도를 나타냅니다.
}

/// 투사체를 이동시키는 함수입니다.
fn update_projectiles(
    mut commands: Commands,                                             // 명령을 통해 엔티티를 조작할 수 있는 구조체입니다.
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>, // 투사체 엔티티를 쿼리하여 속성과 위치 정보를 가져옵니다.
    time: Res<Time>,                                                    // 시간 리소스를 사용하여 프레임별 시간 차이를 계산합니다.
    resolution: Res<resolution::Resolution>                             // 해상도 정보를 가져와 화면 크기를 참고합니다.
) {
    for(entity, projectile, mut transform) in projectile_query.iter_mut() {
        // 투사체의 Y 좌표를 속도와 시간에 따라 업데이트합니다.
        transform.translation.y += projectile.speed * time.delta_seconds();
        
        // 투사체가 화면을 벗어나면 해당 엔티티를 제거합니다.
        if transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();  // 엔티티를 삭제(소멸)합니다.
        }
    }
}

const BULLET_RADIUS: f32 = 24.;  // 투사체와 외계인의 충돌 판정을 위한 반경 값을 정의합니다.

/// 투사체와 외계인의 상호작용을 처리하는 함수입니다.
fn update_alien_interactions(
    mut alien_query: Query<(&mut alien::Alien, &Transform), Without<alien::Dead>>,  // 살아 있는 외계인을 쿼리합니다.
    mut projectile_query: Query<(Entity, &Transform), With<Projectile>>,            // 투사체 엔티티를 쿼리합니다.
    mut commands: Commands                                                          // 엔티티를 조작하기 위한 명령 구조체입니다.
) {
    for(mut alien, alien_transform) in alien_query.iter_mut() {
        for(projectile_entity, projectile_transform) in projectile_query.iter_mut() {
            // 투사체의 현재 위치를 2D 벡터로 변환합니다.
            let projectile_pos = Vec2::new(
                projectile_transform.translation.x,
                projectile_transform.translation.y,
            );

            // 외계인의 현재 위치를 2D 벡터로 변환합니다.
            let alien_pos = Vec2::new(
                alien_transform.translation.x,
                alien_transform.translation.y
            );

            // 투사체와 외계인의 거리가 BULLET_RADIUS 내에 있으면 충돌로 간주합니다.
            if Vec2::distance(alien_pos, projectile_pos) < BULLET_RADIUS {
                alien.dead = true;  // 외계인을 죽은 상태로 변경합니다.
                // 쿼리 내에서 바로 엔티티를 삭제하는 것은 권장되지 않지만, 게임에는 큰 영향을 미치지 않습니다.
                commands.entity(projectile_entity).despawn();  // 투사체 엔티티를 소멸시킵니다.
            }
        }
    }
}