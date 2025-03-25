use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

const W: f32 = 1800.0;
const H: f32 = 1500.0;
const SCALE: f32 = 20.0;
const COLS: usize = (W / SCALE) as usize;
const ROWS: usize = (H / SCALE) as usize;

struct Model {
    terrain: [[f32; ROWS]; COLS],
    flying: f32,
    perlin: Perlin,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

// Equivalent to Processing setup() function.
// Provides a set of global variables for the terain.
fn model(_app: &App) -> Model {
    Model {
        terrain: [[0.0; ROWS]; COLS],
        flying: 0.0,
        perlin: Perlin::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Scroll vertically (Y axis)
    model.flying += 0.1; // Set the scroll speed.
    let mut x_off = 0.0;
    
    for x in 0..COLS {
        let mut y_off = model.flying; // Scroll by the scroll speed.
        for y in 0..ROWS {
            // Sample 2D Perlin noise (returns [-1, 1], mapped to [-100, 100])
            let noise_val = model.perlin.get([x_off, y_off as f64]);
            model.terrain[x][y] = map_range(noise_val as f32, -1.0, 1.0, -100.0, 100.0);
            y_off += 0.1;
        }
        x_off += 0.1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    
    let purple = srgba(150.0/255.0, 0.0, 200.0/255.0, 1.0);
    let turquoise = srgba(64.0/255.0, 224.0/255.0, 208.0/255.0, 1.0);
    
    // Simple rotation - just around X axis
    let transform = Mat4::from_rotation_x(PI / 3.0)
        * Mat4::from_rotation_z(PI / 36.0)
        * Mat4::from_translation(vec3(-W / 2.0, -H / 1.2, 0.0));
    
    for y in 0..ROWS-1 {
        let mut vertices = Vec::new();
        let mut colors = Vec::new();
        
        for x in 0..COLS {
            let t = map_range(
                (x as f32 - COLS as f32 / 2.0).abs(), 
                0.0, 
                COLS as f32 / 2.0, 
                0.0, 
                1.0
            ).clamp(0.0, 1.0);
            let stroke_color = lerp_color(turquoise, purple, t);
            
            vertices.push(transform.transform_point3(pt3(
                x as f32 * SCALE,
                y as f32 * SCALE,
                model.terrain[x][y],
            )));
            colors.push(stroke_color);
            
            vertices.push(transform.transform_point3(pt3(
                x as f32 * SCALE,
                (y + 1) as f32 * SCALE,
                model.terrain[x][y + 1],
            )));
            colors.push(stroke_color);
        }
        
        draw.polyline()
            .weight(1.0)
            .points_colored(vertices.into_iter().zip(colors.into_iter()));
    }
    
    draw.to_frame(app, &frame).unwrap();
}

fn lerp_color(a: Srgba, b: Srgba, t: f32) -> Srgba {
    Srgba::new(
        a.red + (b.red - a.red) * t,
        a.green + (b.green - a.green) * t,
        a.blue + (b.blue - a.blue) * t,
        a.alpha + (b.alpha - a.alpha) * t,
    )
}
