use crate::prelude::*;

#[system]
#[write_component(FieldOfView)]
#[read_component(Point)]
pub fn field_of_view(
    ecs: &mut SubWorld,
    #[resource] map: &mut Map,
) {
    let mut field_of_views = <(&Point,&mut FieldOfView)>::query();
    field_of_views.iter_mut(ecs)
        .filter(|(_pos, fov)| fov.is_dirty)
        .for_each(|(pos, fov)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
            fov.is_dirty = false;
        });
}