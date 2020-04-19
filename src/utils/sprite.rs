use piston_window::*;
use sprite::*;
use std::rc::Rc;

///Load a sprite
pub fn load_sprite(factory: &mut gfx_device_gl::Factory, path: &str) 
	-> Sprite<gfx_texture::Texture<gfx_device_gl::Resources>>
{
    let mut tex_ctx = TextureContext {
        factory: factory.clone(),
        encoder: factory.create_command_buffer().into()
    };
	let texture = Rc::new(Texture::from_path(
		&mut tex_ctx,
		path,
		Flip::None,
		&TextureSettings::new()
	).unwrap());
	Sprite::from_texture(texture.clone())
}