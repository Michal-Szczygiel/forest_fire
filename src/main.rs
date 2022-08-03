mod configuration;
mod simulation;
mod visualization;

fn main() {
    // Pobranie nazwy ścieżki pliku konfiguracyjnego z prametrów wywołania
    let config_path = match configuration::get_configuration_file() {
        Ok(path) => path,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    // Załadowanie konfiguracji z pliku
    let config = match configuration::load_configuration(&config_path) {
        Ok(config) => config,
        Err(error) => {
            println!("{}", error);
            configuration::print_configuration_specification();
            return;
        }
    };

    // Walidacja konfiguracji
    match configuration::validate_configuration(&config) {
        Err(error) => {
            println!("{}", error);
            return;
        }
        _ => {}
    }

    // Wypisanie wartości parametrów
    configuration::print_configuration(&config, &config_path);

    // Przeprowadzenie symulacji
    match simulation::simulate(&config) {
        Err(error) => {
            println!("{}", error);
        }
        _ => {}
    }
}
