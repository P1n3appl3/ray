# Ray

A toy unbiased path tracer made by following the excellent tutorials: "Ray Tracing in One Weekend" and "Ray Tracing: the Next Week".

## Examples

![Cornell Box](https://i.imgur.com/nRxOnPy.png)
![Ray Tracing In One Weekend](https://i.imgur.com/oCkkcKU.png)
![Ray Tracing: The Next Week](https://i.imgur.com/Vgl8uKd.png)

If I do more renders I'll put them [here](https://imgur.com/a/CIZrqYd).

## Features

- Geometric primatives: sphere, rectangle, and rectancular prism
- 3D Mesh support (using .obj files)
- Materials: diffuse, specular, dielectric, isotropic, and emissive
- Textures: constant, image based (including hdr), and procedural (checkered/perlin noise/gradient)
- Acceleration using bounded volume heirarchy and [parallelization](https://github.com/rayon-rs/rayon)
- Global illumination using radiant textured world sphere and emmisive entities
- Camera with depth of field

See [the todo file](todo.md) for stuff that I'm considering implementing in the future.

## Non-Features

These are techniques or features I've given up on implementing due to difficulty or lack of interest.

- Motion blur
- Animation
- Importance sampling
- GPU acceleration
- Networked/distributed rendering
- Tone mapping
- Non-code scene description

## Resources

- [Real Time Rendering](http://www.realtimerendering.com) hosts a bunch of free ray tracing books including the "in One Weekend" series. They also link to ["Physically Based Rendering"](http://www.pbr-book.org/3ed-2018/contents.html) which is like the bible of raytracing (haven't read it myself but everyone recommends it) and [Abrash's Black Book](http://www.jagregory.com/abrash-black-book/) which has nothing to do with raytracing but is full of really cool optimizations for graphics code.
- [Peter Shirley's blog](http://in1weekend.blogspot.com) is a great companion to his books. It goes more in depth on the topics covered as well as showing off more modern ray tracing techniques.
- [Scratchapixel](https://www.scratchapixel.com) is my favorite comprehensive resource for computer graphics basics. It covers many techniques and algorithms used in raytracing with code samples and visualizations.
- In terms of inspiration, I've always loved minimalist graphics programs like [the business card raytracer](http://fabiensanglard.net/rayTracing_back_of_business_card/), [smallpt](http://www.kevinbeason.com/smallpt), and [donut.c](https://www.a1k0n.net/2011/07/20/donut-math.html). I also watch a youtube series called [Two Minute Papers](https://www.youtube.com/channel/UCbfYPyITQ-7l4upoX8nvctg) which gives brief explanations of academic papers mostly in the fields of computer graphics and machine learning. The guy who runs it also happens to teach a [college level raytracing course](https://www.cg.tuwien.ac.at/courses/Rendering/VU.SS2017.html) which I quite enjoyed the lectures from.
- I've mostly stayed away from reading academic papers or following real college level graphics courses, but there are a few that I've skimmed and gotten something out of. These inlcude [UT Austin's graphics course](https://www.cs.utexas.edu/users/fussell/courses/cs384g-fall2011) which had a good lesson on adaptive super sampling, an [MIT OCW graphics course](https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-837-computer-graphics-fall-2012), and the [University of Tartu's online resources](https://cglearn.codelight.eu/pub/computer-graphics/environment-mapping#material-sphere-map-1).
- I've looked at dozens of blog posts and repos from others building their own ray tracers varrying in complexity from just as "toy" as mine to large scale projects. They've been a great source of inspiration, as well as being good mini-lessons for raytracing problems like [generating random points in a sphere](https://karthikkaranth.me/blog/generating-random-points-in-a-sphere). In no particular order here are some that I read while working on mine:
  - [Will Usher](https://www.willusher.io/projects) made a bunch of awesome graphics projects. In particular I really like [tray_rust](https://github.com/Twinklebear/tray_rust)
  - [Seena Burns](http://seenaburns.com/benchmarking-rust-with-cargo-bench/) followed Ri1W and the next week in rust
  - [Arshia Mufti](https://github.com/arshiamufti/tracy) followed Ri1W in rust
  - [Brook Heisler](https://bheisler.github.io/post/writing-gpu-accelerated-path-tracer-part-1/) wrote a GPU accelerated path tracer in rust
  - [bitshifter](https://bitshifter.github.io/2018/04/29/rust-ray-tracer-in-one-weekend/) followed Ri1W in rust and went on to optimize the heck out of it
  - [Aras Pranckevičius](http://aras-p.info/blog/2018/03/28/Daily-Pathtracer-Part-0-Intro/) wrote a few raytracers using different languages and technologies (including WASM!) and they have a bunch of posts on general graphics stuff
  - [demofox](https://blog.demofox.org/2016/09/21/path-tracing-getting-started-with-diffuse-and-emissive/) has written dozens of informative posts on ray tracing techniques
  - [Kevin Beason](http://www.kevinbeason.com/worklog/) wrote a fairly advanced raytracer and has great doccumentation of their progress
- The [MERL BDRF database](https://www.merl.com/brdf/), [HDRI Haven](https://hdrihaven.com/), and [texturify](http://texturify.com/) are great free resources for texturing and environment mapping.
- [This](https://all3dp.com/3d-file-format-3d-files-3d-printer-3d-cad-vrml-stl-obj/) is a great comparison of popular 3d file formats.
- Adam Perry wrote [a great walkthrough](https://blog.anp.lol/rust/2016/07/24/profiling-rust-perf-flamegraph/) for performance profiling rust with flamegraphs.
