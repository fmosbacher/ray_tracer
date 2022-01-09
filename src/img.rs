pub fn generate_ppm(
    resolution: (u32, u32),
    max_value: u32,
    pixels: Vec<(u32, u32, u32)>,
) -> String {
    let (width, height) = resolution;
    let ppm_header = format!("P3 {} {} {}", width, height, max_value);
    let pixels_string = pixels
        .iter()
        .map(|(r, g, b)| format!("{} {} {}", r, g, b))
        .collect::<Vec<String>>()
        .join("\n");

    [ppm_header, pixels_string].join("\n")
}
