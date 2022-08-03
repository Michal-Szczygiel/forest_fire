use console::style;
use ron::de::from_reader;
use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Struktura reprezentująca konfigurację symulacji.
#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    /// Liczba klatek symulacji.
    pub frames: u32,

    /// Liczba klatek na sekundę animacji.
    pub frame_rate: u32,

    /// Ścieżka do pliku wynikowego. Formatem wynikowym jest GIF więc
    /// nazwa pliku powinna mieć rozszerzenie .gif.
    pub output_path: String,

    /// Rozdzielczość generowanej animacji: (pozioma, pionowa).
    pub resolution: (u32, u32),

    /// Wielkość pojedynczej komórki w pikselach. Ten parametr musi być tak
    /// dobrany aby dzielił wartość rozdzielczości poziomej i pionowej bez reszty.
    pub cell_size: u32,

    /// Frakcja komórek żywych na początku symulacji. Parametr opcjonalny.
    #[serde(default = "Configuration::default_alive_fraction")]
    pub alive_fraction: f32,

    /// Prawdopodobieństwo wykiełkowania (przy kontakcie z innymi drzewami). Parametr opcjonalny.
    #[serde(default = "Configuration::default_sprout_probability")]
    pub sprout_probability: f32,

    /// Prawdopodobieństwo losowego wykiełkowania. Parametr opcjonalny.
    #[serde(default = "Configuration::default_random_sprout_probability")]
    pub random_sprout_probability: f32,

    /// Szybkość wzrostu drzew. Parametr opcjonalny.
    #[serde(default = "Configuration::default_growth_rate")]
    pub growth_rate: f32,

    /// Łatwopalność drzew, określa jak łatwo drzewa zajmują się ogniem od
    /// innych drzew. Parametr opcjonalny.
    #[serde(default = "Configuration::default_inflammability")]
    pub inflammability: f32,

    /// Prawdopodobieństwo samozapłonu. Parametr opcjonalny.
    #[serde(default = "Configuration::default_self_ignition_probability")]
    pub self_ignition_probability: f32,

    /// Szybkość spalania. Parametr opcjonalny.
    #[serde(default = "Configuration::default_burning_rate")]
    pub burning_rate: f32,

    /// Paleta kolorystyczna lasu. Parametr opcjonalny.
    #[serde(default = "Configuration::default_forest_color_palette")]
    pub forest_color_palette: Vec<(u8, u8, u8)>,

    /// Kolor płomienia. Paramter opcjonalny.
    #[serde(default = "Configuration::default_fire_color")]
    pub fire_color: (u8, u8, u8),

    /// Kolor podłoża. Parametr opcjonalny.
    #[serde(default = "Configuration::default_ground_color")]
    pub ground_color: (u8, u8, u8),
}

impl Configuration {
    /// Domyślna wartość dla paramteru: alive_fraction
    const fn default_alive_fraction() -> f32 {
        0.5
    }

    /// Domyślna wartość dla paramteru: sprout_probability
    const fn default_sprout_probability() -> f32 {
        0.005
    }

    /// Domyślna wartość dla paramteru: random_sprout_probability
    const fn default_random_sprout_probability() -> f32 {
        0.00075
    }

    /// Domyślna wartość dla paramteru: growth_rate
    const fn default_growth_rate() -> f32 {
        0.001
    }

    /// Domyślna wartość dla paramteru: inflammability
    const fn default_inflammability() -> f32 {
        0.075
    }

    /// Domyślna wartość dla paramteru: self_ignition_probability
    const fn default_self_ignition_probability() -> f32 {
        0.000005
    }

    /// Domyślna wartość dla paramteru: burning_rate
    const fn default_burning_rate() -> f32 {
        0.075
    }

    /// Domyślna wartość dla paramteru: forest_color_palette
    fn default_forest_color_palette() -> Vec<(u8, u8, u8)> {
        vec![
            (88, 227, 21),
            (104, 221, 4),
            (48, 175, 32),
            (185, 242, 10),
            (20, 180, 78),
            (3, 71, 84),
        ]
    }

    /// Domyślna wartość dla paramteru: fire_color
    const fn default_fire_color() -> (u8, u8, u8) {
        (255, 28, 28)
    }

    /// Domyślna wartość dla paramteru: ground_color
    const fn default_ground_color() -> (u8, u8, u8) {
        (2, 12, 5)
    }
}

/// Funkcja wyciągająca ścieżkę do pliku z konfiguracją z parametrów wywołania programu.
#[inline]
pub fn get_configuration_file() -> Result<String, String> {
    let arguments: Vec<String> = std::env::args().collect();

    if arguments.len() < 2 {
        Err(format!(
            "{}\n\nSzczegóły:\n    Nie podano ścieżki do pliku konfiguracyjnego .ron.\n",
            style("Błąd parametrów!").red().bold()
        ))
    } else if arguments.len() > 2 {
        Err(format!(
            "{}\n\nSzczegóły:\n    Podano zbyt wiele parametrów.\n",
            style("Błąd parametrów!").red().bold()
        ))
    } else {
        Ok(arguments[1].clone())
    }
}

/// Funkcja ładująca konfigurację z pliku .ron.
#[inline]
pub fn load_configuration(path: &str) -> Result<Configuration, String> {
    let config_file = match File::open(Path::new(path)) {
        Ok(config_file) => config_file,
        Err(error) => {
            return Err(format!(
                "{} ({})\n\nSzczegóły:\n    {}\n",
                style("Błąd podczas otwierania pliku konfiguracyjnego!")
                    .red()
                    .bold(),
                style(format!("\"{}\"", path)).cyan().italic().bold(),
                error
            ));
        }
    };
    let config_reader = BufReader::new(config_file);
    let config: Configuration = match from_reader(config_reader) {
        Ok(config) => config,
        Err(error) => {
            return Err(format!(
                "{} ({})\n\nSzczegóły:\n    {} (serde error: https://serde.rs/)\n",
                style("Błąd podczas parsowania pliku konfiguracyjnego!")
                    .red()
                    .bold(),
                style(format!("\"{}\"", path)).cyan().italic().bold(),
                error
            ));
        }
    };

    Ok(config)
}

/// Funkcja walidująca poprawność parametrów w konfiguracji.
#[inline]
pub fn validate_configuration(config: &Configuration) -> Result<(), String> {
    // Sprawdzenie dla parametru: frames
    if config.frames < 1 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości większe od 0, podano: {}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"frame_rate\"").yellow().bold(),
            config.frames
        ));
    }

    // Sprawdzenie dla parametru: frame_rate
    if config.frame_rate < 1 || config.frame_rate > 30 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości z zakresu 1..30, podano: {}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"frame_rate\"").yellow().bold(),
            config.frame_rate
        ));
    }

    // Sprawdzenie dla parametru: resolution
    if config.resolution.0 < 256
        || config.resolution.0 > 4096
        || config.resolution.1 < 256
        || config.resolution.1 > 2160
    {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości z zakresu \
            (256..4096, 256..2160), podano: {:?}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"resolution\"").yellow().bold(),
            config.resolution
        ));
    }

    // Sprawdzenie dla paramteru: cell_size
    if config.resolution.0 % config.cell_size != 0 || config.resolution.1 % config.cell_size != 0 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować taką wartość aby dzielć \
            rozdzielczość poziomą i pionową bez reszty,\n    podano: ({} % {} = {}, {} % {} = {})\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"cell_size\"").yellow().bold(),
            config.resolution.0,
            config.cell_size,
            config.resolution.0 % config.cell_size,
            config.resolution.1,
            config.cell_size,
            config.resolution.1 % config.cell_size
        ));
    }

    // Sprawdzenie dla parametru: alive_fraction
    if config.alive_fraction < 0.0 || config.alive_fraction > 1.0 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości z zakresu \
            0.0..1.0, podano: {:?}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"alive_fraction\"").yellow().bold(),
            config.alive_fraction
        ));
    }

    // Sprawdzenie dla parametru: sprout_probability
    if config.sprout_probability < 0.0 || config.sprout_probability > 1.0 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości z zakresu \
            0.0..1.0, podano: {:?}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"sprout_probability\"").yellow().bold(),
            config.sprout_probability
        ));
    }

    // Sprawdzenie dla parametru: random_sprout_probability
    if config.random_sprout_probability < 0.0 || config.random_sprout_probability > 1.0 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości z zakresu \
            0.0..1.0, podano: {:?}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"sprout_probability\"").yellow().bold(),
            config.random_sprout_probability
        ));
    }

    // Sprawdzenie dla parametru: growth_rate
    if config.growth_rate < 0.0 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości większe od \
            0.0, podano: {:?}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"growth_rate\"").yellow().bold(),
            config.growth_rate
        ));
    }

    // Sprawdzenie dla paramteru: inflammability
    if config.inflammability < 0.0 || config.inflammability > 1.0 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości z zakresu \
            0.0..1.0, podano: {:?}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"inflammability\"").yellow().bold(),
            config.inflammability
        ));
    }

    // Sprawdzenie dla paramteru: self_ignition_probability
    if config.self_ignition_probability < 0.0 || config.self_ignition_probability > 1.0 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości z zakresu \
            0.0..1.0, podano: {:?}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"self_ignition_probability\"").yellow().bold(),
            config.self_ignition_probability
        ));
    }

    // Sprawdzenie dla parametru: burning_rate
    if config.burning_rate <= 0.0 {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Parametr {} musi przyjmować wartości większe od \
            0.0, podano: {:?}\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"burning_rate\"").yellow().bold(),
            config.burning_rate
        ));
    }

    // Sprawdzenie dla parametru: forest_color_palette
    if config.forest_color_palette.is_empty() {
        return Err(format!(
            "{}\n\nSzczegóły:\n    Lista {} nie może być pusta\n",
            style("Błąd konfiguracji!").red().bold(),
            style("\"forest_color_palette\"").yellow().bold()
        ));
    }

    Ok(())
}

/// Funkcja wypisująca specyfikację konfiguracji.
#[inline]
pub fn print_configuration_specification() {
    println!(
        "{}",
        format!(
            "\n\n{}\n\n{}\n\
                ____________________________________________________________________________________________________\
                \n1  |{}(\
                \n2  |    {} <u32>,                                  <- Liczba klatek symulacji\
                \n3  |    {} <u32>,                              <- Liczba klatek na sekundę animacji\
                \n4  |    {} <String>,                          <- Ścieżka do pliku wynikowego\
                \n5  |    {} <(u32, u32)>,                       <- Rozdzielczość\
                \n6  |    {} <u32>,                               <- Wielkość komórki\
                \n7  |    {} <f32>,                          <- Frakcja komórek zajętych przez drzewa\
                \n8  |    {} <f32>,                      <- Prawdopodobieństwo wykiełkowania\
                \n9  |    {} <f32>,               <- Prawdopodobieństwo losowego wykiełkowania\
                \n10 |    {} <f32>,                             <- Tempo wzrostu\
                \n11 |    {} <f32>,                          <- Łatwopalność\
                \n12 |    {} <f32>,               <- Prawdopodobieństwo samozapłonu\
                \n13 |    {} <f32>,                            <- Szybkość spalania\
                \n14 |    {} <[(u8, u8, u8), ...]>,    <- Paleta kolorystyczna lasu\
                \n15 |    {} <(u8, u8, u8)>,                     <- Kolor ognia\
                \n16 |    {} <(u8, u8, u8)>,                   <- Kolor podłoża\
                \n17 |)\n",
            style("Specyfikacja pliku konfiguracyjnego:").blue().bold(),
            style("example.ron").bold(),
            style("Configuration").cyan().bold(),
            style("frames:").green(),
            style("frame_rate:").green(),
            style("output_path:").green(),
            style("resolution:").green(),
            style("cell_size:").green(),
            style("alive_fraction:").yellow(),
            style("sprout_probability:").yellow(),
            style("random_sprout_probability:").yellow(),
            style("growth_rate:").yellow(),
            style("inflammability:").yellow(),
            style("self_ignition_probability:").yellow(),
            style("burning_rate:").yellow(),
            style("forest_color_palette:").yellow(),
            style("fire_color:").yellow(),
            style("ground_color:").yellow()
        )
    );
}

/// Funkcja wypisująca aktualnie zadaną konfigurację.
#[inline]
pub fn print_configuration(config: &Configuration, configuration_file: &str) {
    println!(
        "{}",
        format!(
            "{} ({}):\n\
                \n{}(\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n    {} {},\
                \n)",
            style("Parametry symulacji").blue().bold(),
            style(format!("\"{}\"", configuration_file))
                .cyan()
                .italic()
                .bold(),
            style("Configuration").cyan().bold(),
            style("frames:").green(),
            style(format!("{}", config.frames)).bold(),
            style("frame_rate:").green(),
            style(format!("{}", config.frame_rate)).bold(),
            style("output_path:").green(),
            style(format!("\"{}\"", config.output_path)).bold(),
            style("resolution:").green(),
            style(format!("{:?}", config.resolution)).bold(),
            style("cell_size:").green(),
            style(format!("{}", config.cell_size)).bold(),
            style("alive_fraction:").yellow(),
            style(format!("{}", config.alive_fraction)).bold(),
            style("sprout_probability:").yellow(),
            style(format!("{}", config.sprout_probability)).bold(),
            style("random_sprout_probability:").yellow(),
            style(format!("{}", config.random_sprout_probability)).bold(),
            style("growth_rate:").yellow(),
            style(format!("{}", config.growth_rate)).bold(),
            style("inflammability:").yellow(),
            style(format!("{}", config.inflammability)).bold(),
            style("self_ignition_probability:").yellow(),
            style(format!("{}", config.self_ignition_probability)).bold(),
            style("burning_rate:").yellow(),
            style(format!("{}", config.burning_rate)).bold(),
            style("forest_color_palette:").yellow(),
            style(format!("{:?}", config.forest_color_palette)).bold(),
            style("fire_color:").yellow(),
            style(format!("{:?}", config.fire_color)).bold(),
            style("ground_color:").yellow(),
            style(format!("{:?}", config.ground_color)).bold()
        )
    );
}
