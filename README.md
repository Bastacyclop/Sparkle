Sparkle
=======

Sparkle is another Entity Component System (ECS) written in Rust. It has been highly inspirated
by already existing ECS: entityx, artemis framework and anax.

## Overview

*Currently, Sparkle provides the following features:*

- Automatic implementation of Component trait
- Helpers to implement systems with their own set of entity
- A blackboard and a command queue for communication
- Support of entity tags and groups

## Installation

To install Sparkle put these lines in your Cargo.toml file:

```toml
  [dependencies.sparkle]
  
  git = "https://github.com/RustSparkle/Sparkle/"

  [dependencies.sparkle_macros]
  
  git = "https://github.com/RustSparkle/Sparkle/"
```

and these in your main file:

```rust
  #![feature(plugin)]
  
  #[plugin] #[no_link] #[macro_use]
  extern crate sparkle_macros;
  extern crate sparkle;
```

## Examples

### Declare a Component

Declaring a new component is as easy as following:

```rust
  #[SparkleComponent]
  struct Position {
      pub x: i32,
      pub y: i32
  }
```

### Declare a System

A system is responsible for updating components in the world. Most of the time you'll either want no filtering or the default implementation provided by the macro `sparkle_default_system_filtering!`:

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
          // This line safely retrieves component stores.
          // Note that you can also use get_store directly but it will panic if the
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
      
      // This macro implements methods that will manage self.entities according to self.filter
      sparkle_default_system_filtering!()
  }
```

### Use a Space

A space represents a part of your game world. In small projects one instance should be sufficient, but in larger ones you may need to isolate groups of entities by creating multiple spaces.

```rust
  use sparkle::prelude::*;
  
  let blackboard = SharedBlackboard::new();
  let space = Space::new(blackboard);
  
  let entity = space.em.create_entity();
  space.em.set_tag(entity, "a_tag");
  
  // a MetaEntity contains the extra information associated to an entity.
  let meta_entity = space.em.get_mentity_mut(entity);
  space.cm.insert(meta_entity, Position { x: 5, y: 8 });
  
  space.cm.get::<Position>(entity).x = 8;
  ...
  
  space.sm.insert(|_cmd_sender, _blackboard| PositionPrinter::new());
  
  ...
  space.update(dt) // This should be called every frame
  space.fixed_update() // And this at a fixed timestep
```

### Other examples

For a more specific example, you can look at the small game [Snaked](https://github.com/RustSparkle/Snaked).

## Notes

Sparkle is in a very early stage and we would appreciate feedback and contributions.

## Documentation

Coming...
