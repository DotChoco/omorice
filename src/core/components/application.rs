use std::str::FromStr;

use crate::core::enums::build_mode::BuildMode;

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub struct Application{
  path: String,
  assets_path: String,
  export_path: String,
  build_mode: BuildMode,
}

impl Application {
  pub fn new() -> Application{
    Application{
      path:String::new(),
      assets_path:String::new(),
      export_path:String::new(),
      build_mode:BuildMode::NONE,
    }
  }

  //Getters and Setters
  pub fn get_path(&self)->&str{
    &self.path
  }

  pub fn get_assets_path(&self)->&str{
    &self.assets_path
  }

  pub fn get_build_mode(&self)->BuildMode{
    self.build_mode
  }

  pub fn set_build_mode(&mut self, mode:BuildMode){
    self.build_mode = mode;
    self.export_assets();
  }


  pub fn export_assets(&mut self){

  }

}

