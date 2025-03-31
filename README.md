# gba-emu

![Rust Mascot](https://www.rust-lang.org/logos/rust-logo-512x512.png)

A Game Boy Advance (GBA) emulator written in Rust.

## Description

This project is an emulator for the Game Boy Advance (GBA) console. It is built using the Rust programming language and aims to provide an efficient and accurate environment for running GBA games. However, to run the emulator, a frontend is required.

## Features

- Emulates the GBA hardware with accuracy
- Written in Rust for performance and safety
- Extensible architecture allowing for further development and features
- Support for various GBA ROMs and tests

## Frontend Options

To run the emulator, you'll need a frontend. There are two options:

1. **[minifb-frontend](https://github.com/gba-rs/minifb-frontend)**  
   A simple, minimalistic frontend using the `minifb` crate for displaying the emulation window.

2. **[web-frontend](https://github.com/gba-rs/web-frontend)**  
   A web-based frontend that allows you to run the emulator directly in your browser.

## Installation

To get started, you need to have Rust installed on your system. You can download and install Rust from [the official site](https://www.rust-lang.org/).

1. Clone the repository:

    ```bash
    git clone https://github.com/gba-rs/gba-emu.git
    ```

2. Navigate into the project directory:

    ```bash
    cd gba-emu
    ```

3. Build the project:

    ```bash
    cargo build
    ```

4. Clone and set up the frontend you want to use:

    - For the **minifb-frontend**:

        ```bash
        git clone https://github.com/gba-rs/minifb-frontend.git
        cd minifb-frontend
        cargo build
        ```

    - For the **web-frontend**:

        ```bash
        git clone https://github.com/gba-rs/web-frontend.git
        cd web-frontend
        cargo build
        ```

5. Run the emulator with the appropriate frontend:

    - For **minifb-frontend** (replace `your_rom.gba` with your ROM):

        ```bash
        cargo run --path ../gba-emu -- your_rom.gba
        ```

    - For **web-frontend** (replace `your_rom.gba` with your ROM):

        ```bash
        cargo run --path ../gba-emu -- your_rom.gba
        ```

## Dependencies

- Rust (latest stable version)
- A GBA ROM file to test the emulator
- A frontend (either `minifb-frontend` or `web-frontend`)

## Contributing

Contributions are welcome! If you have any improvements or bug fixes, feel free to open a pull request.

## License

This project is licensed under either of the following licenses:

- Apache License, Version 2.0, [LICENSE_APACHE](LICENSE_APACHE)
- MIT License, [LICENSE_MIT](LICENSE_MIT)

You may choose one of them.

## Acknowledgements

- The emulator is built with a focus on GBA hardware emulation accuracy.
- Thanks to all the contributors to the project and the Rust community.

## Contact

For any issues or questions, please open an issue on the GitHub repository or contact the maintainers.
