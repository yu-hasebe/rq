use crate::{browser::new_image, sprite::Sprite};

use super::Rect;

use anyhow::{anyhow, Result};
use web_sys::CanvasRenderingContext2d;

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
}
impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self.context
            .clear_rect(rect.x.into(), rect.y.into(), rect.w.into(), rect.h.into());
    }

    pub fn render<S: Sprite>(&self, sprite: S) -> Result<()> {
        // TODO 画像読み込みを減らそう
        let image = new_image()?;
        let src = format!("{}.png", sprite.source_image());
        image.set_src(&src);
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                sprite.frame().x.into(),
                sprite.frame().y.into(),
                sprite.frame().w.into(),
                sprite.frame().h.into(),
                sprite.destination().x.into(),
                sprite.destination().y.into(),
                sprite.destination().w.into(),
                sprite.destination().h.into(),
            )
            .map_err(|err| anyhow!("error drawing image: {:#?}", err))
    }
}
