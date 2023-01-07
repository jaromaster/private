# PRIVATE - tool to keep your data private


## About
Encrypts and decrypts files and directories using the fernet algorithm. <br>
Additionally, hashing via SHA3 (256) is supported. <br>
All operations are executed inplace.

## Encryption
`private encrypt file.txt images_folder` <br>
`private encrypt image.jpg image.png`

## Decryption
`private decrypt file.txt images_folder`<br>
`private decrypt image.jpg image.png`

## Hashing
`private hash file.txt.org file.txt` <br>
`private hash file.txt my_folder/sub_folder` 
