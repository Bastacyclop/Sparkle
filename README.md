Sparkle
=======

Sparkle is another Entity Component System (ECS) written in Rust. It has been highly inspirated
by already existing ECS: entityx, artemis framework and anax.

## Overview

*Currently Sparkle provides the following features:*

- Automatic implementation of Component trait.
- Components are stored in a VecMap and accessible by indexes.
- Helpers to implements system with their own entities queue
- A blackboard and a command queue for communication
- Support of entities tags and groups

## Installation

To install Sparkle put these lines in your Cargo.toml file:

```toml
  [dependencies.sparkle]
  
  git = "https://github.com/RustSparkle/Sparkle/"

  [dependencies.sparkle_macros]
  
  git = "https://github.com/RustSparkle/Sparkle/"
```

and add these in your main rust file:

```rust
  #![feature(plugin)]
  
  #[plugin]  #[no_link] #[macro_use]
  extern crate sparkle_macros;
  extern crate sparkle;
```

## Examples

### Declare a Component

Declaring a new component is easy as following:

```rust
  #[SparkleComponent]
  struct Position {
      pub x: i32,
      pub y: i32
  }
```

### Declare a System

A system is responsible for updating components in the world. Most of the time
you will the default implementation provided by the macro
sparkle_default_system_filtering!:

```rust
  struct PositionPrinter {
      filter: Filter,
      entities: HashSet<Entity>
  }
  
  impl PositionPrinter {
      pub fn new() -> PositionPrinter {
          let filter = sparkle_filter!(require components: Position);
          PositionPrinter {
              filter: filter,
              entities: HashSet::new()
          }
      }
  }
  
  impl System for PositionPrinter {
      fn fixed_update(_em: &mut EntityMapper, cm: &mut ComponentMapper) {
          // This line safely retrieve component stores.
          // Note that you can also use get_store directly but this one can fail if the
          // store doesn't exist.
          let (position_store,) = sparkle_get_stores!(cm, Position);
          
          for entity in self.entities() {
              let position = position_store.get(entity).unwrap();
              println!("Position: {}, {}", position.x, position.y);
          }
      }
      
      fn update(_em: &mut EntityMapper, _cm: &mut ComponentMapper, _dt: f32) {
          // Use this if you want an update every frame.
      }
      
      // This implement methods that fill self.entities according to the filter
      sparkle_default_system_filtering!()
  }
```

### Use the Space struct

A space struct represent a part of your game world. In small project
one instance should be sufficient but in larger one you may need to
isolate groups of entities and create multiple space

```rust
  use sparkle::prelude::*;
  
  let blackboard = SharedBlackboard::new();
  let space = Space::new(blackboard);
  
  let entity = space.em.create_entity();
  space.em.set_tag(entity, "aTag");
  
  // a MetaEntity contains the extras information associated with an entity.
  let meta_entity = space.em.get_mentity_mut(entity);
  space.cm.insert(meta_entity, Position { x: 5, y: 8 });
  
  space.cm.get::<Position>(entity).x = 8;
  ...
  
  space.sm.insert(|_cmd_sender, _blackboard| PositionPrinter::new());
  
  ...
  space.update(dt) // This should be called every frame
  space.fixed_update() // And this in a fixed timestep
```

### Other example

For more in deep example you can see [snaked](https://github.com/RustSparkle/Snaked)

## Notes

Sparkle is in a very early stage and we would appreciate feedback and contributions

## Documentation

Coming...
