use super::{Point, Rect, SpriteSheet};

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

    pub fn draw_image(
        &self,
        sprite_sheet: &SpriteSheet,
        frame_name: &str,
        destination: &Point,
    ) -> Result<()> {
        let sheet = sprite_sheet
            .sheet
            .as_ref()
            .ok_or_else(|| anyhow!("error getting SpriteSheet"))?;
        let cell = sheet
            .frames
            .get(frame_name)
            .ok_or_else(|| anyhow!("invalid frame_name: {}", frame_name))?;

        let image = sprite_sheet
            .image
            .as_ref()
            .ok_or_else(|| anyhow!("error getting HtmlImageElement"))?;
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                cell.frame.x.into(),
                cell.frame.y.into(),
                cell.frame.w.into(),
                cell.frame.h.into(),
                destination.x.into(),
                destination.y.into(),
                cell.frame.w.into(),
                cell.frame.h.into(),
            )
            .map_err(|err| anyhow!("error drawing image: {:#?}", err))?;
        Ok(())
    }
}
