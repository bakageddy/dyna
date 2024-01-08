# Dyna
A Local Search Engine written in Rust <3

# Usage

```console
$ cargo b
$ ./target/{release,debug}/dyna --index test --save file_to_save
$ ./target/{release,debug}/dyna --save file_to_save --search "Search Term"
```

# TODO

1. Implement tf-idf algorithm
2. Search Text over pdf(s), epub, docx
3. Make a gui for searching docs in browser
4. Implement image object search
5. Lemmatization of input

# Thanks to

[Tsoding](https://github.com/tsoding/seroost) for inspiring this project

