## Complex Web Graph

A tool to visualize and modify functions that act in both the real and complex planes.

TODO:

- Add a is_complex function to ast
- Function customization
- Dynamic/Playable sliders
- Mouse hints/point data/text to screen
- Optimize parser

---

<details>
      <summary>Old (OUTDATED) README.md</summary>

A non-exhaustive checklist on what is needed to complete this project:

- [x] Boilerplate JS and WASM interfacing.
- [x] Research and impl. structs to handle tokens, preferablly easily expandible
- [x] Scanner for lexemes in evaluation.
- [x] Evaluator to to create tokens to use.
- [x] Impliment Recursive Descent Parser or the shunting yard algorithim to
      build and AST from tokens.
- [x] Global variables and variable assignment, with multiple assignment
- [ ] Function handling, arbitrary amount of arguments
- [x] Complex number input, _i_ operator and basic complex functions via vectors
- [ ] Complex trig operators, with the mclauren series used as an approximation
      around a specificly defined point.
- [x] Wasm & Js Rendering using HTML5 canvas, currently thinking rendering with Js
- [x] Drawing an infinitely resizable and moveble grid, containerized for
      multiple instances in the future
- [x] Calculating an output 2d vector array from an input 2d vector array using
      AST (or other method of function representation) given a range and
      percision determined by the grid's view window in WASM.<sup>1</sup>
- [x] Rendering a series of points and line from the 2d vector array by only
      graphing the reals
- [x] Being able to select the coordinate space of the output for different views
- [ ] Multiple views that move simultaneously of the output data
      (calculate once, render n-times)
- [x] Better input, n-many input boxes

Optional:

- [ ] KaTeX rendering of input/evaluated expression for better looking results
- [ ] 3D grid rendering for more types of complex ouput visualization.
- [ ] Lua/Pyton based scripting api for changing things

---

Note: I am writing this project alongside learning Rust, so in the event that
I learn any new paridigms or design patterns that I want/need to impl. I will
add them here:

- [x] Debug "mode" to only compile console_log!'s when told.
- [x] Canvas modification via wasm
- [x] canvas compositing and instantiation for each function.

---

[1] This means that
when using functions based on the mclauren series, the center of the graph
is the a point, and all calculations of surrounding points are passed
through that in order to cut down on making a new approximating function
for each entry in the array.

</details>
