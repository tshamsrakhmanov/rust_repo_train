use crossterm::{
    cursor,
    event::{Event, KeyCode, poll, read},
    execute,
    style::{self, Stylize},
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use nalgebra::{self, Vector3};
use std::cmp;
use std::{
    io::{Cursor, Write, stdout},
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    let screen_size = terminal::size()?;

    let p0: Vector3<u16> = Vector3::new(10, 10, 0);
    let p1: Vector3<u16> = Vector3::new(20, 10, 0);
    let p2: Vector3<u16> = Vector3::new(10, 20, 0);
    let p3: Vector3<u16> = Vector3::new(20, 20, 0);

    let tr1 = TriangleV3::new(p0, p1, p2);
    let tr2 = TriangleV3::new(p1, p3, p2);

    execute!(stdout, cursor::Hide)?;
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    'main_loop: loop {
        for pos in tr1.rasterize() {
            execute!(stdout, cursor::MoveTo(pos.0, pos.1))?;
            execute!(stdout, style::PrintStyledContent("O".yellow()))?;
        }

        for pos in tr2.rasterize() {
            execute!(stdout, cursor::MoveTo(pos.0, pos.1))?;
            execute!(stdout, style::PrintStyledContent("O".blue()))?;
        }
        stdout.flush()?;

        if poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    }
    disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    execute!(stdout, LeaveAlternateScreen)
}

struct TriangleV3 {
    point0: Vector3<u16>,
    point1: Vector3<u16>,
    point2: Vector3<u16>,
}
impl TriangleV3 {
    fn rasterize(&self) -> Vec<(u16, u16)> {
        let mut answer: Vec<(u16, u16)> = Vec::new();

        // make edge vectors
        let edge_0 = Vector3::new(
            self.point1.x as i16 - self.point0.x as i16,
            self.point1.y as i16 - self.point0.y as i16,
            0,
        );

        let edge_1 = Vector3::new(
            self.point2.x as i16 - self.point1.x as i16,
            self.point2.y as i16 - self.point1.y as i16,
            0,
        );

        let edge_2 = Vector3::new(
            self.point0.x as i16 - self.point2.x as i16,
            self.point0.y as i16 - self.point2.y as i16,
            0,
        );

        // make region of intereset - the boundaries of triabnle
        let min_x = cmp::min(cmp::min(self.point0.x, self.point1.x), self.point2.x);
        let max_x = cmp::max(cmp::max(self.point0.x, self.point1.x), self.point2.x);
        let min_y = cmp::min(cmp::min(self.point0.y, self.point1.y), self.point2.y);
        let max_y = cmp::max(cmp::max(self.point0.y, self.point1.y), self.point2.y);

        // loop through region of intereset and check if points is compatible with conditions
        for x in min_x..max_x {
            for y in min_y..max_y {
                // make local point vector according to eges
                let edge_0_p = Vector3::new(
                    x as i16 - self.point0.x as i16,
                    y as i16 - self.point0.y as i16,
                    0,
                );
                let edge_1_p = Vector3::new(
                    x as i16 - self.point1.x as i16,
                    y as i16 - self.point1.y as i16,
                    0,
                );
                let edge_2_p = Vector3::new(
                    x as i16 - self.point2.x as i16,
                    y as i16 - self.point2.y as i16,
                    0,
                );
                let edge_0_cross = edge_0.cross(&edge_0_p);
                let edge_1_cross = edge_1.cross(&edge_1_p);
                let edge_2_cross = edge_2.cross(&edge_2_p);
                let w0 = edge_0_cross.z;
                let w1 = edge_1_cross.z;
                let w2 = edge_2_cross.z;
                if w0 > 0 && w1 > 0 && w2 > 0 {
                    answer.push((x as u16, y as u16));
                }
            }
        }

        answer
    }
    fn new(point0: Vector3<u16>, point1: Vector3<u16>, point2: Vector3<u16>) -> TriangleV3 {
        let a = TriangleV3 {
            point0: point0,
            point1: point1,
            point2: point2,
        };
        a
    }
}
