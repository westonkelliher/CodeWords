use ggez::{Context, ContextBuilder, GameResult, conf};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::Color;
use ggez::mint::Point2;
//use ggez::input::mouse::MouseButton;

use targetlib::{CPClient, CPSpec, Panel, Button, ControlDatum};//, Joystick};

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

enum CardColor {
    Neutral,
    Red,
    Blue,
    Death,
}

fn cardcolor_to_vec(cc: &CardColor) -> [u8; 4] {
    match cc {
        CardColor::Neutral => [220, 220, 220, 255],
        CardColor::Red     => [240, 200, 200, 255],
        CardColor::Blue    => [200, 200, 240, 255],
        CardColor::Death   => [120, 120, 120, 255],
    }
}

struct WordCard {
    word: String,
    // TODO: text_graphic so we dont render each frame
    color: CardColor,
    flipped: bool,
}
impl WordCard {
    fn draw(&self, ctx: &mut Context, x: f32, y: f32, w: f32, h: f32) -> GameResult<()> {
        // determine exact colors for the card
        let (c1, c2, c3) = if self.flipped {
            match self.color {
                CardColor::Neutral =>
                    (Color::new(0.75, 0.75, 0.75, 1.0),
                     //Color::new(0.45, 0.5, 0.4, 1.0),
                     Color::new(0.85, 0.85, 0.85, 1.0),
                     Color::new(0.85, 0.85, 0.85, 1.0)),
                CardColor::Blue =>
                    (Color::new(0.6, 0.6, 0.8, 1.0),
                     //Color::new(0.35, 0.35, 0.55, 1.0),
                     Color::new(0.7, 0.7, 0.85, 1.0),
                     Color::new(0.7, 0.7, 0.85, 1.0)),
                CardColor::Red =>
                    (Color::new(0.8, 0.6, 0.6, 1.0),
                     //Color::new(0.55, 0.35, 0.35, 1.0),
                     Color::new(0.85, 0.7, 0.7, 1.0),
                     Color::new(0.85, 0.7, 0.7, 1.0)),
                CardColor::Death =>
                    (Color::new(0.4, 0.4, 0.4, 1.0),
                     //Color::new(0.1, 0.1, 0.1, 1.0),
                     Color::new(0.5, 0.5, 0.5, 1.0),
                     Color::new(0.5, 0.5, 0.5, 1.0)),
            }
        } else {
            (Color::new(0.8, 0.7, 0.6, 1.0),
             Color::new(0.6, 0.5, 0.2, 1.0),
             Color::new(0.9, 0.8, 0.7, 1.0),)
        };

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
            c1,
        )?;
        let r2 = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(x+5.0, y+5.0, w-10.0, h-10.0),
            h/10.0,
            c2,
        )?;
        let r3 = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(x+7.0, y+7.0, w-14.0, h-14.0),
            h/10.0,
            c3,
        )?;        
        ezdraw!(r1); ezdraw!(r2); ezdraw!(r3);

        // draw text
        let (text_scale, mut text_color) = match self.flipped {
            true => (w/8.0, Color::new(0.5, 0.5, 0.5, 1.0)),
            false => (w/7.0, Color::new(0.05, 0.05, 0.05, 1.0)),
        };
        if let CardColor::Death = self.color {
            if self.flipped == true {
                text_color = Color::new(1.0, 1.0, 1.0, 1.0);
            }
        }
        let text = graphics::Text::new(graphics::TextFragment {
            text: self.word.clone(),
            color: Some(text_color),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(text_scale)),
        });
        let dims = text.dimensions(ctx);
        let (text_x, text_y) = match self.flipped {
            true => (x + w*0.10,
                     y + h*0.68,
            ),
            false => (x + (w - dims.w)/2.0,
                      y + (h - dims.h)/2.0,
            ),
        };
        ezdrawxy!(text, text_x, text_y);
        // done
        Ok(())
    }
}

struct MyRunner {
    clients: Vec<CPClient>,
    word_cards: Vec<Vec<WordCard>>,
    //current_turn: CardColor, // red or blue
}

impl MyRunner {
    fn new() -> Result<Self> {
        // errs when cant read file
        let s = fs::read_to_string("words/game_words.txt")?;
        let all_words_ = s.split("\n").collect::<Vec<&str>>(); 
        let all_words = all_words_[..all_words_.len()-1].iter(); // always a newline at the end so last element is empty
        let mut rng = thread_rng();
        let chosen_words = all_words.choose_multiple(&mut rng, 25);
        let mut runner = MyRunner {
            clients: Vec::new(),
            word_cards: Vec::new(),
        };
        // randomly choose card colors
        let card_indices: Vec<usize> = (0..25).collect();
        let non_neutrals = card_indices.clone().into_iter().choose_multiple(&mut rng, 18);
        let blue_and_death = non_neutrals.clone().into_iter().choose_multiple(&mut rng, 10);
        let death = blue_and_death.clone().into_iter().choose_multiple(&mut rng, 1);
        // initiate cards
        for j in 0..5 {
            runner.word_cards.push(Vec::new());
            for i in 0..5 {
                let i_flat = j*5 + i;
                let color = if death.contains(&i_flat) {
                    CardColor::Death
                } else if blue_and_death.contains(&i_flat) {
                    CardColor::Blue
                } else if non_neutrals.contains(&i_flat) {
                    CardColor::Red
                } else {
                    CardColor::Neutral
                };
                runner.word_cards[j].push(WordCard {
                    word: (*chosen_words[i_flat as usize]).into(),
                    color: color,
                    flipped: false,
                });
            }
        }
        Ok(runner)
    }

    fn get_cp_spec(&self, ctlr_num: usize, w: u32, h: u32) -> CPSpec {
        let cc = match ctlr_num {
            0 => CardColor::Red,
            1 => CardColor::Blue,
            _ => CardColor::Death,
        };

        let main_w = (w*8)/10;
        let plr_pnl_w = (w - main_w)/2;
        let btn_w = main_w/7;
        let btn_h = h/9;
        let x_space = (btn_w as f32 * (7.0 - 5.0)/6.0) as u32;
        let y_space = (btn_h as f32 * (9.0 - 5.0)/6.0) as u32;
        let mut buttons: Vec<Button> = vec![];
        let mut panels: Vec<Panel> = vec![
            Panel::new(101,
                       0, 0, plr_pnl_w, h,
                       cardcolor_to_vec(&cc)),
            Panel::new(102,
                       w - plr_pnl_w, 0, plr_pnl_w, h,
                       cardcolor_to_vec(&cc)),
        ];
        for j in 0..5_u32 {
            for i in 0..5_u32 {
                let card = &self.word_cards[j as usize][i as usize];
                if !card.flipped {
                    panels.push(
                        Panel::new(j*5 + i,
                                   plr_pnl_w + x_space + i*(btn_w + x_space) - 10,
                                   y_space + j*(btn_h + y_space) - 10,
                                   btn_w + 20, btn_h + 20,
                                   cardcolor_to_vec(&card.color))
                    );
                    buttons.push(
                        Button::new(j*5 + i,
                                    plr_pnl_w + x_space + i*(btn_w + x_space),
                                    y_space + j*(btn_h + y_space),
                                    btn_w, btn_h)
                    );
                } else {
                    panels.push(
                        Panel::new(j*5 + i,
                                   plr_pnl_w + x_space + i*(btn_w + x_space) - 10,
                                   y_space + j*(btn_h + y_space) - 10,
                                   btn_w + 20, btn_h + 20,
                                   cardcolor_to_vec(&card.color))
                    );
                }
            }
        }
        CPSpec::new(panels, buttons, vec![])
    }
}


impl EventHandler<ggez::GameError> for MyRunner {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut controller_change = false;
        for client in self.clients.iter() {
            for event in targetlib::get_events(&client) {
                match event.datum {
                    ControlDatum::Press => {
                        let j = event.element_id as usize / 5;
                        let i = event.element_id as usize % 5;
                        self.word_cards[j][i].flipped = true;
                        controller_change = true;
                    }
                    _ => (),
                }
            }
        }

        if targetlib::clients_changed() || controller_change {
            self.clients = targetlib::get_client_info();
            // asign specs
            for (n, client) in self.clients.iter().enumerate() {
                targetlib::assign_spec(client,
                                       self.get_cp_spec(n, client.w, client.h));
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, Color::WHITE); 

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
