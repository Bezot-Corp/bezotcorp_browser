use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::window::Window;

use crate::browser::chrome::TOOLBAR_HEIGHT;
use crate::browser::render::{RenderCommand, RenderTree};
use crate::browser::state::{BrowserLoadingState, BrowserToolbarState};

pub(crate) struct ChromeRenderer {
    _context: Context<Rc<Window>>,
    surface: Surface<Rc<Window>, Rc<Window>>,
}

impl ChromeRenderer {
    pub(crate) fn new(window: Rc<Window>) -> Result<Self, softbuffer::SoftBufferError> {
        let context = Context::new(window.clone())?;
        let surface = Surface::new(&context, window)?;

        Ok(Self {
            _context: context,
            surface,
        })
    }

    pub(crate) fn render(
        &mut self,
        width: u32,
        height: u32,
        toolbar_state: &BrowserToolbarState,
        loading_state: BrowserLoadingState,
    ) -> Result<(), softbuffer::SoftBufferError> {
        let Some(width) = NonZeroU32::new(width) else {
            return Ok(());
        };
        let Some(height) = NonZeroU32::new(height) else {
            return Ok(());
        };

        self.surface.resize(width, height)?;

        let mut buffer = self.surface.buffer_mut()?;
        let width = width.get() as usize;
        let height = height.get() as usize;
        let toolbar_height = TOOLBAR_HEIGHT.min(height as u32) as usize;

        clear(&mut buffer, 0x000000);
        draw_toolbar(
            buffer.as_mut(),
            width,
            toolbar_height,
            toolbar_state,
            loading_state,
        );

        buffer.present()?;

        Ok(())
    }

    pub(crate) fn render_bezot(
        &mut self,
        width: u32,
        height: u32,
        toolbar_state: &BrowserToolbarState,
        loading_state: BrowserLoadingState,
        render_tree: &RenderTree,
    ) -> Result<(), softbuffer::SoftBufferError> {
        let Some(width) = NonZeroU32::new(width) else {
            return Ok(());
        };
        let Some(height) = NonZeroU32::new(height) else {
            return Ok(());
        };

        self.surface.resize(width, height)?;

        let mut buffer = self.surface.buffer_mut()?;
        let width = width.get() as usize;
        let height = height.get() as usize;
        let toolbar_height = TOOLBAR_HEIGHT.min(height as u32) as usize;

        clear(&mut buffer, 0x101014);
        draw_bezot_commands(&mut buffer, width, height, toolbar_height, render_tree);
        draw_toolbar(
            buffer.as_mut(),
            width,
            toolbar_height,
            toolbar_state,
            loading_state,
        );

        buffer.present()?;

        Ok(())
    }
}

fn clear(buffer: &mut [u32], color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

fn draw_toolbar(
    buffer: &mut [u32],
    width: usize,
    toolbar_height: usize,
    toolbar_state: &BrowserToolbarState,
    loading_state: BrowserLoadingState,
) {
    draw_toolbar_background(buffer, width, toolbar_height);

    let accent = if loading_state == BrowserLoadingState::Loading {
        0x00AACC
    } else {
        0x606060
    };

    draw_toolbar_accent(buffer, width, toolbar_height, accent);
    draw_button(buffer, width, 8, toolbar_state.can_go_back());
    draw_button(buffer, width, 44, toolbar_state.can_go_forward());
    draw_button(buffer, width, 80, true);
    draw_address_bar(buffer, width, toolbar_state.address_input_active());
}

fn draw_bezot_commands(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    toolbar_height: usize,
    render_tree: &RenderTree,
) {
    for command in &render_tree.commands {
        match command {
            RenderCommand::Clear { r, g, b, .. } => {
                let color = rgb(*r, *g, *b);

                for y in toolbar_height..height {
                    for x in 0..width {
                        let index = y * width + x;
                        if index < buffer.len() {
                            buffer[index] = color;
                        }
                    }
                }
            }
            RenderCommand::Text { x, y, value } => {
                draw_fake_text(buffer, width, height, *x as usize, *y as usize, value);
            }
        }
    }
}

fn draw_fake_text(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    value: &str,
) {
    let text_width = value
        .len()
        .saturating_mul(8)
        .min(width.saturating_sub(x + 16));
    let text_height = 14;

    for py in y..(y + text_height).min(height) {
        for px in x..(x + text_width).min(width) {
            let index = py * width + px;
            if index < buffer.len() {
                buffer[index] = 0xD8D8D8;
            }
        }
    }
}

fn draw_toolbar_background(buffer: &mut [u32], width: usize, toolbar_height: usize) {
    for y in 0..toolbar_height {
        for x in 0..width {
            let index = y * width + x;
            if index < buffer.len() {
                buffer[index] = 0x202020;
            }
        }
    }
}

fn draw_toolbar_accent(buffer: &mut [u32], width: usize, toolbar_height: usize, color: u32) {
    for y in toolbar_height.saturating_sub(3)..toolbar_height {
        for x in 0..width {
            let index = y * width + x;
            if index < buffer.len() {
                buffer[index] = color;
            }
        }
    }
}

fn draw_button(buffer: &mut [u32], width: usize, x: usize, enabled: bool) {
    let color = if enabled { 0x404040 } else { 0x2A2A2A };

    for y in 10..38 {
        for px in x..x + 28 {
            let index = y * width + px;
            if index < buffer.len() {
                buffer[index] = color;
            }
        }
    }
}

fn draw_address_bar(buffer: &mut [u32], width: usize, active: bool) {
    let color = if active { 0x303850 } else { 0x303030 };

    for y in 10..38 {
        for x in 120..width.saturating_sub(16) {
            let index = y * width + x;
            if index < buffer.len() {
                buffer[index] = color;
            }
        }
    }
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}
