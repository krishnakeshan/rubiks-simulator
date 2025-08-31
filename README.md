# Play!
*It's recommended to read the section on cube rotation just below this section, first.*

You can use the simulator at https://krishnakeshan.github.io/rubiks-simulator

The wasm binary is currently pretty chunky at 17MB so the first load may take some time. Subsequent visits should load the wasm from your disk cache so those should be faster.

# Rotation
Beside the cube are cube controls. These may behave differently than you might expect them to at first but below is the intuition for the controls.

This simulator uses Bevy’s right-handed 3D coordinates and applies moves very literally:
when you click a face, the 9 cubies on that face are rotated by ±90° around that face’s outward normal (an invisible arrow sticking straight out of the face) — nothing more, nothing less.

## The rule

Think right-hand rule. Point your right thumb along the face’s outward normal (from the cube center through the face).

+90° = curl of your right fingers.

-90° = opposite direction.

This is true no matter where the camera is. If you’re looking at the back face from the front, +90° may look reversed. This is expected.

These rules may seem non-sensical (and perhaps they are) but they were used to make the implementation as simple as possible while plugging in to what the engine offers. The more intuitive rotation model of each arrow doing exactly what it looks like would require a lot of hard coding, that I don't like. However, contributions are always welcome so if you'd like to 'fix' this, feel free to send a PR.
