# Bevy-Snake

A basic snake game built using the Bevy game engine for learning purposes.

## Notes

Each segment is an entity with the following components:
- `Transform` - position and rotation
- `GlobalTransform` - global position and rotation
- `Material` - color
- `Mesh` - shape
- `Visibility` - whether the segment is visible
- `SnakeSegment` - index of the segment within the `SnakeState` resource used to update the positions of the segments

Rather than using parent-child relationships between the segment entities, I am using a global `SnakeState` resource
that stores the coordinate of each segment in a `LinkedList`. This is because it is much easier to move the snake
and propagate the segment positions by pushing and popping from the front and back of the list, than it is to propagate
updates to children, and the bevy built in transform propagation system does not work well with this. The segment entities
have a `SnakeSegment` component which contains the index of the segment within the linked list, and a `render_system` is
used to update the transforms of the segment entities based on the grid cell index stored in the `SnakeState` resource.