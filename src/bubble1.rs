use ggez::{ Context, GameResult };
use ggez::event::EventHandler;
use ggez::graphics::{ self, Canvas, Color, DrawMode, DrawParam, Point2 };
use std::collections::HashSet;

pub struct Bubble1
{
    pub list:   Vec<(Color, u32)>,
    pub sorted: bool,

    cursor:    Option<usize>,
    swaps:     usize,
    max_swaps: usize,

    sectors:   Vec<Canvas>,
    updated:   HashSet<usize>,
    offsets:   (f32, f32),
}

impl Bubble1
{
    pub fn new(context: &mut Context, x: f32, y: f32, list: Vec<(Color, u32)>, max_swaps: usize)
        -> GameResult<Bubble1>
    {
        let max_sectors = (list.len() as f32 / max_swaps as f32).ceil() as usize;

        let mut sectors = vec![];
        let mut updated = HashSet::new();
        for i in 0..max_sectors {
            sectors.push(Canvas::with_window_size(context)?);
            updated.insert(i);
        }

        Ok(Bubble1 {
            list:      list,
            sorted:    false,

            cursor:    None,
            swaps:     1,
            max_swaps: max_swaps,

            sectors:   sectors,
            updated:   updated,
            offsets:   (x, y),
        })
    }
}

impl EventHandler for Bubble1
{
    fn update(&mut self, _: &mut Context) -> GameResult<()>
    {
        if self.sorted {
            return Ok(());
        }

        let cursor;

        if let Some(num) = self.cursor {
            cursor = num;
        } else {
            self.swaps = 0;
            cursor     = 1;
        }

        let len = self.list.len();
        let max = (cursor + self.max_swaps).min(len);

        for i in cursor..max {
            if self.list[i - 1].1 > self.list[i].1 {
                self.list.swap(i - 1, i);
                self.swaps += 1;
            }
        }

        self.cursor = Some(max);

        if max >= len {
            self.cursor = None;

            if self.swaps == 0 {
                self.sorted = true;
                return Ok(());
            }
        }

        for i in cursor..max {
            self.updated.insert(i * self.sectors.len() / self.list.len());
        }
        
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()>
    {
        let list_len = self.list.len();
        let sect_len = self.sectors.len();
        let width    = 400;
        let height   = 300;

        let x_size = (1 * width  / list_len).max(1) as f32;
        let y_size = (1 * height / list_len).max(1) as f32;
 
        graphics::set_background_color(context, Color::from([ 0., 0., 0., 0. ]));

        for update in self.updated.iter() {
            graphics::set_canvas(context, Some(&self.sectors[update.clone()]));
            graphics::clear(context);

            let st = (*update)     * list_len / sect_len; 
            let ed = (*update + 1) * list_len / sect_len;

            let old_color = graphics::get_color(context);

            for i in st..ed {
                let (color, value) = self.list[i];
                let value = value as usize;

                let x = self.offsets.0 + (i * width  / list_len) as f32; 
                let y = self.offsets.1 + (height - (value * height / list_len)) as f32; 

                graphics::set_color(context, color)?;
                graphics::polygon(context, DrawMode::Fill, &[
                    Point2::new(x,          y         ),
                    Point2::new(x + x_size, y         ),
                    Point2::new(x + x_size, y + y_size),
                    Point2::new(x,          y + y_size),
                ])?;
            }

            graphics::set_color(context, old_color)?;
        }

        self.updated.clear();

        graphics::set_canvas(context, None);

        for i in 0..self.sectors.len() {
            graphics::draw_ex(context, &self.sectors[i], DrawParam {
                dest: Point2::new(400., 300.),
                ..Default::default()
            })?;
        }
        
        Ok(())
    }
}
