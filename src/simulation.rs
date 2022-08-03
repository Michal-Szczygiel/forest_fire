use crate::{configuration::Configuration, visualization::draw};

use console::{style, Term};
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
    seq::SliceRandom,
    thread_rng,
};

use image::{
    codecs::gif::{GifEncoder, Repeat},
    ColorType,
};

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Bazowy rozmiar drzewa.
const BASE_TREE_SIZE: f32 = 0.2;

/// Reprezentacja komórki.
pub enum Cell {
    Alive { size: f32, color: (u8, u8, u8) },
    OnFire { progress: f32 },
    Dead,
}

/// Losowa inicjalizacja planszy.
#[inline]
pub fn initialize_grid(rng: &mut ThreadRng, config: &Configuration) -> Vec<Cell> {
    let cells_x = config.resolution.0 / config.cell_size;
    let cells_y = config.resolution.1 / config.cell_size;

    let flat_distr = Uniform::new(0.0, 1.0);
    let distr_size = Uniform::new(BASE_TREE_SIZE, 1.0);

    (0..(cells_x + 2) * (cells_y + 2))
        .map(|_| {
            if flat_distr.sample(rng) <= config.alive_fraction {
                Cell::Alive {
                    size: distr_size.sample(rng),
                    color: config.forest_color_palette.choose(rng).unwrap().clone(),
                }
            } else {
                Cell::Dead
            }
        })
        .collect()
}

/// Domyślna inicjalizacja planszy.
#[inline]
pub fn initialize_grid_default(config: &Configuration) -> Vec<Cell> {
    let cells_x = config.resolution.0 / config.cell_size;
    let cells_y = config.resolution.1 / config.cell_size;

    (0..(cells_x + 2) * (cells_y + 2))
        .map(|_| Cell::Dead)
        .collect()
}

/// Generowania nowego stanu (kolejny krok symulacji).
#[inline]
pub fn generate_current_state(
    rng: &mut ThreadRng,
    config: &Configuration,
    previous_state: &Vec<Cell>,
    current_state: &mut Vec<Cell>,
) {
    let cells_x = (config.resolution.0 / config.cell_size) as usize + 2;
    let cells_y = (config.resolution.1 / config.cell_size) as usize + 2;

    let flat_distr = Uniform::new(0.0, 1.0);

    for index_y in 1..cells_y - 1 {
        for index_x in 1..cells_x - 1 {
            match previous_state[index_y * cells_x + index_x] {
                // Rozwiązanie dla komórki żywej
                Cell::Alive { size, color } => {
                    let mut trees_on_fire = 0;

                    // Sprawdzenie z iloma płonącymi drzewami sąsiaduje
                    match previous_state[(index_y - 1) * cells_x + (index_x - 1)] {
                        Cell::OnFire { .. } => trees_on_fire += 1,
                        _ => {}
                    }

                    match previous_state[(index_y - 1) * cells_x + index_x] {
                        Cell::OnFire { .. } => trees_on_fire += 1,
                        _ => {}
                    }

                    match previous_state[(index_y - 1) * cells_x + (index_x + 1)] {
                        Cell::OnFire { .. } => trees_on_fire += 1,
                        _ => {}
                    }

                    match previous_state[index_y * cells_x + (index_x - 1)] {
                        Cell::OnFire { .. } => trees_on_fire += 1,
                        _ => {}
                    }

                    match previous_state[index_y * cells_x + (index_x + 1)] {
                        Cell::OnFire { .. } => trees_on_fire += 1,
                        _ => {}
                    }

                    match previous_state[(index_y + 1) * cells_x + (index_x - 1)] {
                        Cell::OnFire { .. } => trees_on_fire += 1,
                        _ => {}
                    }

                    match previous_state[(index_y + 1) * cells_x + index_x] {
                        Cell::OnFire { .. } => trees_on_fire += 1,
                        _ => {}
                    }

                    match previous_state[(index_y + 1) * cells_x + (index_x + 1)] {
                        Cell::OnFire { .. } => trees_on_fire += 1,
                        _ => {}
                    }

                    // Sprawdzenie czy drzewo stanie w ogniu, proces zależny od wielkości drzewa
                    if flat_distr.sample(rng)
                        < (1.0 - (1.0 - config.inflammability).powi(trees_on_fire)) * size
                    {
                        current_state[index_y * cells_x + index_x] = Cell::OnFire { progress: 1.0 };
                    // Sprawdzenie czy drzewo dokona samozapłonu, proces zależny od wielkości drzewa
                    } else if flat_distr.sample(rng) < config.self_ignition_probability * size {
                        current_state[index_y * cells_x + index_x] = Cell::OnFire { progress: 1.0 };
                    // Jeżeli nie nastąpi nic z powyższych, drzewo rośnie
                    } else {
                        current_state[index_y * cells_x + index_x] = Cell::Alive {
                            size: 1.0_f32.min(size + config.growth_rate),
                            color,
                        };
                    }
                }
                // Rozwiązanie dla płonącego drzewa
                Cell::OnFire { progress } => {
                    let new_progress = progress - config.burning_rate;

                    // Jeżeli drzewo nie spłonęło, to płonie dalej
                    if new_progress > 0.0 {
                        current_state[index_y * cells_x + index_x] = Cell::OnFire {
                            progress: new_progress,
                        };
                    // Jeżeli drzewo spłonęło to jest martwe (pusta komórka)
                    } else {
                        current_state[index_y * cells_x + index_x] = Cell::Dead;
                    }
                }
                // Rozwiązanie dla martwego drzewa
                Cell::Dead => {
                    let mut trees_alive = 0;

                    // Sprawdzenie z iloma żywymi drzewami sąsiaduje ta komórka
                    match previous_state[(index_y - 1) * cells_x + (index_x - 1)] {
                        Cell::Alive { .. } => trees_alive += 1,
                        _ => {}
                    }

                    match previous_state[(index_y - 1) * cells_x + index_x] {
                        Cell::Alive { .. } => trees_alive += 1,
                        _ => {}
                    }

                    match previous_state[(index_y - 1) * cells_x + (index_x + 1)] {
                        Cell::Alive { .. } => trees_alive += 1,
                        _ => {}
                    }

                    match previous_state[index_y * cells_x + (index_x - 1)] {
                        Cell::Alive { .. } => trees_alive += 1,
                        _ => {}
                    }

                    match previous_state[index_y * cells_x + (index_x + 1)] {
                        Cell::Alive { .. } => trees_alive += 1,
                        _ => {}
                    }

                    match previous_state[(index_y + 1) * cells_x + (index_x - 1)] {
                        Cell::Alive { .. } => trees_alive += 1,
                        _ => {}
                    }

                    match previous_state[(index_y + 1) * cells_x + index_x] {
                        Cell::Alive { .. } => trees_alive += 1,
                        _ => {}
                    }

                    match previous_state[(index_y + 1) * cells_x + (index_x + 1)] {
                        Cell::Alive { .. } => trees_alive += 1,
                        _ => {}
                    }

                    // Sprawdzenie czy drzewo wykiełkuje (kontakt z żyjącymi drzewami)
                    if flat_distr.sample(rng)
                        < 1.0 - (1.0 - config.sprout_probability).powi(trees_alive)
                    {
                        current_state[index_y * cells_x + index_x] = Cell::Alive {
                            size: BASE_TREE_SIZE,
                            color: config.forest_color_palette.choose(rng).unwrap().clone(),
                        }
                    // Sprawdzenie czy drzewo wykiełkuje (losowo)
                    } else if flat_distr.sample(rng) < config.random_sprout_probability {
                        current_state[index_y * cells_x + index_x] = Cell::Alive {
                            size: BASE_TREE_SIZE,
                            color: config.forest_color_palette.choose(rng).unwrap().clone(),
                        };
                    // Jeśli nic się nie stanie to drzewo wciąż jest martwe (nie ma drzewa)
                    } else {
                        current_state[index_y * cells_x + index_x] = Cell::Dead;
                    }
                }
            }
        }
    }
}

/// Zamiana plansz.
#[inline]
pub fn swap_states(previous_state: &mut Vec<Cell>, current_state: &mut Vec<Cell>) {
    std::mem::swap(previous_state, current_state);
}

/// Główna procedura symulacji.
#[inline]
pub fn simulate(config: &Configuration) -> Result<(), String> {
    let mut rng = thread_rng();
    let mut previous_state = initialize_grid(&mut rng, config);
    let mut current_state = initialize_grid_default(config);

    // Bufor na klatkę obrazu (wielokrotnego użycia, mechanizm oszczędzania na
    // dealokacji pamięci)
    let mut frame_buffer: Vec<u8> =
        vec![0; (config.resolution.0 * config.resolution.1 * 3) as usize];

    let gif_file = match File::create(Path::new(&config.output_path)) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!(
                "{} ({})\n\nSzczegóły:\n    {}\n",
                style("Błąd podczas tworzenia pliku wynikowego!")
                    .red()
                    .bold(),
                style(format!("\"{}\"", config.output_path))
                    .cyan()
                    .italic()
                    .bold(),
                error
            ));
        }
    };

    // Utworzenie writera dla formatu GIF
    let gif_writer = BufWriter::new(gif_file);
    let mut gif_encoder = GifEncoder::new_with_speed(gif_writer, config.frame_rate as i32);

    // Zapętlenie gifa
    gif_encoder.set_repeat(Repeat::Infinite).unwrap();

    print!("\n\n");
    let term = Term::stdout();
    term.hide_cursor().unwrap();

    // Główna pętla symulacji
    for frame_number in 0..config.frames {
        term.move_cursor_left(1000).unwrap();
        print!(
            "{} {}/{}",
            style("Postęp symulacji:").green().bold(),
            frame_number + 1,
            config.frames
        );

        // Generowanie nowego stanu
        generate_current_state(&mut rng, config, &previous_state, &mut current_state);

        // Rysowanie nowego stanu
        draw(config, &current_state, &mut frame_buffer);

        // Zapis nowego stanu
        gif_encoder
            .encode(
                &frame_buffer,
                config.resolution.0,
                config.resolution.1,
                ColorType::Rgb8,
            )
            .unwrap();

        // Zamiana plansz (mechanizm oszczędzania na dealokacji pamięci)
        swap_states(&mut previous_state, &mut current_state);
    }

    // Zakończenie symulacji
    println!("\n{}", style("Ukończono!").green().bold());
    term.show_cursor().unwrap();

    Ok(())
}
