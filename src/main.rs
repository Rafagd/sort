extern crate ggez;
extern crate rand;
extern crate sort;

use ggez::{ conf, Context, event, GameResult, graphics };
use rand::{ Rng, thread_rng };
use sort::*;

struct Main
{
    bg_color: graphics::Color,
    tests:    Vec<Box<event::EventHandler>>,
}

impl Main
{
    fn new(context: &mut Context) -> GameResult<Main>
    {
        let list_size = 1000;
        let part_size = 100;

        let mut rng  = rand::thread_rng();
        let mut list = vec![];

        graphics::clear(context);

        for i in 0..list_size {
            let r = rng.gen_range(0, 255);
            let g = rng.gen_range(0, 255);
            let b = rng.gen_range(0, 255);

            list.push((graphics::Color::from((r, g, b)), i));
        }

        thread_rng().shuffle(&mut list[..]);

        Ok(Main {
            bg_color: graphics::get_background_color(context),
            tests:  vec![
                Box::new(Bubble1::new(context, 0.,     0., list.clone(), part_size.clone())?),
                Box::new(Bubble2::new(context, 400.,   0., list.clone(), part_size.clone())?),
                Box::new(Bubble3::new(context, 0.,   300., list,         part_size)?),
            ],
        })
    }
}

impl event::EventHandler for Main
{
    fn update(&mut self, context: &mut Context) -> GameResult<()>
    {   
        for test in self.tests.iter_mut() {
            test.update(context)?;
        }
        
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()>
    {
        graphics::set_canvas(context, None);
        graphics::set_background_color(context, self.bg_color);
        graphics::clear(context);
 
        for test in self.tests.iter_mut() {
            test.draw(context)?;
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
