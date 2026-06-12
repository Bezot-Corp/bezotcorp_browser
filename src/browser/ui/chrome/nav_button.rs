use vello::kurbo::{BezPath, Point};

use crate::browser::chrome::ChromeLayout;
use crate::browser::ui::component::{Component, RenderContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavButtonKind {
    Back,
    Forward,
    Reload,
    Stop,
}

pub(crate) struct NavButton {
    pub(crate) kind: NavButtonKind,
    pub(crate) enabled: bool,
    pub(crate) x: f64,
}

impl Component for NavButton {
    fn render(&self, cx: &mut RenderContext<'_>) {
        let bg = if self.enabled {
            cx.theme.button_on()
        } else {
            cx.theme.button_off()
        };
        let icon = if self.enabled {
            cx.theme.icon_on()
        } else {
            cx.theme.icon_off()
        };
        cx.fill(&ChromeLayout::button_rect(self.x), bg);
        let (cx_pos, cy_pos) = ChromeLayout::button_center(self.x);
        match self.kind {
            NavButtonKind::Back => Self::draw_arrow_left(cx, cx_pos, cy_pos, icon),
            NavButtonKind::Forward => Self::draw_arrow_right(cx, cx_pos, cy_pos, icon),
            NavButtonKind::Reload => Self::draw_reload(cx, cx_pos, cy_pos, icon),
            NavButtonKind::Stop => Self::draw_stop(cx, cx_pos, cy_pos, icon),
        }
    }
}

impl NavButton {
    fn draw_arrow_left(
        cx: &mut RenderContext<'_>,
        cx_pos: f64,
        cy_pos: f64,
        color: vello::peniko::Color,
    ) {
        let mut p = BezPath::new();
        p.move_to(Point::new(cx_pos + 5.0, cy_pos - 6.0));
        p.line_to(Point::new(cx_pos - 4.0, cy_pos));
        p.line_to(Point::new(cx_pos + 5.0, cy_pos + 6.0));
        cx.stroke(&p, 2.0, color);
    }

    fn draw_arrow_right(
        cx: &mut RenderContext<'_>,
        cx_pos: f64,
        cy_pos: f64,
        color: vello::peniko::Color,
    ) {
        let mut p = BezPath::new();
        p.move_to(Point::new(cx_pos - 5.0, cy_pos - 6.0));
        p.line_to(Point::new(cx_pos + 4.0, cy_pos));
        p.line_to(Point::new(cx_pos - 5.0, cy_pos + 6.0));
        cx.stroke(&p, 2.0, color);
    }

    fn draw_reload(
        cx: &mut RenderContext<'_>,
        cx_pos: f64,
        cy_pos: f64,
        color: vello::peniko::Color,
    ) {
        let mut p = BezPath::new();
        p.move_to(Point::new(cx_pos + 6.0, cy_pos - 2.0));
        p.line_to(Point::new(cx_pos + 6.0, cy_pos - 6.0));
        p.line_to(Point::new(cx_pos, cy_pos - 6.0));
        p.line_to(Point::new(cx_pos - 5.0, cy_pos - 3.0));
        p.line_to(Point::new(cx_pos - 6.0, cy_pos));
        p.line_to(Point::new(cx_pos - 5.0, cy_pos + 3.0));
        p.line_to(Point::new(cx_pos, cy_pos + 6.0));
        p.line_to(Point::new(cx_pos + 5.0, cy_pos + 3.0));
        cx.stroke(&p, 2.0, color);
        let mut a = BezPath::new();
        a.move_to(Point::new(cx_pos + 3.0, cy_pos - 8.0));
        a.line_to(Point::new(cx_pos + 6.0, cy_pos - 2.0));
        a.line_to(Point::new(cx_pos + 9.0, cy_pos - 6.0));
        cx.stroke(&a, 2.0, color);
    }

    fn draw_stop(
        cx: &mut RenderContext<'_>,
        cx_pos: f64,
        cy_pos: f64,
        color: vello::peniko::Color,
    ) {
        cx.stroke_line(
            cx_pos - 5.0,
            cy_pos - 5.0,
            cx_pos + 5.0,
            cy_pos + 5.0,
            2.0,
            color,
        );
        cx.stroke_line(
            cx_pos + 5.0,
            cy_pos - 5.0,
            cx_pos - 5.0,
            cy_pos + 5.0,
            2.0,
            color,
        );
    }
}
