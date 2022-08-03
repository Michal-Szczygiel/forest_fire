use crate::{configuration::Configuration, simulation::Cell};

/// Funkcja rysująca krok symulacji do bufora klatki.
#[inline]
pub fn draw(config: &Configuration, state: &[Cell], frame_buffer: &mut [u8]) {
    let cells_x = (config.resolution.0 / config.cell_size) as usize + 2;

    frame_buffer
        .chunks_mut(config.resolution.0 as usize * 3)
        .enumerate()
        .for_each(|(index_y, chunk)| {
            let mut cell_index_x: usize = 0;
            let mut cell_index_y: usize = 0;

            chunk
                .chunks_mut(3)
                .enumerate()
                .for_each(|(index_x, pixel)| {
                    cell_index_x = index_x / config.cell_size as usize;
                    cell_index_y = index_y / config.cell_size as usize;

                    match state[(cell_index_y + 1) * cells_x + cell_index_x + 1] {
                        // Jasność komórki żywej jest zależna od rozmiaru drzewa
                        Cell::Alive { size, color } => unsafe {
                            *pixel.get_unchecked_mut(0) = (color.0 as f32 * size) as u8;
                            *pixel.get_unchecked_mut(1) = (color.1 as f32 * size) as u8;
                            *pixel.get_unchecked_mut(2) = (color.2 as f32 * size) as u8;
                        },
                        // Jasność płonącego drzewa jest zależna od postępu spalania
                        Cell::OnFire { progress } => unsafe {
                            *pixel.get_unchecked_mut(0) =
                                (config.fire_color.0 as f32 * progress.max(0.3)) as u8;
                            *pixel.get_unchecked_mut(1) =
                                (config.fire_color.1 as f32 * progress.max(0.3)) as u8;
                            *pixel.get_unchecked_mut(2) =
                                (config.fire_color.2 as f32 * progress.max(0.3)) as u8;
                        },
                        // Komórka martwa ma kolor podłoża
                        Cell::Dead => unsafe {
                            *pixel.get_unchecked_mut(0) = config.ground_color.0;
                            *pixel.get_unchecked_mut(1) = config.ground_color.1;
                            *pixel.get_unchecked_mut(2) = config.ground_color.2;
                        },
                    }
                })
        });
}
