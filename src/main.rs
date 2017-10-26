extern crate ggez;
extern crate rand;

use ggez::{ conf, Context, event, GameResult, graphics };
use rand::{ Rng, thread_rng };
use std::cmp;
use std::time::Duration;

struct Bubble
{
    pub list: Vec<(graphics::Color, u32)>,

    cursor:    Option<usize>,
    swaps:     usize,
    max_swaps: usize,
}

impl Bubble
{
    fn new(list: Vec<(graphics::Color, u32)>, max_swaps: usize) -> Bubble
    {
        Bubble {
            list:      list,
            cursor:    None,
            swaps:     1,
            max_swaps: max_swaps,
        }
    }

    fn run(&mut self) -> Option<(usize, usize)>
    {
        let cursor;

        if let Some(num) = self.cursor {
            cursor = num;
        } else {
            self.swaps = 0;
            cursor     = 1;
        }

        let len = self.list.len();
        let max = cmp::min(cursor + self.max_swaps, len);

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
                return None;
            }
        }

        Some((cursor - 1, max))
    }
}

struct Main
{
    width:  u32,
    height: u32,

    updated: (usize, usize),
    test:    Bubble,
}

impl Main
{
    fn new(context: &mut Context) -> GameResult<Main>
    {
        let mut rng  = rand::thread_rng();
        let mut list = vec![];

        graphics::clear(context);

        for i in 0..100 {
            let r = rng.gen_range(0, 255);
            let g = rng.gen_range(0, 255);
            let b = rng.gen_range(0, 255);

            list.push((graphics::Color::from((r, g, b)), i));
        }

        thread_rng().shuffle(&mut list[..]);
        
        Ok(Main {
            width:  context.conf.window_width,
            height: context.conf.window_height,

            updated: (0, list.len()),
            test:    Bubble::new(list, 100),
        })
    }
}

impl event::EventHandler for Main
{
    fn update(&mut self, context: &mut Context, dt: Duration) -> GameResult<()>
    {
        if let Some(update) = self.test.run() {
            self.updated = update;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()>
    {
        macro_rules! ro3 {
            ($a: expr, $b: expr, $c: expr) => {
                ($a as f32) * ($b as f32) / ($c as f32)
            };
        }

        let list     = &mut self.test.list;
        let len      = list.len();
        let (st, ed) = self.updated;

        let px_size = ro3!(1., self.width, len);

        let gst = ro3!(st, self.width, len);
        let ged = ro3!(ed, self.width, len);
        
        graphics::clear(context);
        /*
        graphics::rectangle(context, graphics::DrawMode::Fill, graphics::Rect::new(
            0. + self.width as f32 / 2.,
            0. + self.height as f32 / 2.,
            ged - gst,
            self.height as f32
        ))?;
        */
        for i in st..ed {
            graphics::set_color(context, list[i].0);
            
            let x = ro3!(i,         self.width,  len) + px_size / 2.;
            let y = ro3!(list[i].1, self.height, len) + px_size / 2.;
            let y = self.height as f32 - y;

            println!("{} {} {} ", x, y, px_size);

            graphics::rectangle(context, graphics::DrawMode::Fill,
                graphics::Rect::new(x, y, px_size, px_size)
            )?;
        }

        graphics::present(context);
        Ok(())
    }
}

fn main()
{
    let     config  = conf::Conf::new();
    let mut context = Context::load_from_conf("drawing", "ggez", config).unwrap();
    let mut state   = Main::new(&mut context).unwrap();

    if let Err(e) = event::run(&mut context, &mut state) {
        println!("Error: {}", e);
    }
}
