use raylib::prelude::*;

fn main() {
    let image_width = 500;
    let image_height = 500;

    let mut new_image = Image::gen_image_color(
        image_width, image_height, Color::BLACK
    );
    
    let pixel_position_x = 320;
    let pixel_pposition_y = 250;

    new_image.draw_pixel(
        pixel_position_x, pixel_pposition_y, Color::WHITE
    );

    let output_file_name = "my_first_image.png";

    new_image.export_image(output_file_name);
    
    println!("Image created and saved successfully as '{}'!", output_file_name);
}
