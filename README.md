# **Forest-fire model:**

The '**bin**' directory contains precompiled versions of the program for Windows and Linux (x86-64 architecture). To run the program, you need a configuration file in the [.ron](https://github.com/ron-rs/ron) format. Sample files are located in the '**config**' directory. The full program invocation from the console on Windows is: ./forest_fire.exe config.ron. The result of the program is an animation in the GIF format.

### Sample configuration file:
#### '**green_tea.ron**'

``` ron
Configuration(
    // Number of simulation frames
    frames: 500,

    // The number of frames per second of the animation
    frame_rate: 25,

    // Path to the output file. The resulting format is GIF,
    // so the file name should have the extension .gif
    output_path: "green_tea.gif",

    // Resolution of generated animation: (horizontal, vertical)
    resolution: (960, 540),

    // The size of a single cell in pixels. This parameter must be selected in such a way 
    // that it divides the horizontal and vertical resolution values without any remainder
    cell_size: 4,

    // Fraction of alive cells at the beginning of the simulation. Optional parameter
    alive_fraction: 0.5,

    // Probability of germination (when in contact with other trees). Optional parameter
    sprout_probability: 0.005,

    // Probability of random germination. Optional parameter
    random_sprout_probability: 0.00075,

    // Tree growth speed. Optional parameter
    growth_rate: 0.001,

    // The flammability of trees determines how easily trees catch fire from other trees. 
    // Optional parameter
    inflammability: 0.075,

    // Likelihood of spontaneous combustion. Optional parameter
    self_ignition_probability: 0.000005,

    // Burning rate. Optional parameter
    burning_rate: 0.075,

    // The color palette of the forest. Optional parameter
    forest_color_palette: [(88, 227, 21), (104, 221, 4), (48, 175, 32), (185, 242, 10), (20, 180, 78), (3, 71, 84)],
    
    // The color of the flame. Optional parameter
    fire_color: (255, 28, 28),

    // Ground color. Optional parameter
    ground_color: (2, 12, 5),
)
```

# Examples:

https://user-images.githubusercontent.com/79999342/182649367-3f7a90ea-6362-413d-b9d4-e10f16133fcf.mp4

https://user-images.githubusercontent.com/79999342/182650979-0f8a79d5-f731-4b51-b4ff-4c99e9a8f47c.mp4
