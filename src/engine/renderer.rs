use super::Rect;

use anyhow::{anyhow, Result};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

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
        image: &HtmlImageElement,
        frame: &Rect,
        destination: &Rect,
    ) -> Result<()> {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                frame.x.into(),
                frame.y.into(),
                frame.w.into(),
                frame.h.into(),
                destination.x.into(),
                destination.y.into(),
                destination.w.into(),
                destination.h.into(),
            )
            .map_err(|err| anyhow!("error drawing image: {:#?}", err))?;
        Ok(())
    }
}
