use std::collections::HashMap;
use std::cmp::Ordering;


#[derive(Debug)]
struct Record {
    win: u32,
    draw: u32,
    lose: u32,
}

impl Record {
    fn new(win: u32, draw: u32, lose: u32) -> Record {
        Record {
            win: win,
            draw: draw,
            lose: lose,
        }
    }

    fn win(&mut self) {
        self.win += 1
    }

    fn draw(&mut self) {
        self.draw += 1;
    }

    fn lose(&mut self) {
        self.lose += 1;
    }

    fn matches(&self) -> u32 {
        self.win + self.draw + self.lose
    }

    fn scores(&self) -> u32 {
        self.win * 3 + self.draw
    }
}

fn add_winer(matches: &mut HashMap<String, Record>, team: &str) {
    matches
        .entry(team.to_string())
        .or_insert_with(|| Record::new(0, 0, 0))
        .win();
}

fn add_loser(matches: &mut HashMap<String, Record>, team: &str) {
    matches
        .entry(team.to_string())
        .or_insert_with(|| Record::new(0, 0, 0))
        .lose();
}

fn add_drawn(matches: &mut HashMap<String, Record>, team: &str) {
    matches
        .entry(team.to_string())
        .or_insert_with(|| Record::new(0, 0, 0))
        .draw();
}

pub fn tally(input: &str) -> String {
    let mut matches = HashMap::new();

    for text in input.lines() {
        let cmd = text.split(';').collect::<Vec<_>>();

        match cmd[2] {
            "win" => {
                add_winer(&mut matches, cmd[0]);
                add_loser(&mut matches, cmd[1])
            }
            "draw" => {
                add_drawn(&mut matches, cmd[0]);
                add_drawn(&mut matches, cmd[1])
            }
            "loss" => {
                add_loser(&mut matches, cmd[0]);
                add_winer(&mut matches, cmd[1])
            }
            _ => (),
        };
    }

    let mut teams = matches.iter().collect::<Vec<_>>();

    teams.sort_by(|&(ateam, ascore), &(bteam, bscore)| {
        let ascores = ascore.scores();
        let bscores = bscore.scores();
        let r = bscores.cmp(&ascores);
        match r {
            Ordering::Equal => ateam.cmp(bteam),
            _ => r,
        }
    });

    let mut result = "Team                           | MP |  W |  D |  L |  P".to_string();
    for (team, score) in teams {
        result.push('\n');

        result += &format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            team,
            score.matches(),
            score.win,
            score.draw,
            score.lose,
            score.scores()
        );
    }

    result
}
