use crossbeam::channel;
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, Event, KeyCode}, style::Color};
use fruitmachine::{render::render, frame::{self, Drawable}, credit::Credit, NUM_COLS, NUM_ROWS, reels::Reels, winmeter::WinMeter};
use rusty_audio::Audio;
use std::{io, thread, error::Error, time::{Duration, Instant}};

fn main() -> Result<(), Box<dyn Error>> {
    // Set up sounds
    let mut audio=Audio::new();
    audio.add("start","sounds/start.wav");
    audio.add("stop","sounds/stop.wav");
    audio.add("win50","sounds/win50.wav");
    audio.add("win100","sounds/win100.wav");
    audio.add("win500","sounds/win500.wav");

    // Set up terminal and frame renderer
    let mut stdout=io::stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();

    // Render thread
    let (render_tx,render_rx)=channel::unbounded();
    let render_handle = thread::spawn(move || {
        let mut last_frame=frame::Frame::new(Color::Blue);
        let mut stdout=io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame=match render_rx.recv() {
                Ok(x) => x,
                Err(_)=> break,
            };
            render(&mut stdout, &last_frame, &curr_frame,false);
            last_frame=curr_frame;
        }
    });

    let mut reels=Reels::new();
    let mut credit=Credit::new(NUM_COLS/5,NUM_ROWS-2,100_00);
    let mut win_meter=WinMeter::new(NUM_COLS*3/5,NUM_ROWS-2);
    let mut fast_test=false;
    let mut total_games:u32=0;
    let mut total_stake:u32=0;
    let mut total_wins:u32=0;
    let mut spinning=false;
    let mut instant=Instant::now();
    let mut force=["","",""];

    // Game loop
    'gameloop: loop {
        // Per-frame init
        let mut curr_frame=frame::Frame::new(Color::Blue);
        let delta=if fast_test { Duration::from_secs(5) } else {instant.elapsed()};
        instant=Instant::now();
        let mut start_game=false;

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event)=event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if !spinning && credit.enough_credit(20) {
                            start_game=true;
                        }
                    },
                    KeyCode::Char('f') => {
                        fast_test=true;
                    },
                    KeyCode::Char('1') => {
                        force=["Cherry","Cherry","Lemon"];
                        start_game=true;
                    },
                    KeyCode::Char('2') => {
                        force=["Seven","Seven","Seven"];
                        start_game=true;
                    },
                    KeyCode::Char('3') => {
                        force=["Bell","Bell","Bell"];
                        start_game=true;
                    },
                    _ => {}
                }
            }
        }
        if start_game || (!spinning && fast_test) {
            if fast_test && !credit.enough_credit(20) {
                credit.add_win(100_00);
            }
            credit.take_credit(20);
            win_meter.set_amount(0);
            if !fast_test {
                audio.play("start");
                thread::sleep(Duration::from_millis(200));
            }
            reels.spin(fast_test,&force);
            force=["","",""];
            spinning=true;
        }
        if spinning {
            if !reels.update_spin(delta,&mut audio,fast_test) {
                let win=reels.calculate_win();
                credit.add_win(win);
                win_meter.set_amount(win);
                spinning=false;
                if fast_test {
                    total_games+=1;
                    total_stake+=20;
                    total_wins+=win;
                    if total_games==100_000 {
                        break 'gameloop;
                    }
                } else {
                    match win {
                        0_50 => audio.play("win50"),
                        1_00 => audio.play("win100"),
                        5_00 => audio.play("win500"),
                        _ => {}
                    }
                }
            }
        }
        credit.draw(&mut curr_frame);
        win_meter.draw(&mut curr_frame);
        reels.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    if fast_test {
        println!("Fast test complete");
        println!("Games: {}",total_games);
        println!("VTP: £{:.2}",(total_stake as f64)/100.0);
        println!("Win: £{:.2}",(total_wins as f64)/100.0);
        println!("RTP: {}%",(total_wins as f64)*100.0/(total_stake as f64));
    }
    Ok(())
}
