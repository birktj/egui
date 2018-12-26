use crate::{math::*, types::*};

/// TODO: a Style struct which defines colors etc
fn translate_cmd(out_commands: &mut Vec<PaintCmd>, cmd: GuiCmd) {
    match cmd {
        GuiCmd::PaintCommands(mut commands) => out_commands.append(&mut commands),
        GuiCmd::Rect {
            rect,
            style,
            interact,
        } => match style {
            RectStyle::Button => {
                let fill_style = if interact.active {
                    "#888888ff".to_string()
                } else if interact.hovered {
                    "#666666ff".to_string()
                } else {
                    "#444444ff".to_string()
                };
                out_commands.push(PaintCmd::Rect {
                    corner_radius: 5.0,
                    fill_style: Some(fill_style),
                    outline: None,
                    pos: rect.pos,
                    size: rect.size,
                });
            }
        },
        GuiCmd::Slider {
            interact,
            label,
            max,
            min,
            rect,
            value,
        } => {
            let thin_rect = Rect::from_center_size(rect.center(), vec2(rect.size.x, 8.0));

            let marker_center_x = remap_clamp(value, min, max, rect.min().x, rect.max().x);

            let marker_rect =
                Rect::from_center_size(vec2(marker_center_x, rect.center().y), vec2(16.0, 16.0));

            let marker_fill_style = if interact.active {
                "#888888ff".to_string()
            } else if interact.hovered {
                "#666666ff".to_string()
            } else {
                "#444444ff".to_string()
            };

            out_commands.push(PaintCmd::Rect {
                corner_radius: 2.0,
                fill_style: Some("#222222ff".to_string()),
                outline: None,
                pos: thin_rect.pos,
                size: thin_rect.size,
            });

            out_commands.push(PaintCmd::Rect {
                corner_radius: 3.0,
                fill_style: Some(marker_fill_style),
                outline: None,
                pos: marker_rect.pos,
                size: marker_rect.size,
            });

            out_commands.push(PaintCmd::Text {
                fill_style: "#ffffffbb".to_string(),
                font: "14px Palatino".to_string(),
                pos: rect.min(),
                text: format!("{}: {:.3}", label, value),
                text_align: TextAlign::Start,
            });
        }
        GuiCmd::Text {
            pos,
            text,
            text_align,
            style,
        } => {
            let fill_style = match style {
                TextStyle::Button => "#ffffffbb".to_string(),
                TextStyle::Label => "#ffffffbb".to_string(),
            };
            out_commands.push(PaintCmd::Text {
                fill_style,
                font: "14px Palatino".to_string(),
                pos,
                text,
                text_align,
            });
        }
    }
}

pub fn into_paint_commands(gui_commands: &[GuiCmd]) -> Vec<PaintCmd> {
    let mut paint_commands = vec![];
    for gui_cmd in gui_commands {
        translate_cmd(&mut paint_commands, gui_cmd.clone())
    }
    paint_commands
}
