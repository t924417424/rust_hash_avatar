use hash_avatar::Generator;
fn main() {
    Generator::new()
        .create()
        .save_to_png("fractal.png")
        .unwrap();
    Generator::new()
        .set_img_size(250)
        .set_padding(10)
        .set_block_num(8)
        .create()
        .save_to_png("fractal2.png")
        .unwrap();
}
