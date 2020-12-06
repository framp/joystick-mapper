# joystick-mapper

A rust library to map joystick input to keyboard keys, mouse presses and more.

And a couple of showcases implementations.

## How to use

1. Create a configuration file for the mapping you want to setup
2. The file needs to follow the YAML format and contain two maps `buttons` and `axis`
3. Set up input buttons using [these identifiers](https://gilrs-project.gitlab.io/gilrs/doc/gilrs/ev/enum.Button.html#variants)
4. Set up input axis using [these identifiers](https://gilrs-project.gitlab.io/gilrs/doc/gilrs/ev/enum.Axis.html#variants) and passing an array of two keys
5. Set up output keyboard keys using [these identifiers](https://docs.rs/enigo/0.0.14/enigo/enum.Key.html) or a letter
6. Set up output mouse buttons using a map MouseButton with one of [these identifiers](https://docs.rs/enigo/0.0.14/enigo/enum.MouseButton.html)
7. Set up output mouse axis using `MouseX` and `MouseY`
8. Run it `cargo run --bin joystick-mapper path/to/configuration.conf` or `joystick-mapper path/to/configuration.conf`
9. Enjoy!

## Among Us Edition

`joystick-mapper-among-us` is `joystick-mapper` plus shortcuts for venting in the game [Among Us](https://store.steampowered.com/app/945360/Among_Us/).

When a `VentAction` is dispatched, a screenshot is taken and a sophisticated algorithm will map clicking on the vents you see on screen to buttons on your controller.

A few cases in Mira HQ may look a bit ambiguos because there are 3 arrows and they're not straight.

The algorithm will map vents always in the same way though, so you just have to learn the directions.

## Example configuration:

Checkout the [examples](https://github.com/framp/joystick-mapper/tree/master/examples) directory for sample configurations

## License:

MIT
