pub struct DrawCommand();

impl DrawCommand {
    pub fn fill_screen(cpu: &mut super::cpu::MicroCVMCpu, color: super::types::Color) {
        for i in 0..cpu.video_memory.len() {
            cpu.video_memory[i].r = color.r;
            cpu.video_memory[i].g = color.g;
            cpu.video_memory[i].b = color.b;
        }
    }

    pub fn clear_screen(cpu: &mut super::cpu::MicroCVMCpu) {
        cpu.video_memory.fill(super::types::Color::new(0, 0, 0));
    }

    pub fn get_index_from_coordinate(coordinate: super::types::Point, width: isize) -> isize {
        coordinate.y * (width as isize) + coordinate.x
    }

    pub fn draw_line(
        cpu: &mut super::cpu::MicroCVMCpu,
        color: super::types::Color,
        line_start: super::types::Point,
        line_end: super::types::Point,
        thickness: isize,
    ) {
        let width = cpu.framebuffer_width / 2;
        let height = cpu.framebuffer_height / 2;

        let mut x0 = line_start.x;
        let mut y0 = line_start.y;

        let x1 = line_end.x;
        let y1 = line_end.y;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let radius = thickness as f32 / 2.0;
        let radius_squared = radius * radius;

        loop {
            for offset_y in -thickness..=thickness {
                for offset_x in -thickness..=thickness {
                    if (offset_x * offset_x + offset_y * offset_y) as f32 <= radius_squared {
                        let tx = x0 + offset_x;
                        let ty = y0 + offset_y;

                        if tx >= 0 && tx < width as isize && ty >= 0 && ty < height as isize {
                            let index = Self::get_index_from_coordinate(
                                super::types::Point::new(tx, ty),
                                width as isize,
                            );
                            cpu.video_memory[index as usize] = color;
                        }
                    }
                }
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}
