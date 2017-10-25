extern crate ggez;
extern crate rand;

use ggez::{ conf, Context, event, GameResult, graphics };
use rand::{ Rng, thread_rng };
use std::sync::{ Arc, Mutex, MutexGuard, PoisonError };
use std::thread;
use std::time::Duration;

struct List<T>(Arc<Mutex<Vec<T>>>);

impl<T> List<T>
{
    fn new(list: Vec<T>) -> List<T>
    {
        List(Arc::new(Mutex::new(list)))
    }

    fn clone(&self) -> List<T>
    {
        List(self.0.clone())
    }

    fn shuffle(&mut self)
    {
        let mut list = self.0.lock().unwrap();
        thread_rng().shuffle(&mut (*list)[..]);
    }

    fn lock<'a>(&'a self) -> Result<MutexGuard<'a, Vec<T>>, PoisonError<MutexGuard<'a, Vec<T>>>>
    {
        self.0.lock()
    }
}

struct Main
{
    width:  u32,
    height: u32,

    list: List<(graphics::Color, u32)>,

    running: bool,
    test:    Box<Fn(List<(graphics::Color, u32)>) + Send + Sync + 'static>,
}

fn bubble_sort<T>(list: List<T>)
{

}

fn test<T: Send + Sync + 'static>(list: List<T>)
{
    thread::spawn(move || { (bubble_sort)(list); });
}

impl Main
{
    fn new(context: &mut Context) -> GameResult<Main>
    {
        let mut rng = rand::thread_rng();
        let mut v   = vec![];

        for i in 0..10000 {
            let r = rng.gen_range(0, 255);
            let g = rng.gen_range(0, 255);
            let b = rng.gen_range(0, 255);

            v.push((graphics::Color::from((r, g, b)), i));
        }

        let mut list = List::new(v);
        list.shuffle();

        Ok(Main {
            width:  context.conf.window_width,
            height: context.conf.window_height,

            list: list,

            running: false,
            test: Box::new(bubble_sort),
        })
    }
}

impl event::EventHandler for Main
{
    fn update(&mut self, context: &mut Context, dt: Duration) -> GameResult<()>
    {
        if !self.running {
            test(self.list.clone());
            self.running = true;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()>
    {
        graphics::clear(context);

        let mut list = self.list.lock().unwrap();
        let     max  = list.len();

        for i in 0..max {
            graphics::set_color(context, list[i].0);
            
            let x = i         as u32 * self.width  / max as u32;
            let y = list[i].1 as u32 * self.height / max as u32;

            graphics::rectangle(context, graphics::DrawMode::Fill, graphics::Rect::new(x as f32, y as f32, 1., 1.))?;
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
