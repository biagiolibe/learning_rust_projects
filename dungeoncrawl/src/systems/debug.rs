use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn debug(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
) {
    let mut player_query = <&Point>::query().filter(component::<Player>());
    let player_position = player_query.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    let mut y_position = 5;
    let offset = Point::new(camera.left_x, camera.top_y);
    draw_batch.target(2);
    draw_batch.print_color(Point::new(0, increment_and_get(&mut y_position)), "Developer info", ColorPair::new(RED, WHITE));
    draw_batch.print_color(Point::new(0, increment_and_get(&mut y_position)),
                                 format!("Camera bounds: left {}, right {}, top {}, bottom {}",
                                         camera.left_x,
                                         camera.right_x,
                                         camera.top_y,
                                         camera.bottom_y),
                                 ColorPair::new(RED, WHITE));
    draw_batch.print_color(Point::new(0, increment_and_get(&mut y_position)),
                                 format!("Mouse position: {:?} (on screen: {:?}", (*mouse_pos/4)+offset, *mouse_pos),
                                 ColorPair::new(RED, WHITE));
    draw_batch.print_color(Point::new(0, increment_and_get(&mut y_position)),
                                 format!("Player position: {:?}", *player_position),
                                 ColorPair::new(RED, WHITE));
    draw_batch.submit(10000).expect("HUD rendering error")
}

fn increment_and_get(y :&mut i32) -> i32 {
    *y+=1;
    *y
}
