# Inventory implementation example

After a [reddit feedback](http://www.reddit.com/r/rust/comments/2srrx0/another_entity_component_system/), I decided to try and implement a simple inventory prototype with Sparkle.

## Expected Output

````
======= Inventories: =======
joe owns:
  - food
  - crowbar
bob owns:
  - boots
  - hat
============================
* bob steals joe's crowbar *
======= Inventories: =======
joe owns:
  - food
bob owns:
  - crowbar
  - boots
  - hat
============================
````
