#![feature(test)]
extern crate rand;
use std::path::Path;
use std::fs::*;
extern crate libc;
use std::fs::File;
use std::io::LineWriter;
use std::fs;
use std::io::prelude::*;

extern crate test;
use test::Bencher;

extern crate termion;
use termion::{color, style};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_dummy(){
        assert_eq!(dummy(), 42);
    }

    #[bench]
    fn bench_dummy(b: &mut Bencher) {
        b.iter(|| dummy());
    }

    #[test]
    fn test_new_pixel(){
        let p = Pixel{r:3, g:2, b:1};
        assert_eq!(Pixel::new(3, 2, 1), p);
    }

    #[bench]
    fn bench_new_pixel(b: &mut Bencher) {
        b.iter(|| Pixel::new(3, 2, 1));
    }

    #[test]
    fn test_to_string(){
        let p = Pixel{r:3, g:2, b:1};
        let f = "R:3 G:2 B:1";
        assert_eq!(Pixel::to_string(p),f);
    }

    #[bench]
    fn bench_to_string(b: &mut Bencher) {
        let p = Pixel{r:3, g:2, b:1};
        b.iter(|| Pixel::to_string(p));
    }

    //#[test]
    fn test_display_pixel() {
        let p = Pixel{r:140, g:2, b:1};
        p.display();
    }

    //#[test]
    fn test_display_image() {
        let file = Path::new("./test.ppm");
        let image = Image::new_with_file(file);
        image.display();
    }

    #[test]
    fn test_invert() {

        let mut p = Pixel{r:3, g:2, b:1};

        let p1 = Pixel{r:252, g:253, b:254};

        Pixel::invert(&mut p);

        assert_eq!(p, p1);

    }

    #[bench]
    fn bench_invert(b: &mut Bencher) {
        let mut p = Pixel{r:3, g:2, b:1};

        b.iter(|| Pixel::invert(&mut p));
    }

    #[test]
    fn test_grayscale(){

        let mut p = Pixel{r:254, g:156, b:10};

        let p1 = Pixel{r:140, g:140, b:140};

        Pixel::grayscale(&mut p);

        assert_eq!(p, p1);        
    }

    #[bench]
    fn bench_grayscale(b: &mut Bencher) {
        let mut p = Pixel{r:254, g:156, b:10};

        b.iter(|| Pixel::invert(&mut p));
    }

    #[test]
    fn test_eq(){

        let p = Pixel{r:254, g:156, b:10};

        let p1 = Pixel{r:140, g:140, b:140};

        let p2 = Pixel{r:140, g:140, b:140};

        let res = PartialEq::eq(&p, &p1);
        let res2 = PartialEq::eq(&p2, &p1);

        assert_eq!(res, false);
        assert_eq!(res2, true);

    }

    #[bench]
    fn bench_eq(b: &mut Bencher) {
        let p1 = Pixel{r:140, g:140, b:140};
        let p2 = Pixel{r:140, g:140, b:140};

        b.iter(|| PartialEq::eq(&p1, &p2));
    }

    #[test]
    fn test_new_with_file(){
        let file = Path::new("./test.ppm");

        let p = Pixel{r:255, g:0, b:0};
        let p1 = Pixel{r:0, g:255, b:0};
        let p2 = Pixel{r:0, g:0, b:255};
        let p3 = Pixel{r:255, g:255, b:0};
        let p4 = Pixel{r:255, g:255, b:255};
        let p5 = Pixel{r:0, g:0, b:0};

        let i = Image{
            type_i : "P3".to_string(),
            width : 3,
            height : 2,
            max_val : 255,
            pixels : vec![p, p1, p2, p3, p4, p5]
        };

        assert_eq!(Image::new_with_file(file), i)
    }

    #[bench]
    fn bench_new_with_file(b: &mut Bencher) {
        let file = Path::new("./test.ppm");

        b.iter(|| Image::new_with_file(file));
    }

    #[test]
    fn test_invert_image() {

        let p = Pixel{r:3, g:2, b:1};

        let p1 = Pixel{r:252, g:253, b:254};


        let mut i1 = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p, p]
        };

        let i2 = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p1, p1]
        }; 

        Image::invert(&mut i1);

        assert_eq!(i1.pixels, i2.pixels);
    }

    #[bench]
    fn bench_invert_image(b: &mut Bencher) {
        let p = Pixel{r:3, g:2, b:1};
        let mut i = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p, p]
        };

        b.iter(|| Image::invert(&mut i));
    }

    #[test]
    fn test_grayscale_image(){

        let p = Pixel{r:254, g:156, b:10};

        let p1 = Pixel{r:140, g:140, b:140};

        let mut i1 = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p, p]
        };

        let i2 = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p1, p1]
        }; 

        Image::grayscale(&mut i1, 140);

        assert_eq!(i1.pixels, i2.pixels);
    }

    #[bench]
    fn bench_grayscale_image(b: &mut Bencher) {
        let p = Pixel{r:254, g:156, b:10};

        let mut i = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p, p]
        };

        b.iter(|| Image::grayscale(&mut i, 140));
    }

    #[test]
    fn test_filter_image(){

        let p = Pixel{r:254, g:156, b:10};

        let p1 = Pixel{r:254, g:0, b:0};

        let p2 = Pixel{r:112, g:156, b:10};

        let p3 = Pixel{r:112, g:0, b:0};

        let mut i1 = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p, p2]
        };

        let i2 = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p1, p3]
        }; 

        Image::filter(&mut i1, 'r');

        assert_eq!(i1.pixels, i2.pixels);
    }

    #[bench]
    fn bench_filter_image(b: &mut Bencher) {
        let p1 = Pixel{r:254, g:156, b:10};
        let p2 = Pixel{r:112, g:156, b:10};

        let mut i = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p1, p2]
        };

        b.iter(|| Image::filter(&mut i, 'r'));
    }

    #[test]
    fn test_save_image(){
        let p = Pixel{r:255, g:0, b:0};
        let p1 = Pixel{r:0, g:255, b:0};
        let p2 = Pixel{r:0, g:0, b:255};
        let p3 = Pixel{r:255, g:255, b:0};
        let p4 = Pixel{r:255, g:255, b:255};
        let p5 = Pixel{r:0, g:0, b:0};

        let i = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p, p1, p2, p3, p4, p5]
        }; 

        let path = Path::new("./test.ppm");
        i.save(path);

        let image_res = Image::new_with_file(path);
        assert_eq!(i, image_res);
    }

    #[bench]
    fn bench_save_image(b: &mut Bencher) {
        let p = Pixel{r:255, g:0, b:0};
        let p1 = Pixel{r:0, g:255, b:0};
        let p2 = Pixel{r:0, g:0, b:255};
        let p3 = Pixel{r:255, g:255, b:0};
        let p4 = Pixel{r:255, g:255, b:255};
        let p5 = Pixel{r:0, g:0, b:0};

        let mut i = Image{
            type_i:"P3".to_string(),
            width:3,
            height:2,
            max_val:255,
            pixels:vec![p, p1, p2, p3, p4, p5]
        };

        let path = Path::new("./test.ppm");
        b.iter(|| i.save(path));
    }
}

/*___________________________________*/
/*         Lvl 0 : Warmup            */
/*___________________________________*/

 
#[no_mangle]
pub extern fn dummy() -> i32{
    return 42
}

/*___________________________________*/
/*         Lvl 1 : Struct            */
/*___________________________________*/

#[derive(Debug, Clone, Copy)]
pub struct Pixel{
    r : u8,
    g : u8,
    b : u8
} 

/*___________________________________*/
/* Epic functions for an epic pixel  */
/*___________________________________*/
#[no_mangle]
impl Pixel {
    //constructor
    #[no_mangle]
    fn new(red: u8, green: u8, blue: u8) -> Pixel {
        let p = Pixel{
            r:red, 
            g:green, 
            b:blue
        };
        
        p
    }
    

    // Getters
    fn get_red(self) -> u8 {
        self.r
    }

    fn get_green(self) ->u8 {    
        self.g
    }

    fn get_blue(self) -> u8 {
        self.b
    }

    // Format the pixel to_string
    fn to_string(self) -> std::string::String {
        format!("R:{} G:{} B:{}", self.get_red(), self.get_green(), self.get_blue())

    }

    // Displays the pixel in the console
    fn display(self) {
        print!("{color} {reset}", 
            color = color::Bg(color::Rgb(self.get_red(), self.get_green(), self.get_blue())),
            reset = color::Bg(color::Reset));
    }

    // Invert the pixel color
    fn invert(&mut self) {
        self.b = 255 - self.get_blue();
        self.r = 255 - self.get_red();
        self.g = 255 - self.get_green();
    }

    // Convert to grayscale
    fn grayscale(&mut self){
        let r : u64 = (self.get_red()) as u64;
        let g : u64 = (self.get_green()) as u64;
        let b : u64 = (self.get_blue()) as u64;

        let mut total : u64 = (r + g + b) as u64;
        total = total / 3;

        self.r = (total) as u8;
        self.g = (total) as u8;
        self.b = (total) as u8;
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        self.get_red() == other.get_red() &&
        self.get_green() == other.get_green() &&
        self.get_blue() == other.get_blue()
    }
}

/*___________________________________*/
/*   Lvl 2: Image Manipulation       */
/*___________________________________*/

#[derive(Debug, Clone)]
pub struct Image{
    type_i : String,
    width : usize,
    height : usize,
    max_val : usize,
    pixels : Vec<Pixel>
} 
#[warn(irrefutable_let_patterns)]
impl Image {

    // Convert a PPM File into an Image struct
    fn new_with_file(filename: &Path) -> Image {
        let contents : String = read_to_string(filename).expect("Something went wrong reading the file");

        let type_i : String;
        let mut width : usize = 0;
        let mut height : usize = 0;
        let max_val : usize;
        let mut pixels : Vec<Pixel> = Vec::new();

        let mut lines = contents.lines();

        // Get type of netbpm
        match lines.next() {
            Some(t) => type_i = String::from(t),
            None        => type_i = "".to_string(),
        }
        println!("Type : {}", type_i);

        // Get width and height
        if let Some(line) = &lines.next() {
            let mut width_and_height = line.split(" ");

            match width_and_height.next() {
                Some(w) => width = w.to_owned().parse::<usize>().unwrap(),
                None        => width = 0,
            }

            println!("Width : {}", width);

            match width_and_height.next() {
                Some(h) => height = h.to_owned().parse::<usize>().unwrap(),
                None        => height = 0,
            }
            println!("Height : {}", height);
        }

        // Get max
        match lines.next() {
            Some(max) => max_val = max.to_owned().parse::<usize>().unwrap(),
            None        => max_val = 0,
        }
        println!("MaxVal : {}", max_val);


        // Get each pixel lines
        for _i in 0..height {
            match lines.next() { 
                Some(line) => pixels = Image::process_line_of_pixels(pixels, line.to_string(), width),
                None => println!("No more lines"),
            }
        }

        let image_result = Image{
            type_i: type_i, 
            width: width, 
            height: height,
            max_val: max_val,
            pixels: pixels
        };

        image_result
    }

    // Processes a line of pixel to add to the pixel Vec of an Image struct
    fn process_line_of_pixels(mut pixels : Vec<Pixel>, line : String, width : usize) -> Vec<Pixel> {
        let mut pixel_values = line.split_whitespace();

        for _i in 0..width {
            let mut pixel_vec = Vec::new();

            // Push each 3 value into a new pixel
            for _j in 0..3{
                match pixel_values.next() {
                    Some(pixel_line_value) => pixel_vec.push(pixel_line_value.to_owned().parse::<u8>().unwrap()),
                    None => println!("A pixel is supposed to be 3 values")
                }
            }

            let pixel = Pixel{
                r:pixel_vec[0],
                g:pixel_vec[1],
                b:pixel_vec[2]
            };

            pixels.push(pixel);
        }
        pixels
    }
    
    // Inver an Image
    fn invert(&mut self){
        for i in 0..self.pixels.len() {
            self.pixels[i].b = 255 - self.pixels[i].get_blue();
            self.pixels[i].r = 255 - self.pixels[i].get_red();
            self.pixels[i].g = 255 - self.pixels[i].get_green();
        }
    }

    // Converts an Image to grayscale
    fn grayscale(&mut self, color : u8){
        for i in 0..self.pixels.len() {

            self.pixels[i].r = color;
            self.pixels[i].g = color;
            self.pixels[i].b = color;
        }
    }

    // Filters an Image on a given color
    fn filter(&mut self, color : char){
        for i in 0..self.pixels.len() {
            if color == 'r' {
                self.pixels[i].g = 0;
                self.pixels[i].b = 0;
            }
            else if color == 'b' {
                self.pixels[i].g = 0;
                self.pixels[i].r = 0;
            }
            else if color == 'g' {
                self.pixels[i].b = 0;
                self.pixels[i].r = 0;
            }
        }
    }

    // Saves the Image struct to a File
    fn save(&self, filename : &Path){
        fs::remove_file(filename);
        let mut file = OpenOptions::new().append(true).create(true).open(filename).unwrap();
        
        write!(&mut file, "{}\n", self.type_i);
        write!(&mut file, "{} {}\n",  self.width, self.height);
        write!(&mut file, "{}\n",  self.max_val);

        for i in 0..self.pixels.len() {

            if(i % 3 == 0 && i != 0){
                write!(&mut file, "\n");
            }
            write!(&mut file, "{:?} {:?} {:?}   ",  self.pixels[i].r, self.pixels[i].g, self.pixels[i].b );
        }
    }

    // Prints the Image in the console
    fn display(&self) {
        for i in 1..self.pixels.len()+1 {
            self.pixels[i-1].display();
            if (i % self.width == 0 && i != 0) {
                print!("\n");
            }
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Image) -> bool {
        self.type_i == other.type_i &&
        self.width == other.width &&
        self.height == other.height &&
        self.max_val == other.max_val &&
        self.pixels == other.pixels
    }
}