# PPM Rust Library

## Members

* ROUVEL Pauline
* HICHRI Mouna
* CHAMPAUD Alexandre

## Launch Project

To use our library, **cargo** is required.

You can compile the library for "DEBUG" purpose :

`cargo build`

Or you can compile the library for "RELEASE" build :

`cargo build --release`

You can then find the library in the `target/` folder

## Test

We have set up a test fuction for each fuction.

In order to test our functions, run :

`cargo test `

## Benchmark 

In order to test the performance of our code we have set up Benchmark tests.

We used the `Bencher` lib

We can run Benchmark tests we by executing this command:

`cargo bench`


## Structs

### Pixel

#### Create a pixel

`fn new(red: u8, green: u8, blue: u8) -> Pixel {`

To create a pixel, use the struct

Ex: `let p = Pixel{r:255, g:255, b:255}` -> Pixel

#### Getters

`fn get_red(self) -> u8 {`, `fn get_green(self) -> u8 {`, `fn get_blue(self) -> u8 {`

Get the value for color

Ex: `p.get_red()`, `p.get_green()`, `p.get_blue()` -> usize

#### To_String format

`fn to_string(self) -> std::string::String {`

You can format the pixel to a String

Ex: `p.to_string()` -> string

#### Display the Pixel

To display the pixel in the console

`p.display()`

#### Invert the Pixel

To invert the pixel

Ex: `p.invert()`

#### Grayscale the Pixel

To grayscale the pixel

Ex: `p.grayscale()`

### Image

#### Create an Image

To create an Image, call the `Image::new_with_file()` method which returns an Image

Ex : `let i = Image::new_with_file(file);`

This method will process the input file

#### Process a line of pixel

This method is used to create an Image (new_with_file), it processes a `line of pixels` with a `width` to push new Pixels into a given `Vec`

Ex: `let mut pixels : Vec<Pixel> = Image::process_line_of_pixels(pixels, line.to_string(), width)`

#### Invert Filter

`fn invert(&mut self){`

To invert the Image

Ex: `i.invert()`

#### Grayscale Filter

`fn grayscale(&mut self, color : u8){`

To grayscale the Image

Ex: `i.grayscale(140)`

#### Filter the Image on a Color

`fn filter(&mut self, color : char){`

To filter the Image on a given color

Ex : `i.filter("r")`

#### Save the Image in a File

`fn save(&self, filename : &Path){`

To save the image in a File

Ex: `i.save("./test.ppm")`

#### Display the Image

`fn display(&self) {`

To display the image in the console

Ex: `i.display()`
