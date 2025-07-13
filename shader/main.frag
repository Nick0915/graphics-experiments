#version 460 core

uniform vec2 u_resolution;
uniform vec2 u_pan;
uniform float u_zoom;

out vec3 color;

vec2 screen2ndc(vec2 screen_coords) {
    return (screen_coords / u_resolution - 0.5f) * 2.f;
}

vec2 ndc2screen(vec2 ndc_coords) {
    return ((ndc_coords / 2.f) + .5f) * u_resolution;
}

vec3 lerp(vec3 a, vec3 b, float t) {
    return a + (b - a) * t;
}

vec3 color_interp(vec3 a, vec3 b, float t, float rate) {
    return a + (b - a) * pow(t, rate);
}

float ease_out_expo(float t) {
    return (t == 1.f) ? 1.f : 1.f - pow(2.f, -10.f * t);
}

#define MAX_ITER uint(100)
#define COLOR_START (vec3(50.f, 20.f, 100.f) / 255.f)
#define COLOR_END (vec3(230.f, 110.f, 62.f) / 255.f)

vec3 mandelbrot_color(vec2 c) {
    uint i;
    vec2 z = vec2(0.f, 0.f);

    // int max_iter = int(2 / u_zoom + 100);
    int max_iter = 5000;

    for (i = 0; i < max_iter; i++) {
        if (z.x * z.x + z.y * z.y > 4.f) { break; }

        vec2 z_new = z;
        z_new.x = z.x * z.x - z.y * z.y;
        z_new.y = 2.f * z.x * z.y;

        z_new += c;

        z.x = z_new.x;
        z.y = z_new.y;
    }

    if (i == max_iter) {
        return vec3(0.f, 0.f, 0.f);
    }

    // return COLOR_END;

    return color_interp(COLOR_START, COLOR_END, float(i) / (max_iter - 1), .2);
    // return lerp(COLOR_START, COLOR_END, float(i) / (MAX_ITER - 1));
}

void main() {
    vec2 xy = (screen2ndc(gl_FragCoord.xy) * u_zoom) + screen2ndc(u_pan);
    // vec2 colorxy = (xy + 1.f) / 2.f;

    // if (xy.x < -1.f || xy.y < -1.f || xy.x >= 1.f || xy.y >= 1.f) {
    //     color = vec3(0.f, 0.f, 0.f);
    // } else {
    //     color = vec3(colorxy, 1.f);
    //     color = mandelbrot_color(xy);
    // }
    color = mandelbrot_color(xy);
}