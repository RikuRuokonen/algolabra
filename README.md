## Readme

### Usage

- Download source
- base command is  ```cargo run``` that is run in application root, params are below
- use ```--generate-keys``` to generate public/private key-pair. Keys are stored to application's root folder.
- passing in ```--file-name``` and ```--output```, will encrypt file's contents to output file-named target file. Input file has to exist, output file will be created.
- passing in ```--encrypted-file``` and ```--decrypted-output```, will decrypt encrypted files contents to decrypted output file, which is also created during run-time.

All files are expected to be located in application root.

#### Example
You have file called test.txt in application root folder, that contains plaintext.

You run  ```cargo run -- --generate-keys```. Now you have key-pair created to application root.

Next you run  ```cargo run -- --file-name test.txt --output encrypted.txt``` --> test.txt content is encrypted and placed into new file encrypted.txt

Finally, you run ```cargo run -- --encrypted-file encrypted.txt --decrypted_output decrypted_message.txt``` --> you have decrypted plaintext file which should match original test.txt

CLI is still in works, so some bugs are expected.