# joystick-mapper

A simple program to map joystick input to keyboard keys.

## How to use

Run it with:

```bash
cargo run examples/among-us.conf
# or
joystick-mapper examples/among-us.conf
```

1. Create a configuration file for the mapping you want to setup
2. The file needs to follow the YAML format and contain two maps `buttons` and `axis`
3. Set up input buttons using [these identifiers](https://gilrs-project.gitlab.io/gilrs/doc/gilrs/ev/enum.Button.html#variants)
4. Set up input axis using [these identifiers](https://gilrs-project.gitlab.io/gilrs/doc/gilrs/ev/enum.Axis.html#variants) and passing an array of two keys
5. Set up output keyboard keys using [these identifiers](https://docs.rs/enigo/0.0.14/enigo/enum.Key.html) or a letter
6. Set up output mouse buttons using a map MouseButton with one of [these identifiers](https://docs.rs/enigo/0.0.14/enigo/enum.MouseButton.html)
7. Set up output mouse axis using `MouseX` and `MouseY`

## Example:

```yaml
buttons:
  East: e
  South: q
  West: r
  North: Tab
  DPadUp: w
  DPadDown: s
  DPadRight: d
  DPadLeft: a
  LeftTrigger2: Escape
  RightTrigger2:
    MouseButton: Left

axis:
  LeftStickX: [a, d]
  LeftStickY: [s, w]
  RightStickX: [MouseX, MouseX]
  RightStickY: [MouseY, MouseY]
```

## Dependencies
Linux: libudev, libxdo

## License:

MIT
