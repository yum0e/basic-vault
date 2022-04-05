# basic-vault

Simple implementation in Rust of this [basic vault](https://www.youtube.com/watch?v=Ztr2Jet2-YY).

This should NOT be used for production, feel free to use it for learning purposes though.

## explanations

This basic vault contract allows anyone to create a Vault and put some egld in it.
A list of users is associated to this vault and will receive an equal amount of all the egld stored in this vault when the `distribute` function is called.

In the v1 you can add egld to your created vault whenever you want.
