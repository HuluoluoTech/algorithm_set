// update list of entities.

pub trait Entity : AoiRadius {  
    fn step_on(&mut self, steps: u32);
    fn get_postion(&self) -> (u32, u32);
}

pub trait AoiRadius {
    // y 方向半径 、 x 方向 半径
    // --------------------------
    // |                        |
    // |                        |
    // |                        |
    // |                        |
    // |         self           | 
    // |                        |
    // |                        |
    // |                        |
    // |                        |
    // --------------------------
    fn radius(&mut self) -> (u32, u32);
    fn check(&self, aoi: &Box<dyn Entity>) -> bool;
}

enum RoleType {
    Player,
    Npc,
    Static,
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UNKNOWN,
}

#[derive(Debug, Default)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Velocity {
    direction: Direction,
    value: f32,
}
impl Velocity {
    pub fn new() -> Self {
        Velocity {
            direction: Direction::UP,
            value: 0_f32,
        }
    }
}

struct Aoi {
    pub in_my_area: Vec<Box<dyn Entity>>,
    pub me_in_area: Vec<Box<dyn Entity>>,
}
impl Aoi {
    pub fn new() -> Self {
        Aoi { in_my_area: vec![], me_in_area: vec![] }
    }
}

struct Player {
    v: Velocity, 
    p: Position,
    aoi: Aoi,
}
impl Player {
    pub fn new() -> Self {
        Player { v: Velocity::new() , p: Position::default(), aoi: Aoi::new() }
    }
}

impl Entity for Player {
    fn step_on(&mut self, steps: u32) {
        // 默认方向走 steps 步。
        // 假如只往x方向走。。。 #TODO

        self.p.x += steps;
    }

    fn get_postion(&self) -> (u32, u32) {
        (self.p.x, self.p.y)
    }
}

impl AoiRadius for Player {
    fn radius(&mut self) -> (u32, u32) {
        (10_u32, 10_u32)
    }

    fn check(&self, aoi: &Box<dyn Entity>) -> bool {
        let player = aoi.get_postion();
        // 判断当前player是否在以 self 为中心的area 内
        // TODO

        true
    }
}

struct SceneManager {
    objects: Vec<Box<dyn Entity>>,
}
impl SceneManager {
    // 场景循环
    pub fn r#loop(&mut self) {
        self.update_aoi();
    }

    pub fn update_aoi(&mut self) {
        let mut index = 0;
        let mut len = self.objects.len();
        for index in 0..len {
            let mut obj = &self.objects[index];

            for index2 in index+1..len {
                let obj2 = &self.objects[index2];
                let ret = obj.check(obj2);
                if ret {
                    // 更新 aoi list
                    // #TODO
                }
            }   
        }
    }
}
impl AoiRadius for SceneManager {
    fn radius(&mut self) -> (u32, u32) {
        (100, 100)
    }

    fn check(&self, _aoi: &Box<dyn Entity>) -> bool {
        false
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
