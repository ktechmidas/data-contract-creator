# Data Contract Creator

This is a simple web app written in Rust using the [Yew](https://yew.rs/) framework for generating data contract schemas for Dash Platform.

## Features

- Dynamically generate and modify data contract JSON schemas using a web interface
- Add/remove object properties, array items, and object indices
- Set data types, validation constraints, and descriptions for properties and indices
- View the generated data contract JSON schema in real-time

## Setup

If you've never used Yew:

1. Install WebAssembly target: `rustup target add wasm32-unknown-unknown`
2. Install Trunk: `cargo install --locked trunk`

Else:

1. Clone the repository: `git clone https://github.com/pauldelucia/data-contract-creator.git`
2. Change into the project directory: `cd data-contract-creator`
3. Start the app `trunk serve --open`

## Usage

### Adding a Document Type

1. Click the "Add document type" button
2. Enter a name for the document type
3. Add properties, indices, and other options as desired

### Adding a Property

1. Click the "Add property" button in the "Properties" section of a document type (new document types automatically have one)
2. Enter a name for the property
3. Select a data type from the dropdown menu
4. Add validation constraints, descriptions, and other options as desired

### Adding an Index

1. Click the "Add Index" button in the "Indices" section of a document type
2. Enter a name for the index
3. Select the index type (unique or non-unique)
4. Enter the properties to include in the index and their sort order

### Generating a Data Contract

1. Click the "Submit" button
2. View the generated schema in the "Contract" section

## Contributing

Contributions are welcome! Please submit a pull request or open an issue if you encounter any problems or have suggestions for improvement.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
