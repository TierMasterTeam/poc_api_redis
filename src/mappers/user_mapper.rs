use crate::dto::user_dto::UserDto;
use crate::entities::user_entity::UserEntity;

pub fn dto_to_entity(dto: UserDto) -> UserEntity {
    UserEntity {
        id: dto.id,
        email: dto.email,
        name: dto.name,
    }
}

pub fn entity_to_dto(entity: UserEntity) -> UserDto {
    UserDto {
        id: entity.id,
        email: entity.email,
        name: entity.name,
    }
}