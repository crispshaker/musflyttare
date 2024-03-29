# musflyttare
This Rust program is crafted to automate mouse movements, mimicking user activity every 30 seconds if no user activity is detected. Its purpose is to help maintain an active online presence on platforms like Microsoft Teams, Webex and similar applications.

## Prerequisites

Before you proceed, ensure that you have the following prerequisites installed on your system:

- Rust and Cargo: You can download and install Rust and Cargo by following the official installation guide at [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Building

1. **Clone the Repository**:

    If you haven't already, clone the repository containing the program to your local machine.

    ```bash
    git clone https://github.com/crispshaker/musflyttare.git
    cd musflyttare
    ```

2. **Build the Program**:

    ```bash
    cargo build --release
    ```

## Creating an Executable

Once you have successfully built the program, you can create an executable for *musflyttare* by following these steps:

1. **Locate the Executable**:

    After building the program, the compiled executable file will be located in the `target/release` directory within your project folder. The file name will be the same as your project name. In this case, it will be named *musflyttare*.

2. **Copy the Executable**:

    Copy the executable to a location of your choice or distribute it as needed. You can also rename the executable to make it more meaningful for your use case.

    ```bash
    cp target/release/musflyttare /path/to/destination/
    ```

3. **Make the Executable Executable**:

    If the copied file is not already marked as executable, you can do so using the `chmod` command.

    ```bash
    chmod +x /path/to/destination/
    ```

## Security Settings

This application requires access to your computer's accessibility features, which may necessitate granting appropriate permissions. It functions autonomously without the need for an internet connection and refrains from any attempts to establish such connections.

## Disclaimer

USE THIS PROGRAM AT YOUR OWN RISK. The authors and contributors of this program make no guarantees or warranties, and assume no liability for any consequences resulting from the use of this software. Be cautious when using this program, especially in production environments, and ensure that you have appropriate backups and safeguards in place.

## License

This program is open-source software and is distributed under the terms of the MIT License. You can find the license details in the `LICENSE` file in the project repository.