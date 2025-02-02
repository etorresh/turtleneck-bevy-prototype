compile optimization:
- Bevy is dynamically linked (go back to staticly on production)
- using clang llvm linker
- generic sharing with nightly




Camera is Z-up because it makes 2.5D top-down easier.

I want the player controller to be able to interact with dynamic rigidbodies.
- Use physics based movement (it feeels unresponsive and slidy)
- Move Player position directly, and check for collisions, and limit the movement to right before the collision, thje problem with this is that it doesn't allow the player to interact with dynamic rigidbodies since it will always stop right before colliding with them, could I fix this by having the player go slightly into the dynamic bodies? Micro-Penetration for Dynamic Objects. My real life event-loop also runs that system every FixedUpdate.


Physics GameLayers:
- 1 << 0 Default
- 1 << 1 Rigidbodies (not the player)
 