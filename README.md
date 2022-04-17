<div align="center">
  <h1><code>sidewinder</code></h1>
  <p><strong>CPU path tracer.</strong></p>
</div>

Based on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

![The final render](https://github.com/tedbyron/sidewinder/blob/main/book1/13.1.png)

# Usage

```sh
USAGE:
    sidewinder [OPTIONS] [PATH]

ARGS:
    <PATH>    Output path

OPTIONS:
    -d, --depth <MAX_DEPTH>              Diffuse reflection recursion depth [default: 50]
    -f, --force                          Overwrite existing files
    -h, --help                           Print help information
    -r, --aspect-ratio <ASPECT_RATIO>    Image aspect ratio [default: 1.7777777777777777]
    -s, --samples <SAMPLES_PER_PIXEL>    Antialiasing samples per pixel [default: 100]
    -V, --version                        Print version information
    -w, --width <IMAGE_WIDTH>            Image width [default: 400]
```

# Examples

Render an image 1920 pixels wide, with a 3:2 aspect ratio, using 1000 samples per pixel, limiting
the diffuse reflection depth to 100 recursive calls, and saving the output to the file `out.ppm`:

```sh
sidewinder -w 1920 -r $((3.0 / 2.0)) -s 1000 -d 100 out.ppm
```
