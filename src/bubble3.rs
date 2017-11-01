use ggez::{ Context, GameResult };
use ggez::event::EventHandler;
use ggez::graphics::{ self, Canvas, Color, DrawMode, DrawParam, Point2 };
use std::collections::HashSet;

pub struct Bubble3
{
    pub list:   Vec<(Color, u32)>,
    pub sorted: bool,

    cursor:         Option<usize>,
    max_cursor:     usize,
    new_max_cursor: usize,
    max_swaps:      usize,

    sectors: Vec<Canvas>,
    updated: HashSet<usize>,
    offsets: (f32, f32),
}            

impl Bubble3
{
    pub fn new(context: &mut Context, x: f32, y: f32, list: Vec<(Color, u32)>, max_swaps: usize)
        -> GameResult<Bubble3>
    {
        let list_len    = list.len();
        let max_sectors = (list_len as f32 / max_swaps as f32).ceil() as usize;

        let mut sectors = vec![];
        let mut updated = HashSet::new();
        for i in 0..max_sectors {
            sectors.push(Canvas::with_window_size(context)?);
            updated.insert(i);
        }

        Ok(Bubble3 {
            list:      list,
            sorted:    false,

            cursor:         None,
            max_cursor:     list_len,
            new_max_cursor: 0,
            max_swaps:      max_swaps,

            sectors: sectors,
            updated: updated,
            offsets: (x, y),
        })
    }
}

impl EventHandler for Bubble3
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
            cursor = 1;
            self.new_max_cursor = 0;
        }

        let max = (cursor + self.max_swaps).min(self.max_cursor);

        for i in cursor..max {
            if self.list[i - 1].1 > self.list[i].1 {
                self.list.swap(i - 1, i);
                self.new_max_cursor = i;
            }
        }

        self.cursor = Some(max);

        if max >= self.max_cursor {
            self.cursor     = None;
            self.max_cursor = self.new_max_cursor;

            if self.max_cursor == 0 {
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
        let width    = 400 as usize;
        let height   = 300 as usize;

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
