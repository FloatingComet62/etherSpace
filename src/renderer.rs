pub trait Renderer {
    /// Just draw a rectangle to the screen
    /// x -> abscissa of top left corner
    /// y -> ordinate of top left corner
    /// width -> width of rectangle
    /// height -> height of rectangle
    fn draw_rectangle(x: u32, y: u32, width: u32, height: u32);

    /// Just draw a circle to the screen
    /// x -> abscissa of center
    /// y -> ordinate of center
    fn draw_circle(x: u32, y: u32, radius: u32);
}
