A non-exhaustive checklist on what is needed to complete this project:

- [x] Boilerplate JS and WASM interfacing.
- [ ] Research and impl. structs to handle tokens, preferablly easily expandible
- [x] Scanner for lexemes in evaluation.
- [ ] Evaluator to to create tokens to use.
- [ ] Impliment Recursive Descent Parser or the shunting yard algorithim to
      build and AST from tokens.
- [ ] Global variables and variable assignment, with multiple assignment
- [ ] Function handling, arbitrary amount of arguments
- [ ] Complex number input, _i_ operator and basic complex functions via vectors
- [ ] Complex trig operators, with the mclauren series used as an approximation
      around a specificly defined point.
- [ ] Wasm & Js Rendering using webgl, currently thinking rendering with Js
- [ ] Drawing an infinitely resizable and moveble grid, containerized for
      multiple instances in the future
- [ ] Calculating an output 2d vector array from an input 2d vector array using
      AST (or other method of function representation) given a range and
      percision determined by the grid's view window in WASM.<sup>1</sup>
- [ ] Rendering a series of points and line from the 2d vector array by only
      graphing the reals
- [ ] Being able to select the coordinate space of the output for different views
- [ ] Multiple views that move simultaneously of the output data
      (calculate once, render n-times)
- [ ] Better input, n-many input boxes, all formatted with KaTeX live rendering

Optional:

- [ ] KaTeX rendering of input/evaluated expression for better looking results
- [ ] 3D grid rendering for more types of complex ouput visualization.
- [ ] Lua/Pyton based scripting api for changing things

---

Note: I am writing this project alongside learning Rust, so in the event that
I learn any new paridigms or design patterns that I want/need to impl. I will
add them here:

- [ ] Debug "mode" to only compile console_log!'s when told.

---

[1] This means that
when using functions based on the mclauren series, the center of the graph
is the a point, and all calculations of surrounding points are passed
through that in order to cut down on making a new approximating function
for each entry in the array.
