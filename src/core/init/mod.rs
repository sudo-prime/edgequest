//! 
//! Initialize tcod elements.
//! 
//! Depends on the `config` module.
//!
//! NOTE: Maybe change this so we're only loading config once?
//! 

// We need tcod::Console to keep our consoles in scope
#[allow(unused_imports)]
use core::tcod::{Console, console};

pub mod config;

mod init_tests;

///
/// Initialize the root console.
/// 
/// Returns a console that is meant to be used as the root console. Console
/// settings depend on `config::load()`.
/// 
pub fn root() -> console::Root {

  match config::load("config/cfg.yml") {
    Ok(cfg) => {
      
      // Match fonttype based on the FontType enum
      let fonttype = match cfg.fonttype.as_str() {
        "Default" => console::FontType::Default,
        "Greyscale" => console::FontType::Greyscale,
        _ => panic!("Bad font type: {}", cfg.fonttype)
      };

      // Match fontlayout based on the FontLayout enum
      let fontlayout = match cfg.fontlayout.as_str() {
        "Tcod" => console::FontLayout::Tcod,
        "AsciiInRow" => console::FontLayout::AsciiInRow,
        "AsciiInCol" => console::FontLayout::AsciiInCol,
        _ => panic!("Bad font type: {}", cfg.fontlayout)
      };

      // Match renderer based on the Renderer enum
      let renderer = match cfg.renderer.as_str() {
        "SDL" => console::Renderer::SDL,
        "GLSL" => console::Renderer::GLSL,
        "OpenGL" => console::Renderer::OpenGL,
        _ => panic!("Bad font type: {}", cfg.renderer)
      };

      return console::Root::initializer()
        .size(cfg.screen_width as i32, cfg.screen_height as i32)
        .title("EQ")
        .fullscreen(cfg.fullscreen)
        .font(cfg.fontpath, fontlayout)
        .font_type(fonttype)
        .renderer(renderer)
        .init();

    },
    Err(e) => panic!("Error parsing config.yml! {:?}", e)
  }

}

///
/// Get map dimensions as a `Pos`
/// 
pub fn map_dimensions() -> (isize, isize) {
  
  match config::load("config/cfg.yml") {
    Ok(cfg) => return (cfg.map_width, cfg.map_height),
    Err(e) => panic!("Error parsing config.yml! {:?}", e)
  }

}

///
/// Get console height
///
pub fn console_height() -> isize {

  match config::load("config/cfg.yml") {
    Ok(cfg) => return cfg.console_height,
    Err(e) => panic!("Error parsing config.yml! {:?}", e)
  }
  
}

///
/// Get panel width
///
pub fn panel_width() -> isize {

  match config::load("config/cfg.yml") {
    Ok(cfg) => return cfg.panel_width,
    Err(e) => panic!("Error parsing config.yml! {:?}", e)
  }
  
}