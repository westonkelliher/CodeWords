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

const START_WIDTH:  f32 = 1920.0;
const START_HEIGHT: f32 = 1080.0;


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
    my_window_mode.fullscreen_type = conf::FullscreenType::True;

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
    let my_runner = match MyRunner::new(graphics::size(&ctx)) {
        Ok(r) => r,
        Err(e) => panic!("failed to create MyRunner: {}", e)
    };

    // Run!
    event::run(ctx, event_loop, my_runner);

    
}

#[derive(Debug, PartialEq, Eq)]
enum CardColor {
    Neutral,
    Red,
    Blue,
    Death,
}

fn opposite_color(cc: &CardColor) -> CardColor {
    match cc {
        CardColor::Red  => CardColor::Blue,
        CardColor::Blue => CardColor::Red,
        _ => panic!("No opposite of {:?}", cc),
    }
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
    current_turn: CardColor, // red or blue
    winner: CardColor,
    size: (f32, f32),
}

impl MyRunner {
    fn new(size: (f32, f32)) -> Result<Self> {
        println!("{:?}", size);
        // errs when cant read file
        let s = fs::read_to_string("/home/requin/rqn/words/game_words.txt")?;
        let all_words_ = s.split("\n").collect::<Vec<&str>>(); 
        let all_words = all_words_[..all_words_.len()-1].iter(); // always a newline at the end so last element is empty
        let mut rng = thread_rng();
        let chosen_words = all_words.choose_multiple(&mut rng, 25);
        let mut runner = MyRunner {
            clients: targetlib::get_client_info(),
            word_cards: Vec::new(),
            current_turn: CardColor::Blue,
            winner: CardColor::Neutral,
            size: size,
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

        // assign specs for existing control pads
        for (n, client) in runner.clients.iter().enumerate() {
            targetlib::assign_spec(client,
                                   runner.get_cp_spec(n, client.w, client.h));
        }

        Ok(runner)
    }

    fn num_flipped(&self, cc: CardColor) -> usize {
        let mut sum = 0;
        for row in &self.word_cards {
            for card in row {
                if card.flipped && card.color == cc {
                    sum += 1;
                }
            }
        }
        sum
    }


    fn end_turn(&mut self) {
        self.current_turn = opposite_color(&self.current_turn);
    }

    fn end_game(&mut self, winner: CardColor) {
        self.winner = winner;
        for row in &mut self.word_cards {
            for card in row {
                card.flipped = true;
            }
        }
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
        let mut buttons: Vec<Button> = vec![
        ];
        if cc == opposite_color(&self.current_turn) {
            buttons.push(Button::new(100,
                        w - plr_pnl_w + 4, h - plr_pnl_w + 4,
                        plr_pnl_w - 8, plr_pnl_w - 8));
        }
        let mut panels: Vec<Panel> = vec![
            Panel::new(101,
                       0, 0, plr_pnl_w, h,
                       cardcolor_to_vec(&cc)),
            Panel::new(102,
                       w - plr_pnl_w, 0, plr_pnl_w, h,
                       cardcolor_to_vec(&cc)),
        ];
	// If game is over, provide button to exit back to launcher
	if self.winner != CardColor::Neutral {
            buttons.push(
                Button::new(99,
                            w - plr_pnl_w + 4, (h - plr_pnl_w)/2,
                            plr_pnl_w - 8, plr_pnl_w));
	}
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
                    // Clue giver of the opposite team as is guessing gets buttons
                    if cc == opposite_color(&self.current_turn) {
                        buttons.push(
                            Button::new(j*5 + i,
                                        plr_pnl_w + x_space + i*(btn_w + x_space),
                                        y_space + j*(btn_h + y_space),
                                        btn_w, btn_h)
                        );
                    }
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
        CPSpec::new(panels, buttons, vec![], vec![])
    }
    
}


impl EventHandler<ggez::GameError> for MyRunner {

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut controller_change = false;
        let mut end_turn = false;
        let mut end_game: Option<CardColor> = None;
        for client in self.clients.iter() {
            for event in targetlib::get_events(&client) {
                match event.datum {
                    ControlDatum::Press => {
                        controller_change = true;
                        if event.element_id == 100 {
                            end_turn = true;
                            break;
                        }
                        if event.element_id == 99 {
			    ggez::event::quit(ctx);
			    std::process::exit(0);
                        }
                        let j = event.element_id as usize / 5;
                        let i = event.element_id as usize % 5;
                        if self.word_cards[j][i].color == CardColor::Death {
                            end_game = Some(opposite_color(&self.current_turn));
                            end_turn = true;
                            break;
                        }
                        self.word_cards[j][i].flipped = true;
                        if self.num_flipped(CardColor::Red) == 8 { // TODO: magic numbers
                            end_game = Some(CardColor::Red);
                            end_turn = true;
                            break;
                        }
                        if self.num_flipped(CardColor::Blue) == 9 {
                            end_game = Some(CardColor::Blue);
                            end_turn = true;
                            break;
                        }
                        if self.word_cards[j][i].color != self.current_turn {
                            end_turn = true;
                            break;
                        }
                    }
                    _ => (),
                }
            }
            if end_turn {
                break;
            }
        }
        if let Some(cc) = end_game {
            self.end_game(cc);
        }
        if end_turn {
            self.end_turn();
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
        graphics::clear(ctx, Color::WHITE); 

        // determine dimensions of different areas
        let (sw, sh) = (START_WIDTH, START_HEIGHT);//self.size;
        let prompt_h = sh/20.0;
        let prompt_w = sw;
        let card_area_w = sw;
        let card_area_h = sh - prompt_h;
        let card_area_x = 0.0;
        let card_area_y = prompt_h;

        // if game over, draw end game prompt
        if self.winner != CardColor::Neutral {
            let (w_team, w_color) = match self.winner {
                CardColor::Blue => ("Blue", Color::new(0.7, 0.7, 0.85, 1.0)),
                CardColor::Red =>  ("Red", Color::new(0.85, 0.7, 0.7, 1.0)),
                _ => panic!("Bad card color for winner: {:?}", self.winner),
            };
            let text_win = graphics::Text::new(graphics::TextFragment {
                text: format!("{} Team Wins!", w_team),
                color: Some(Color::BLACK),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::PxScale::from(prompt_h/0.9)),
            });
            let dims_win = text_win.dimensions(ctx);
            let text_x_win = (prompt_w - dims_win.w)/2.0;
            let text_y_win = prompt_h/5.0;
            let r = graphics::Mesh::new_rounded_rectangle( // draw current turn color box
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(text_x_win - 4.0, text_y_win - 4.0,
                                    dims_win.w + 8.0, dims_win.h + 5.0),
                4.0,
                w_color,
            )?;        
            ezdraw!(r);
            ezdrawxy!(text_win, text_x_win, text_y_win);
        }
        
        // draw prompt text
        else {
            let a_team = match self.current_turn {
                CardColor::Blue => "Blue", CardColor::Red => "Red", _ => ""};
            let b_team = match self.current_turn {
                CardColor::Blue => "Red", CardColor::Red => "Blue", _ => ""};
            let team_color = match self.current_turn {
                CardColor::Blue => Color::new(0.7, 0.7, 0.85, 1.0),
                CardColor::Red =>  Color::new(0.85, 0.7, 0.7, 1.0),
                _ => panic!("Bad card color for turn: {:?}", self.current_turn),
            };
            
            let text_1 = graphics::Text::new(graphics::TextFragment {
                text: format!("{} team's turn to guess", a_team),
                color: Some(Color::BLACK),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::PxScale::from(prompt_h/1.1)),
            });
            let dims_1 = text_1.dimensions(ctx);
            let text_x_1 = (prompt_w - dims_1.w)/2.0;
            let text_y_1 = prompt_h/5.0;
            let r = graphics::Mesh::new_rounded_rectangle( // draw current turn color box
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(text_x_1 - 4.0, text_y_1 - 4.0, dims_1.w + 8.0, dims_1.h + 5.0),
                4.0,
                team_color,
            )?;        
            ezdraw!(r);
            ezdrawxy!(text_1, text_x_1, text_y_1);

            let text_2 = graphics::Text::new(graphics::TextFragment {
                text: format!("{} team's Clue Giver must touch the button \
                               corresponding to {} team's guess",
                              b_team, a_team),
                color: Some(Color::BLACK),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::PxScale::from(prompt_h/1.8)),
            });
            let dims_2 = text_2.dimensions(ctx);
            let text_x_2 = (prompt_w - dims_2.w)/2.0;
            let text_y_2 = text_y_1 + dims_1.h + prompt_h/10.0;
            ezdrawxy!(text_2, text_x_2, text_y_2);
            /*let (text_x, text_y) = match self.flipped {
            true => (x + w*0.10,
            y + h*0.68,
        ),
            false => (x + (w - dims.w)/2.0,
            y + (h - dims.h)/2.0,
        ),
        };*/
        }

        // draw cards
        for (j, card_row) in self.word_cards.iter().enumerate() {
            for (i, card) in card_row.iter().enumerate() {
                /*let mut texty = graphics::Text::new(&card.word[..]);
                texty.set_font(graphics::Font::default(), graphics::PxScale::from(24.0));
                let params = graphics::DrawParam::default()
                    .dest([50.0 + (i as f32)*100.0, 50.0 + (j as f32)*100.0]);
                graphics::draw(ctx, &texty, params)?;
                ezdrawxy!(texty, 100.0 + (i as f32)*180.0, 50.0 + (j as f32)*160.0);
                 */
                let card_w = card_area_w/7.0;
                let card_h = card_area_h/9.0;
                let x_space = card_w * (7.0 - 5.0)/6.0;
                let y_space = card_h * (9.0 - 5.0)/6.0;
                card.draw(ctx,
                          card_area_x + x_space + (i as f32)*(card_w+x_space),
                          card_area_y + y_space + (j as f32)*(card_h+y_space),
                          card_w, card_h
                )?;
            }
        }
  
        graphics::present(ctx)
    }

}
