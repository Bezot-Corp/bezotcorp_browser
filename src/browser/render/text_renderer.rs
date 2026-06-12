use parley::{
    Alignment, AlignmentOptions, FontContext, Layout, LayoutContext, PositionedLayoutItem,
    StyleProperty,
};
use vello::Scene;
use vello::kurbo::Affine;
use vello::peniko::{Color, Fill};
use vello_encoding::Glyph;

pub(crate) struct TextRenderer {
    font_cx: FontContext,
    layout_cx: LayoutContext<()>,
}

impl TextRenderer {
    pub(crate) fn new() -> Self {
        Self {
            font_cx: FontContext::new(),
            layout_cx: LayoutContext::new(),
        }
    }

    pub(crate) fn draw(
        &mut self,
        scene: &mut Scene,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        max_width: Option<f32>,
        color: Color,
    ) {
        if text.is_empty() {
            return;
        }

        let mut builder = self
            .layout_cx
            .ranged_builder(&mut self.font_cx, text, 1.0, true);
        builder.push_default(StyleProperty::FontSize(font_size));

        let mut layout: Layout<()> = builder.build(text);
        layout.break_all_lines(max_width);
        layout.align(Alignment::Start, AlignmentOptions::default());

        let transform = Affine::translate((x as f64, y as f64));

        for line in layout.lines() {
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                    continue;
                };

                let run = glyph_run.run();
                let font = run.font();
                let size = run.font_size();

                let glyphs: Vec<Glyph> = glyph_run
                    .positioned_glyphs()
                    .map(|g| Glyph {
                        id: g.id,
                        x: g.x,
                        y: g.y,
                    })
                    .collect();

                if glyphs.is_empty() {
                    continue;
                }

                scene
                    .draw_glyphs(font)
                    .font_size(size)
                    .brush(color)
                    .transform(transform)
                    .draw(Fill::NonZero, glyphs.into_iter());
            }
        }
    }
}
