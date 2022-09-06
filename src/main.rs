use ggez::{Context, ContextBuilder, GameResult, conf};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::Color;
use ggez::mint::Point2;
//use ggez::input::mouse::MouseButton;

use std::fs;
use rand::{seq::IteratorRandom, thread_rng};

use std::error::Error;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

const START_WIDTH:  f32 = 1200.0;
const START_HEIGHT: f32 = 800.0;

fn main() {

    // open a window
    let my_window_settings = conf::WindowSetup {
	title: "CodeWords".to_owned(),
	samples: conf::NumSamples::One,
	vsync: true,
	icon: "".to_owned(),
	srgb: true,
    };
    let mut my_window_mode = conf::WindowMode::default();
    my_window_mode.resizable  = true;
    my_window_mode.min_width  = 400.0;
    my_window_mode.min_height = 300.0;
    my_window_mode.width      = START_WIDTH;
    my_window_mode.height     = START_HEIGHT;

    // Make a Context and an EventLoop.
    let (ctx, event_loop) =
       ContextBuilder::new("CodeWords", "weston")
	.window_setup(my_window_settings)
	.window_mode(my_window_mode)
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let my_runner = match MyRunner::new() {
        Ok(r) => r,
        Err(e) => panic!("failed to create MyRunner: {}", e)
    };

    // Run!
    event::run(ctx, event_loop, my_runner);

    
}

struct WordCard {
    word: String,
    // TODO: text_graphic so we dont render each frame
}
impl WordCard {
    fn draw(&self, ctx: &mut Context, x: f32, y: f32, w: f32, h: f32) -> GameResult<()> {
        macro_rules! ezdraw {
            ($a:expr) => {
                graphics::draw(ctx, & $a,  (Point2{x:0.0, y:0.0},))?
            }
        }
        macro_rules! ezdrawxy {
            ($a:expr, $x:expr, $y:expr) => {
                graphics::draw(ctx, & $a,  (Point2{x:$x, y:$y},))?
            }
        }
        // draw rects
        let r1 = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(x, y, w, h),
            h/10.0,
            graphics::Color::new(0.8, 0.7, 0.6, 1.0),
        )?;
        let r2 = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(x+5.0, y+5.0, w-10.0, h-10.0),
            h/10.0,
            graphics::Color::new(0.6, 0.5, 0.2, 1.0),
        )?;
        let r3 = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(x+7.0, y+7.0, w-14.0, h-14.0),
            h/10.0,
            graphics::Color::new(0.9, 0.8, 0.7, 1.0),
        )?;        
        ezdraw!(r1); ezdraw!(r2); ezdraw!(r3);

        // draw text
        let text = graphics::Text::new(graphics::TextFragment {
            text: self.word.clone(),
            color: Some(Color::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(w/7.0)),
        });
        let dims = text.dimensions(ctx);
        let text_x = x + (w - dims.w)/2.0;
        let text_y = y + (h - dims.h)/2.0;
        ezdrawxy!(text, text_x, text_y);
        // done
        Ok(())
    }
}

struct MyRunner {
    word_cards: Vec<Vec<WordCard>>,
}
impl MyRunner {
    fn new() -> Result<Self> {
        // errs when cant read file
        let s = fs::read_to_string("words/game_words.txt")?;
        let all_words = s.split("\n");
        let mut rng = thread_rng();
        let chosen_words = all_words.choose_multiple(&mut rng, 25);
        let mut runner = MyRunner {
            word_cards: Vec::new(),
        };
        for i in 0..5 {
            runner.word_cards.push(Vec::new());
            for j in 0.. 5 {

                runner.word_cards[i].push(WordCard {
                    word: chosen_words[i*5 + j].into(),
                });
            }
        }
        Ok(runner)
    }
}

/*
fn mesh_from(ctx: &mut Context, b: &Button) -> GameResult<graphics::Mesh> {
    let color = match b.depressed {
        true => graphics::Color::new(0.55, 0.55, 0.55, 1.0),
        false => graphics::Color::new(0.7, 0.7, 0.7, 1.0),
    };
    graphics::Mesh::new_rectangle(
        ctx, graphics::DrawMode::fill(),
        graphics::Rect::new(b.x as f32, b.y as f32, b.w as f32, b.h as f32),
        color
    )
}
*/                                                     


impl EventHandler<ggez::GameError> for MyRunner {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, graphics::Color::WHITE); 

        for (j, card_row) in self.word_cards.iter().enumerate() {
            for (i, card) in card_row.iter().enumerate() {
                /*let mut texty = graphics::Text::new(&card.word[..]);
                texty.set_font(graphics::Font::default(), graphics::PxScale::from(24.0));
                let params = graphics::DrawParam::default()
                    .dest([50.0 + (i as f32)*100.0, 50.0 + (j as f32)*100.0]);
                graphics::draw(ctx, &texty, params)?;
                ezdrawxy!(texty, 100.0 + (i as f32)*180.0, 50.0 + (j as f32)*160.0);
                 */
                let (sw, sh) = (START_WIDTH, START_HEIGHT);
                let card_w = sw/7.0;
                let card_h = sh/9.0;
                let x_space = card_w * (7.0 - 5.0)/6.0;
                let y_space = card_h * (9.0 - 5.0)/6.0;
                card.draw(ctx,
                          x_space + (i as f32)*(card_w+x_space),
                          y_space + (j as f32)*(card_h+y_space),
                          card_w, card_h
                )?;
            }
        }
  
        graphics::present(ctx)
    }

}
