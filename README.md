# Introduction

This repository contains the source code for a physics engine I am developing, which is going to be submitted as my A-Level EPQ artefact.

The physics engine attempts to simulate basic mechanics, including: forces; force integration; collision detection and resolution; friction; and so forth.
My language of choice for this project is Rust, and graphics library used is SDL2, by means of the [sdl2-rs](https://docs.rs/sdl2/latest/sdl2/#) crate. 

# Photos
<div align="center">
  
  ### Simulation Demonstration
  <div flex="row">
      <img src="https://github.com/Tymur-Tykva/sdl2-rs-physics/raw/master/images/demo-1.png" width="45%" align="center"></img>
      <img src="https://github.com/Tymur-Tykva/sdl2-rs-physics/raw/master/images/demo-2.png" width="45%" align="center"></img>
  </div>

  ### Collision Detection Visualization
  <img src="https://github.com/Tymur-Tykva/sdl2-rs-physics/raw/master/images/collision-detection-1.png" width="75%" align="center"></img>
  <p>Collision detection test; broad-phase collision detection flag, potential intersect found.</p>
  <br/>

  <img src="https://github.com/Tymur-Tykva/sdl2-rs-physics/raw/master/images/collision-detection-2.png" width="75%" align="center"></img>
  <p>Collision detection test; narrow phase collision detection flag, single considered intersection, and single intersect found.</p>
  <br/>

  <img src="https://github.com/Tymur-Tykva/sdl2-rs-physics/raw/master/images/collision-detection-3.png" width="75%" align="center"></img>
  <p>Collision detection test; narrow phase collision detection flag, multiple considered intersections, and single intersect found.</p>
</div>
