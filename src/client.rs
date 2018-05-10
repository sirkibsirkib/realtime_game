use ggez::{
    Context,
    GameResult,
    conf,
    graphics::{
        self,
        Color,
        DrawMode,
        Point2,
        Mesh,
        spritebatch::SpriteBatch,
    },
    event::{
        self,
        Keycode,
        Mod,
    },
};
use ::game::*;
use ::game::room::*;
use fnv::FnvHashMap;
use std::env;
use std::path;

pub fn client_go(room: Room) {
    let c = conf::Conf::new();
    let mut ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }
    let mut client_state = ClientState::new(&mut ctx, room);
    event::run(ctx, &mut client_state).unwrap();
}

struct ClientState {
    room: Room,
    image_manager: ImageManager,
    batch: SpriteBatch,
}

impl ClientState {
    pub fn new(ctx: &mut Context, room: Room) -> Self {
        let mut image_manager = ImageManager::new();
        let image = image_manager.get(ctx, ImageVariants::Rock);
        let mut batch = graphics::spritebatch::SpriteBatch::new(image);
        for i in 0..5 {
            let param = graphics::DrawParam {
                dest: graphics::Point2::new(
                    i as f32 * 32.0,
                    100.0,
                ),
                scale: graphics::Point2::new(0.5, 0.5),
                ..Default::default()
            };
            batch.add(param);
        }
        Self {
            image_manager: image_manager,
            batch: batch,
            room: room,
        }
    }
}


impl event::EventHandler for ClientState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }


    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (),
        }
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let param = graphics::DrawParam {
            ..Default::default()
        };
        graphics::draw_ex(ctx, &self.batch, param)?;
        graphics::present(ctx);
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum ImageVariants {
    Rock,
}

struct ImageManager {
    map: FnvHashMap<ImageVariants, graphics::Image>
}
impl ImageManager {
    pub fn new() -> Self {
        Self {
            map: FnvHashMap::default(),
        }
    }

    pub fn get(&mut self, ctx: &mut Context, variant: ImageVariants) -> graphics::Image {
        if !self.map.contains_key(&variant) {
            use self::ImageVariants::*;
            let path = match variant {
                Rock => "/rock.png",
            };
            let image = graphics::Image::new(ctx, path).unwrap();
            self.map.insert(variant, image);
        }
        self.map.get(&variant).unwrap().clone()
    }
}