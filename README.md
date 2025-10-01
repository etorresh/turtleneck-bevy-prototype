# Turlteneck

Prototype game built with Bevy + Avian Physics. Custom player controller, time-stop mechanics.

![PauseTimeClip(1)](https://github.com/user-attachments/assets/687934ed-2e9b-43a9-b79f-377e59a9e88a)



# Notes
Animation
    - What are the different methods? I was worried there would be 10 competing ways to animate a 3D model but it seems the standard is "Hierarchical Finite State Machine with Weighted Blending" and Bevy uses nodes to implement it.
    - Progress: Sprites -> Hard-cut state machines -> Linear crossfading -> hierarchical state machines with weighted blending -> Node-based/compositional systems